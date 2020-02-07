#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
#[cfg(feature = "std")]
use thiserror::Error;

mod parser;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Field<'a> {
    pub name: &'a str,
    pub value: String,
}

impl Field<'_> {
    fn new<'a>(name: &'a str, value: &'a str) -> Field<'a> {
        Field {
            name,
            value: value.to_string(),
        }
    }
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

#[derive(Debug)]
#[cfg_attr(feature = "std", derive(Error))]
pub enum Error {
    #[cfg_attr(feature = "std", error("no paragraph in input"))]
    EmptyInput,
}

pub fn parse(input: &str) -> Result<(Option<Paragraph>, &str), Error> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use indoc::indoc;

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
            Paragraph::new(vec![
                Field::new("field", "value"),
                Field::new("field 2", "value 2"),
                Field::new("field 3", "value 3"),
            ])
        );
        assert_eq!(rest, "");
    }
}
