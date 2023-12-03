//! A crate for parsing [Debian control files].
//!
//! [Debian control files]: https://www.debian.org/doc/debian-policy/ch-controlfields.html
//!
//! # Parse complete input
//! The [`parse_str`](fn.parse_str.html) function will parse a complete control file into a vec of
//! [`Paragraph`](struct.Paragraph.html) values:
//! ```
//! # use debcontrol::{Paragraph, Field, parse_str};
//! # fn main() -> Result<(), debcontrol::SyntaxError<'static>> {
//! let paragraphs = parse_str("
//! a-field: with a value
//! another-field: with a...
//!  ...continuation
//! ")?;
//!
//! assert_eq!(paragraphs, vec![Paragraph {
//!     fields: vec![
//!         Field { name: "a-field", value: String::from("with a value") },
//!         Field { name: "another-field", value: String::from("with a...\n...continuation") }
//!     ]
//! }]);
//! # Ok(())
//! # }
//! ```
//!
//! # Parse streaming input
//! The [`parse_streaming`](fn.parse_streaming.html) and [`parse_finish`](fn.parse_finish.html)
//! functions can be used to parse a control file incrementally:
//! ```
//! # use debcontrol::{Paragraph, Field, Streaming, parse_streaming, parse_finish};
//! # fn main() -> Result<(), debcontrol::SyntaxError<'static>> {
//! let result = parse_streaming("field: value")?;
//! assert_eq!(result, Streaming::Incomplete);
//!
//! let result = parse_streaming("field: value\n\n")?;
//! assert_eq!(result, Streaming::Item(("", Paragraph {
//!     fields: vec![
//!         Field { name: "field", value: String::from("value") }
//!     ]
//! })));
//!
//! let result = parse_finish("remaining: input")?;
//! assert_eq!(result, Some(Paragraph {
//!     fields: vec![
//!         Field { name: "remaining", value: String::from("input") }
//!     ]
//! }));
//! # Ok(())
//! # }
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::{string::String, vec::Vec};
use core::fmt;

mod buf_parse;
mod parser;
pub use buf_parse::*;
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
/// This is an opaque error type that wraps an underlying syntax error. The format and level of
/// detail of the error output depends on the `verbose-errors` feature.
#[derive(Debug)]
pub struct SyntaxError<'a> {
    /// The parser input that caused the error.
    pub input: &'a str,
    /// The underlying nom error.
    pub underlying: ErrorType<'a>,
}

impl<'a> fmt::Display for SyntaxError<'a> {
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
impl<'a> std::error::Error for SyntaxError<'a> {}

/// A return value from the streaming parser.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Streaming<T> {
    /// An item returned from the parser.
    Item(T),
    /// More input is needed to make a parsing decision.
    Incomplete,
}

/// Attempt to parse a paragraph from the given input.
///
/// This function returns a paragraph and any remaining input if a paragraph can be unambiguously
/// parsed. If there's no complete paragraph in the input,
/// [`Streaming::Incomplete`](enum.Streaming.html#variant.Incomplete) is returned. In that case,
/// you need to either:
///
/// * read more data from the source and try again or
/// * if there's no more data in the source, call [`parse_finish`](fn.parse_finish.html) with all
///   remaining input.
pub fn parse_streaming(input: &str) -> Result<Streaming<(&str, Paragraph)>, SyntaxError> {
    match parser::streaming::paragraph::<ErrorType>(input) {
        Ok((remaining, Some(item))) => Ok(Streaming::Item((remaining, item))),
        Ok((_, None)) => Ok(Streaming::Incomplete),
        Err(nom::Err::Incomplete(_)) => Ok(Streaming::Incomplete),
        Err(nom::Err::Error(underlying)) => Err(SyntaxError { input, underlying }),
        Err(nom::Err::Failure(underlying)) => Err(SyntaxError { input, underlying }),
    }
}

/// Finish parsing the streaming input and return the final remaining paragraph, if any.
///
/// This is the companion function to [`parse_streaming`](fn.parse_streaming.html). Once all input
/// has been read and `parse_streaming` returns
/// [`Incomplete`](enum.Streaming.html#variant.Incomplete), call this function with any remaining
/// input to parse the final remaining paragraph. If the remaining input is only whitespace and
/// comments, `None` is returned.
pub fn parse_finish(input: &str) -> Result<Option<Paragraph>, SyntaxError> {
    match parser::complete::paragraph::<ErrorType>(input) {
        Ok((_, item)) => Ok(item),
        Err(nom::Err::Error(underlying)) => Err(SyntaxError { input, underlying }),
        Err(nom::Err::Failure(underlying)) => Err(SyntaxError { input, underlying }),
        Err(nom::Err::Incomplete(_)) => unimplemented!(),
    }
}

/// Parse the given complete control file into paragraphs.
///
/// This function does not work for partial input. The entire control file must be passed in at
/// once.
pub fn parse_str(input: &str) -> Result<Vec<Paragraph>, SyntaxError> {
    let mut paragraphs = Vec::new();

    let mut input = input;
    while let Streaming::Item((remaining, item)) = parse_streaming(input)? {
        paragraphs.push(item);
        input = remaining;
    }
    if let Some(paragraph) = parse_finish(input)? {
        paragraphs.push(paragraph);
    }

    Ok(paragraphs)
}
