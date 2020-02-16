use crate::{parse_finish, parse_streaming, Paragraph, Streaming, SyntaxError};
use alloc::vec::Vec;
use core::{
    fmt,
    str::{from_utf8, Utf8Error},
};

/// A helper trait for stream input.
///
/// This trait is modeled on std's `Read`, but is separate so it's usable with `no_std`. When the
/// `std` feature is enabled (it is by default), this trait has a blanket implementation for every
/// type that implement std's `Read`.
pub trait BufParseInput {
    /// The error type returned by read operations.
    type Error;

    /// Read bytes into the provided buffer, up to its length, and return the number of bytes read.
    ///
    /// This function may read fewer bytes. If no more input is available, it should not modify the
    /// buffer and merely return 0.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error>;
}

#[cfg(feature = "std")]
impl<R: std::io::Read + ?Sized> BufParseInput for R {
    type Error = std::io::Error;

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.read(buf)
    }
}

/// An error type returned by [`BufParse`](struct.BufParse.html).
#[derive(Debug)]
pub enum BufParseError<'a> {
    /// The input stream was not valid UTF-8.
    InvalidUtf8(Utf8Error),
    /// There was a syntax error in the input stream.
    InvalidSyntax(SyntaxError<'a>),
}

impl fmt::Display for BufParseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BufParseError::InvalidUtf8(err) => write!(f, "invalid utf-8 in input: {}", err),
            BufParseError::InvalidSyntax(err) => write!(f, "invalid syntax: {}", err),
        }
    }
}

impl<'a> From<Utf8Error> for BufParseError<'a> {
    fn from(err: Utf8Error) -> Self {
        BufParseError::InvalidUtf8(err)
    }
}

impl<'a> From<SyntaxError<'a>> for BufParseError<'a> {
    fn from(err: SyntaxError<'a>) -> Self {
        BufParseError::InvalidSyntax(err)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for BufParseError<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BufParseError::InvalidUtf8(err) => Some(err),
            BufParseError::InvalidSyntax(_) => None,
        }
    }
}

/// A streaming control file parser that buffers input internally.
///
/// This type handles incrementally reading and buffering input from a source implementing the
/// [`BufParseInput`](trait.BufParseInput.html) trait.
///
/// # Example
/// ```
/// # #[cfg(feature = "std")] {
/// use debcontrol::{BufParse, Streaming};
/// use std::fs::File;
///
/// # let file_name = format!("{}/tests/control", env!("CARGO_MANIFEST_DIR"));
/// let f = File::open(file_name).unwrap();
/// let mut buf_parse = BufParse::new(f, 4096);
/// while let Some(result) = buf_parse.try_next().unwrap() {
///     match result {
///         Streaming::Item(paragraph) => {
///             for field in paragraph.fields {
///                 println!("{}: {}", field.name, &field.value);
///             }
///         }
///         Streaming::Incomplete => buf_parse.buffer().unwrap()
///     }
/// }
/// # }
/// ```
#[derive(Debug)]
pub struct BufParse<R> {
    chunk_size: usize,
    buffer: Vec<u8>,
    pos: usize,
    read: R,
    exhausted: bool,
}

impl<R: BufParseInput> BufParse<R> {
    /// Create a new parser.
    pub fn new(read: R, chunk_size: usize) -> Self {
        BufParse {
            chunk_size,
            buffer: Vec::with_capacity(chunk_size),
            pos: 0,
            read,
            exhausted: false,
        }
    }

    /// Read the next chunk of input into the buffer.
    pub fn buffer(&mut self) -> Result<(), R::Error> {
        self.buffer.drain(..self.pos);
        self.pos = 0;

        let end = self.buffer.len();
        self.buffer.resize(end + self.chunk_size, 0);
        let read = self.read.read(&mut self.buffer[end..])?;
        self.buffer.truncate(end + read);

        if read == 0 {
            self.exhausted = true;
        }

        Ok(())
    }

    /// Try to parse the next paragraph from the input.
    ///
    /// A syntax error encountered during parsing is returned immediately. Otherwise, the nature of
    /// the `Ok` result determines what to do next:
    ///
    /// * If it's `None`, all input has been parsed. Future calls will continue to return `None`.
    /// * If it's [`Streaming::Incomplete`](enum.Streaming.html#variant.Incomplete), there's not
    ///   enough buffered input to make a parsing decision. Call
    ///   [`buffer`](struct.BufParse.html#method.buffer) to read more input.
    /// * If it's [`Streaming::Item`](enum.Streaming.html#variant.Item), a paragraph was parsed.
    ///   Call `try_next` again after processing it.
    pub fn try_next(&mut self) -> Result<Option<Streaming<Paragraph>>, BufParseError> {
        let input = Self::from_utf8(&self.buffer[self.pos..])?;

        match parse_streaming(input)? {
            Streaming::Item((rest, paragraph)) => {
                let parsed = input.len() - rest.len();
                self.pos += parsed;
                Ok(Some(Streaming::Item(paragraph)))
            }
            Streaming::Incomplete => {
                if self.exhausted {
                    let input = from_utf8(&self.buffer[self.pos..])?;
                    let result = parse_finish(input)?;
                    self.pos += input.len();
                    Ok(result.map(Streaming::Item))
                } else {
                    Ok(Some(Streaming::Incomplete))
                }
            }
        }
    }

