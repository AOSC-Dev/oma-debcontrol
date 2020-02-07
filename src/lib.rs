#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use core::fmt;
use nom::error::{convert_error, ErrorKind, VerboseError};

mod parser;

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
type ErrorType<'a> = (&'a str, ErrorKind);
#[cfg(feature = "verbose-errors")]
type ErrorType<'a> = VerboseError<&'a str>;

#[derive(Debug)]
pub struct Error<'a> {
    input: &'a str,
    error: ErrorType<'a>,
}

impl<'a> fmt::Display for Error<'a> {
    #[cfg(not(feature = "verbose-errors"))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at '{}'", self.error.1.description(), self.error.0)
    }

    #[cfg(feature = "verbose-errors")]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", convert_error(self.input, self.error.clone()))
    }
}

#[cfg(feature = "std")]
impl<'a> std::error::Error for Error<'a> {}

pub fn parse(input: &str) -> Result<(Option<Paragraph>, &str), Error> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{string::ToString, vec};
    use indoc::indoc;

    pub(crate) fn field<'a>(name: &'a str, value: &'a str) -> Field<'a> {
        Field {
            name,
            value: value.to_string(),
        }
    }

    #[test]
    fn should_parse_simple_paragraph() {
        let (item, rest) = parse(indoc!(
            "
            field: value
            field 2: value 2
            field 3: value 3
            "
        ))
        .unwrap();
        assert_eq!(
            item,
            Some(Paragraph::new(vec![
                field("field", "value"),
                field("field 2", "value 2"),
                field("field 3", "value 3"),
            ]))
        );
        assert_eq!(rest, "");
    }
}
