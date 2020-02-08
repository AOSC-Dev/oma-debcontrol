#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use core::fmt;

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
type ErrorType<'a> = (&'a str, nom::error::ErrorKind);
#[cfg(feature = "verbose-errors")]
type ErrorType<'a> = nom::error::VerboseError<&'a str>;

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
        write!(
            f,
            "{}",
            nom::error::convert_error(self.input, self.error.clone())
        )
    }
}

#[cfg(feature = "std")]
impl<'a> std::error::Error for Error<'a> {}

pub fn next_paragraph(input: &str) -> Result<(&str, Option<Paragraph>), Error> {
    match parser::paragraph::<ErrorType>(input) {
        Ok((rest, item)) => Ok((rest, item)),
        Err(nom::Err::Error(error)) => Err(Error { input, error }),
        Err(nom::Err::Failure(error)) => Err(Error { input, error }),
        Err(nom::Err::Incomplete(_)) => unimplemented!(),
    }
}

#[derive(Debug)]
pub struct ParserIterator<'a> {
    input: &'a str,
}

impl<'a> Iterator for ParserIterator<'a> {
    type Item = Result<Paragraph<'a>, Error<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        match next_paragraph(self.input) {
            Ok((rest, Some(paragraph))) => {
                self.input = rest;
                Some(Ok(paragraph))
            }
            Ok((rest, None)) => {
                self.input = rest;
                None
            }
            Err(err) => Some(Err(err)),
        }
    }
}

pub fn parse(input: &str) -> ParserIterator {
    ParserIterator { input }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    pub(crate) fn field<'a>(name: &'a str, value: &'a str) -> Field<'a> {
        Field {
            name,
            value: value.to_string(),
        }
    }

    mod next_paragraph {
        use super::*;
        use alloc::vec;
        use indoc::indoc;

        #[test]
        fn should_parse_simple_paragraph() {
            let (rest, item) = next_paragraph(indoc!(
                "
            field: value
            field2: value 2
            field3: value 3"
            ))
            .unwrap();
            assert_eq!(
                item,
                Some(Paragraph::new(vec![
                    field("field", "value"),
                    field("field2", "value 2"),
                    field("field3", "value 3"),
                ]))
            );
            assert_eq!(rest, "");
        }

        #[test]
        fn should_fail_on_incomplete_line() {
            let result = next_paragraph(indoc!(
                "
            field: value
             continuation
            incomplete-line
            "
            ));
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_on_unexpected_continuation() {
            let result = next_paragraph(indoc!(
                "
            
             continuation
            field: value
            "
            ));
            assert!(result.is_err());
        }

        #[test]
        fn should_parse_paragraph_with_continuation_lines() {
            let (rest, item) = next_paragraph(indoc!(
                "
            field1: value
             line2
             line3
            field2: value
             line2
            field3: value
            field4: value
             line2


            "
            ))
            .unwrap();
            assert_eq!(
                item,
                Some(Paragraph::new(vec![
                    field("field1", "value\nline2\nline3"),
                    field("field2", "value\nline2"),
                    field("field3", "value"),
                    field("field4", "value\nline2"),
                ]))
            );
            assert_eq!(rest, "\n");
        }

        #[test]
        fn should_parse_paragraph_with_comment_lines() {
            let (rest, item) = next_paragraph(indoc!(
                "
            field1: value
            # comment
            field2: value
            # comment
             line2
            # comment
            # comment
            # more comments
            field3: value
            "
            ))
            .unwrap();
            assert_eq!(
                item,
                Some(Paragraph::new(vec![
                    field("field1", "value"),
                    field("field2", "value\nline2"),
                    field("field3", "value"),
                ]))
            );
            assert_eq!(rest, "");
        }

        #[test]
        fn should_parse_one_of_multiple_paragraphs() {
            let (rest, item) = next_paragraph(indoc!(
                "
            field: value
            field: value
            # comment
            field: value
            \tanother line
            # comment

            another: paragraph
            # more stuff
            field: value
            "
            ))
            .unwrap();
            assert_eq!(
                item,
                Some(Paragraph::new(vec![
                    field("field", "value"),
                    field("field", "value"),
                    field("field", "value\nanother line"),
                ]))
            );
            assert_eq!(
                rest,
                indoc!(
                    "
                another: paragraph
                # more stuff
                field: value
                "
                )
            );
        }

        #[test]
        fn should_parse_paragraph_with_leading_whitespace() {
            let (rest, item) = next_paragraph(indoc!(
                "

            \t\t
              \t

            field: value
            field2: value2
             line2
            "
            ))
            .unwrap();
            assert_eq!(
                item,
                Some(Paragraph::new(vec![
                    field("field", "value"),
                    field("field2", "value2\nline2"),
                ]))
            );
            assert_eq!(rest, "");
        }

        #[test]
        fn should_parse_paragraph_with_leading_comments() {
            let (rest, item) = next_paragraph(indoc!(
                "
            # comment
            field: value
            "
            ))
            .unwrap();
            assert_eq!(item, Some(Paragraph::new(vec![field("field", "value")])));
            assert_eq!(rest, "");
        }

        #[test]
        fn should_parse_paragraph_with_leading_whitespace_and_comments() {
            let (rest, item) = next_paragraph(indoc!(
                "

            \t
            # comment
            \t
            # comments

            field: value
            "
            ))
            .unwrap();
            assert_eq!(item, Some(Paragraph::new(vec![field("field", "value")])));
            assert_eq!(rest, "");
        }

        #[test]
        fn should_return_none_for_input_without_a_paragraph() {
            let (rest, item) = next_paragraph(indoc!(
                "
            \t
            # comment

            # comment


            \t\t  \n

            # comment


            "
            ))
            .unwrap();
            assert_eq!(item, None);
            assert_eq!(rest, "");
        }

        #[test]
        fn should_return_none_for_empty_string() {
            let (rest, item) = next_paragraph("").unwrap();
            assert_eq!(item, None);
            assert_eq!(rest, "");
        }
    }

    mod parse {
        use super::*;
        use indoc::indoc;

        #[test]
        fn should_parse_multiple_paragraphs() {
            let parser = parse(indoc!(
                "
                field: value
                field: value
                 cont
                
                # comment
                field: value
                field2: value2
                # comment
                
                
                field3: value3
                 continuation
                field4: value4
                
                "
            ));
            assert_eq!(
                parser.map(Result::unwrap).collect::<Vec<_>>(),
                vec![
                    Paragraph::new(vec![field("field", "value"), field("field", "value\ncont"),]),
                    Paragraph::new(vec![field("field", "value"), field("field2", "value2"),]),
                    Paragraph::new(vec![
                        field("field3", "value3\ncontinuation"),
                        field("field4", "value4"),
                    ]),
                ]
            );
        }

        #[test]
        fn should_return_error_when_encountering_invalid_paragraph() {
            let mut parser = parse(indoc!(
                "
                field: value
                
                # invalid:
                 continuation
                field2: value2
                "
            ));
            assert_eq!(
                parser.next().unwrap().unwrap(),
                Paragraph::new(vec![field("field", "value")])
            );
            assert!(parser.next().unwrap().is_err());
            assert!(parser.next().unwrap().is_err());
        }
    }
}