    fn from_utf8(bytes: &[u8]) -> Result<&str, Utf8Error> {
        from_utf8(bytes).or_else(|err| {
            let longest_valid = err.valid_up_to();
            from_utf8(&bytes[..longest_valid])
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{
        string::{String, ToString},
        vec,
    };
    use assert_matches::assert_matches;
    use core::cmp::min;
    use indoc::indoc;

    struct Bytes<'a> {
        bytes: &'a [u8],
        pos: usize,
    }

    impl<'a> Bytes<'a> {
        pub fn new(bytes: &'a [u8]) -> Self {
            Bytes { bytes, pos: 0 }
        }
    }

    impl<'a> BufParseInput for Bytes<'a> {
        type Error = ();

        fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            let to_read = min(self.bytes.len() - self.pos, buf.len());
            buf[..to_read].copy_from_slice(&self.bytes[self.pos..self.pos + to_read]);
            self.pos += to_read;
            Ok(to_read)
        }
    }

    fn parse_input(input: &[u8], chunk_size: usize) -> Vec<(String, String)> {
        let mut parser = BufParse::new(Bytes::new(input), chunk_size);
        let mut fields = vec![];
        while let Some(result) = parser.try_next().unwrap() {
            match result {
                Streaming::Item(paragraph) => {
                    fields.extend(
                        paragraph
                            .fields
                            .into_iter()
                            .map(|field| (field.name.to_string(), field.value)),
                    );
                }
                Streaming::Incomplete => parser.buffer().unwrap(),
            }
        }
        fields
    }

    #[test]
    fn should_parse_input_in_a_single_chunk() {
        let result = parse_input(
            indoc!(
                "field: value
                another-field: value"
            )
            .as_bytes(),
            1000,
        );
        assert_eq!(
            result,
            vec![
                ("field".to_string(), "value".to_string()),
                ("another-field".to_string(), "value".to_string())
            ]
        );
    }

    #[test]
    fn should_handle_partial_utf8_on_chunk_border() {
        let result = parse_input("12345:äöüöäüääöüäöäüöüöä".as_bytes(), 7);
        assert_eq!(
            result,
            vec![("12345".to_string(), "äöüöäüääöüäöäüöüöä".to_string())]
        );
    }

    #[test]
    fn should_need_to_buffer_at_least_twice_for_nonempty_input() {
        let mut parse = BufParse::new(Bytes::new(b"a: b"), 100);
        parse.buffer().unwrap();
        assert_matches!(parse.try_next(), Ok(Some(Streaming::Incomplete)));
        parse.buffer().unwrap();
        assert_matches!(parse.try_next(), Ok(Some(Streaming::Item(_))));
        assert_matches!(parse.try_next(), Ok(None));
    }

    #[test]
    fn should_keep_returning_none_when_input_is_exhausted() {
        let mut parse = BufParse::new(Bytes::new(b""), 10);
        parse.buffer().unwrap();
        assert_matches!(parse.try_next(), Ok(None));
        assert_matches!(parse.try_next(), Ok(None));
        assert_matches!(parse.try_next(), Ok(None));
    }

    #[test]
    fn should_fail_on_invalid_utf8_inside_chunk() {
        let mut parse = BufParse::new(Bytes::new(b"abc: a\xe2\x82\x28bcd efgh"), 100);
        parse.buffer().unwrap();
        assert_matches!(parse.try_next(), Ok(Some(Streaming::Incomplete)));
        parse.buffer().unwrap();
        assert_matches!(parse.try_next(), Err(BufParseError::InvalidUtf8(_)));
        assert_matches!(parse.try_next(), Err(BufParseError::InvalidUtf8(_)));
    }

    #[test]
    fn should_fail_on_invalid_utf8_on_chunk_border() {
        let mut parse = BufParse::new(Bytes::new(b"abc: ab\xe2\x82\x28bcd efgh"), 7);
        parse.buffer().unwrap();
        assert_matches!(parse.try_next(), Ok(Some(Streaming::Incomplete)));
        parse.buffer().unwrap();
        assert_matches!(parse.try_next(), Ok(Some(Streaming::Incomplete)));
        parse.buffer().unwrap();
        assert_matches!(parse.try_next(), Ok(Some(Streaming::Incomplete)));
        parse.buffer().unwrap();
        assert_matches!(parse.try_next(), Err(BufParseError::InvalidUtf8(_)));
    }

    #[test]
    fn should_fail_on_trailing_invalid_utf8() {
        let mut parse = BufParse::new(Bytes::new(b"abc: a\xe2\x82\x28"), 100);
        parse.buffer().unwrap();
        assert_matches!(parse.try_next(), Ok(Some(Streaming::Incomplete)));
        parse.buffer().unwrap();
        assert_matches!(parse.try_next(), Err(BufParseError::InvalidUtf8(_)));
    }
}
