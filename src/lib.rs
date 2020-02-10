#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use core::fmt;
mod parser;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Field<'a> {
    pub name: &'a str,
    pub value: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Paragraph<'a> {
    pub fields: Vec<Field<'a>>,
}

impl Paragraph<'_> {
    fn new(fields: Vec<Field>) -> Paragraph {
        Paragraph { fields }
    }
}

#[cfg(not(feature = "verbose-errors"))]
type ErrorType<'a> = (&'a str, nom::error::ErrorKind);
#[cfg(feature = "verbose-errors")]
type ErrorType<'a> = nom::error::VerboseError<&'a str>;

#[derive(Debug)]
pub struct Error<'a> {
    input: &'a str,
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

#[derive(Debug)]
pub enum StreamingErr<'a> {
    Incomplete,
    InvalidSyntax(Error<'a>),
}

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

pub fn parse_finish(input: &str) -> Result<Option<Paragraph>, Error> {
    match parser::complete::paragraph::<ErrorType>(input) {
        Ok((_, item)) => Ok(item),
        Err(nom::Err::Error(underlying)) => Err(Error { input, underlying }),
        Err(nom::Err::Failure(underlying)) => Err(Error { input, underlying }),
        Err(nom::Err::Incomplete(_)) => unimplemented!(),
    }
}

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
