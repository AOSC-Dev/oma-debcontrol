//! A crate for parsing [Debian control files].
//!
//! [Debian control files]: https://www.debian.org/doc/debian-policy/ch-controlfields.html
//!
//! # Examples
//! TODO

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use core::fmt;
mod parser;

#[cfg(test)]
mod tests;

/// A single field in a control file.
///
/// All types of fields [(simple, folded, multiline)] are treated the same: all individual value
/// lines (the part after the colon as well as any continuation lines) are trimmed and concatenated
/// together using a single newline character. This means that field values never begin or end with
/// a newline character, but internal newlines are preserved (and may be transformed or ignored when
/// dealing with folded fields). Leading whitespace and trailing whitespace is always removed,
/// including in continuation lines.
///
/// [(simple, folded, multiline)]: https://www.debian.org/doc/debian-policy/ch-controlfields.html#syntax-of-control-files
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Field<'a> {
    pub name: &'a str,
    pub value: String,
}

/// A paragraph in a control file.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Paragraph<'a> {
    pub fields: Vec<Field<'a>>,
}

impl Paragraph<'_> {
    /// Create a new `Paragraph` from the given fields.
    fn new(fields: Vec<Field>) -> Paragraph {
        Paragraph { fields }
    }
}

#[cfg(not(feature = "verbose-errors"))]
type ErrorType<'a> = (&'a str, nom::error::ErrorKind);
#[cfg(feature = "verbose-errors")]
type ErrorType<'a> = nom::error::VerboseError<&'a str>;

/// A parsing syntax error.
///
/// This is an opaque error type that wraps an underlying error. The format and level of detail of
/// the error output depends on the `verbose-errors` feature.
#[derive(Debug)]
pub struct Error<'a> {
    /// The parser input that caused the error.
    input: &'a str,
    /// The underlying error from nom.
    underlying: ErrorType<'a>,
}

impl<'a> fmt::Display for Error<'a> {
    #[cfg(not(feature = "verbose-errors"))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at '{}'",
            self.underlying.1.description(),
            self.underlying.0
        )
    }

    #[cfg(feature = "verbose-errors")]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            nom::error::convert_error(self.input, self.underlying.clone())
        )
    }
}

#[cfg(feature = "std")]
impl<'a> std::error::Error for Error<'a> {}

/// An error result from the streaming parser.
#[derive(Debug)]
pub enum StreamingErr<'a> {
    /// More input is needed.
    ///
    /// This isn't a fatal error; you must read more input from the source and then try again.
    Incomplete,
    /// A syntax error was found.
    ///
    /// This *is* a fatal error; it indicates unambiguously invalid syntax in the input.
    InvalidSyntax(Error<'a>),
}

/// Attempt to parse a paragraph from the given input.
///
/// This function returns a paragraph and any remaining input if a paragraph can be unambiguously
/// parsed. If there's no complete paragraph in the input, an `Err` containing
/// [`Incomplete`](enum.StreamingErr.html#variant.Incomplete) is returned. In that case, you need to
/// either:
///
/// * read more data from the source and try again or
/// * if there's no more data in the source, call [`parse_finish`](fn.parse_finish.html) with all
///   remaining input.
///
/// Any other `Err` result is a fatal error; adding more input and retrying will not change that
/// result.
pub fn parse(input: &str) -> Result<(&str, Paragraph), StreamingErr> {
    match parser::streaming::paragraph::<ErrorType>(input) {
        Ok((rest, Some(item))) => Ok((rest, item)),
        Ok((_, None)) => Err(StreamingErr::Incomplete),
        Err(nom::Err::Error(underlying)) => {
            Err(StreamingErr::InvalidSyntax(Error { input, underlying }))
        }
        Err(nom::Err::Failure(underlying)) => {
            Err(StreamingErr::InvalidSyntax(Error { input, underlying }))
        }
        Err(nom::Err::Incomplete(_)) => Err(StreamingErr::Incomplete),
    }
}

/// Finish parsing the streaming input and return the final remaining paragraph, if any.
///
/// This is the companion function to [`parse`](fn.parse.html). Once all input has been read and
/// `parse` returns [`Incomplete`](enum.StreamingErr.html#variant.Incomplete), call this function
/// with any remaining input to parse the final remaining paragraph. If the remaining input is only
/// whitespace and comments, `None` is returned.
pub fn parse_finish(input: &str) -> Result<Option<Paragraph>, Error> {
    match parser::complete::paragraph::<ErrorType>(input) {
        Ok((_, item)) => Ok(item),
        Err(nom::Err::Error(underlying)) => Err(Error { input, underlying }),
        Err(nom::Err::Failure(underlying)) => Err(Error { input, underlying }),
        Err(nom::Err::Incomplete(_)) => unimplemented!(),
    }
}

/// Parse the given complete control file into paragraphs.
///
/// This function does not work for partial input. The entire control file must be passed in at
/// once.
pub fn parse_complete(input: &str) -> Result<Vec<Paragraph>, Error> {
    let mut paragraphs = Vec::new();

    let mut rest = input;
    loop {
        match parse(rest) {
            Ok((rest2, item)) => {
                paragraphs.push(item);
                rest = rest2;
            }
            Err(StreamingErr::InvalidSyntax(error)) => return Err(error),
            Err(StreamingErr::Incomplete) => break,
        }
    }

    if let Some(paragraph) = parse_finish(rest)? {
        paragraphs.push(paragraph);
    }

    Ok(paragraphs)
}
