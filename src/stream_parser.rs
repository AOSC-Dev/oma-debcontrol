use crate::{parse_finish, parse_streaming, Error, Paragraph, Streaming};
use alloc::vec::Vec;
use core::str::{from_utf8, Utf8Error};

pub trait Read {
    type Error;

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error>;
}

#[cfg(feature = "std")]
impl<R: std::io::Read> Read for R {
    type Error = std::io::Error;

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.read(buf)
    }
}

#[derive(Debug)]
pub enum ParserError<'a> {
    InvalidUtf8(Utf8Error),
    InvalidSyntax(Error<'a>),
}

impl<'a> From<Utf8Error> for ParserError<'a> {
    fn from(err: Utf8Error) -> Self {
        ParserError::InvalidUtf8(err)
    }
}

impl<'a> From<Error<'a>> for ParserError<'a> {
    fn from(err: Error<'a>) -> Self {
        ParserError::InvalidSyntax(err)
    }
}

pub struct Parser<R> {
    chunk_size: usize,
    buffer: Vec<u8>,
    pos: usize,
    read: R,
    exhausted: bool,
}

impl<R: Read> Parser<R> {
    pub fn new(read: R, chunk_size: usize) -> Self {
        Parser {
            chunk_size,
            buffer: Vec::with_capacity(chunk_size),
            pos: 0,
            read,
            exhausted: false,
        }
    }

    pub fn advance(&mut self) -> Result<(), R::Error> {
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

    pub fn parse(&mut self) -> Result<Option<Streaming<Paragraph>>, ParserError> {
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
    use alloc::{string::ToString, vec};
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

    impl<'a> Read for Bytes<'a> {
        type Error = ();

        fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            let to_read = min(self.bytes.len() - self.pos, buf.len());
            buf[..to_read].copy_from_slice(&self.bytes[self.pos..self.pos + to_read]);
            self.pos += to_read;
            Ok(to_read)
        }
    }

    fn parse_utf8_input(chunk_size: usize) {
        let input = Bytes::new(
            indoc!(
                "
                field: äöüß
                field: value
                 cont
    
                field: ßäöü
                "
            )
            .as_bytes(),
        );
        let mut parser = Parser::new(input, chunk_size);

        let mut fields = vec![];
        while let Some(result) = parser.parse().unwrap() {
            match result {
                Streaming::Item(paragraph) => {
                    fields.extend(
                        paragraph
                            .fields
                            .into_iter()
                            .map(|field| (field.name.to_string(), field.value)),
                    );
                }
                Streaming::Incomplete => parser.advance().unwrap(),
            }
        }

        assert_eq!(
            fields,
            vec![
                ("field".to_string(), "äöüß".to_string()),
                ("field".to_string(), "value\ncont".to_string()),
                ("field".to_string(), "ßäöü".to_string()),
            ]
        );
    }

    #[test]
    fn should_parse_file_with_chunk_size_1() {
        parse_utf8_input(1);
    }

    #[test]
    fn should_parse_file_with_chunk_size_2() {
        parse_utf8_input(2);
    }

    #[test]
    fn should_parse_file_with_chunk_size_10() {
        parse_utf8_input(10);
    }

    #[test]
    fn should_parse_file_with_chunk_size_1000() {
        parse_utf8_input(1000);
    }
}
