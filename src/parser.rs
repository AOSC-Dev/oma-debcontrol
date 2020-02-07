use crate::{Field, Paragraph};
use alloc::string::String;
use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{char, line_ending, not_line_ending, space0, space1},
    combinator::{map, opt, verify},
    multi::{many0, many0_count, many1},
    sequence::{pair, preceded, separated_pair, terminated, tuple},
    IResult,
};

fn is_field_name_char(c: char) -> bool {
    c.is_ascii_graphic() && c != ':'
}

fn starts_with_valid_char(name: &str) -> bool {
    !name.starts_with("#") && !name.starts_with("-")
}

fn field_name(input: &str) -> IResult<&str, &str> {
    verify(take_while1(is_field_name_char), starts_with_valid_char)(input)
}

fn colon_and_whitespace(input: &str) -> IResult<&str, ()> {
    map(pair(char(':'), space0), |_| ())(input)
}

fn field_value(input: &str) -> IResult<&str, &str> {
    terminated(not_line_ending, opt(line_ending))(input)
}

enum Line<'a> {
    Continuation(&'a str),
    Comment,
    Blank,
}

fn field_definition_line(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(field_name, colon_and_whitespace, field_value)(input)
}

fn continuation_line(input: &str) -> IResult<&str, Line> {
    map(preceded(space1, field_value), |value| {
        Line::Continuation(value)
    })(input)
}

fn comment_line(input: &str) -> IResult<&str, Line> {
    map(
        tuple((char('#'), not_line_ending, opt(line_ending))),
        |_| Line::Comment,
    )(input)
}

fn blank_line(input: &str) -> IResult<&str, Line> {
    map(terminated(space0, line_ending), |_| Line::Blank)(input)
}

fn field_definition(input: &str) -> IResult<&str, Field> {
    map(
        pair(
            field_definition_line,
            many0(alt((continuation_line, comment_line))),
        ),
        |((name, first_line), more_lines)| {
            let mut s = String::from(first_line);
            for line in more_lines {
                match line {
                    Line::Continuation(value) => {
                        s.push('\n');
                        s.push_str(value);
                    }
                    _ => {}
                }
            }
            Field { name, value: s }
        },
    )(input)
}

pub(crate) fn paragraph(input: &str) -> IResult<&str, Option<Paragraph>> {
    opt(map(
        preceded(
            many0_count(alt((blank_line, comment_line))),
            many1(field_definition),
        ),
        Paragraph::new,
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod field_name {
        use super::*;

        #[test]
        fn should_parse_field_name_terminated_by_colon() {
            let (rest, name) = field_name("field: rest").unwrap();
            assert_eq!(name, "field");
            assert_eq!(rest, ": rest");
        }

        #[test]
        fn should_parse_field_name_terminated_by_whitespace() {
            let (rest, name) = field_name("field 1").unwrap();
            assert_eq!(name, "field");
            assert_eq!(rest, " 1");
        }

        #[test]
        fn should_parse_field_name_terminated_by_non_ascii_character() {
            let (rest, name) = field_name("fieldä").unwrap();
            assert_eq!(name, "field");
            assert_eq!(rest, "ä");
        }

        #[test]
        fn should_not_parse_empty_field_name() {
            let result = field_name(": value");
            assert!(result.is_err());
        }

        #[test]
        fn should_not_parse_field_name_starting_with_comment_character() {
            let result = field_name("#field: value");
            assert!(result.is_err());
        }

        #[test]
        fn should_not_parse_field_name_starting_with_hyphen() {
            let result = field_name("-field: value");
            assert!(result.is_err());
        }
    }

    mod field_definition {
        use super::*;

        #[test]
        fn should_parse_field_definition() {
            let (rest, item) = field_definition("field: value").unwrap();
            assert_eq!(item, Field::new("field", "value"));
            assert_eq!(rest, "");
        }

        #[test]
        fn should_parse_field_definition_with_trailing_newline() {
            let (rest, item) = field_definition("field: value\n").unwrap();
            assert_eq!(item, Field::new("field", "value"));
            assert_eq!(rest, "");
        }

        #[test]
        fn should_parse_single_line_field_with_trailing_empty_lines() {
            let (rest, item) = field_definition("field: value  \n\n\t\t\n  \n").unwrap();
            assert_eq!(item, Field::new("field", "value  "));
            assert_eq!(rest, "\n\t\t\n  \n");
        }

        #[test]
        fn should_parse_multiline_field() {
            let (rest, item) = field_definition("field: value\n line 2\n line 3\n line 4").unwrap();
            assert_eq!(item, Field::new("field", "value\nline 2\nline 3\nline 4"));
            assert_eq!(rest, "");
        }

        #[test]
        fn should_parse_multiline_field_with_trailing_empty_lines() {
            let (rest, item) = field_definition("field: 1\n\t2\n\n\n  ").unwrap();
            assert_eq!(item, Field::new("field", "1\n2"));
            assert_eq!(rest, "\n\n  ");
        }

        #[test]
        fn should_parse_field_definition_with_inline_comment() {
            let (rest, item) =
                field_definition("field: 1\n# comment\n 2\n# comment\n 3\n").unwrap();
            assert_eq!(item, Field::new("field", "1\n2\n3"));
            assert_eq!(rest, "");
        }

        #[test]
        fn should_parse_single_line_field_definition_with_trailing_comment() {
            let (rest, item) = field_definition("field: value\n# comment").unwrap();
            assert_eq!(item, Field::new("field", "value"));
            assert_eq!(rest, "");
        }
    }

    mod paragraph {
        use super::*;
        use alloc::vec;
        use indoc::indoc;

        #[test]
        fn should_parse_paragraph() {
            let (rest, item) = paragraph(indoc!(
                "
                field1: value
                field2: value
                field3: value"
            ))
            .unwrap();
            assert_eq!(
                item,
                Some(Paragraph::new(vec![
                    Field::new("field1", "value"),
                    Field::new("field2", "value"),
                    Field::new("field3", "value"),
                ]))
            );
            assert_eq!(rest, "");
        }

        #[test]
        fn should_parse_paragraph_with_continuation_lines() {
            let (rest, item) = paragraph(indoc!(
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
                    Field::new("field1", "value\nline2\nline3"),
                    Field::new("field2", "value\nline2"),
                    Field::new("field3", "value"),
                    Field::new("field4", "value\nline2"),
                ]))
            );
            assert_eq!(rest, "\n\n");
        }

        #[test]
        fn should_parse_paragraph_with_comment_lines() {
            let (rest, item) = paragraph(indoc!(
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
                    Field::new("field1", "value"),
                    Field::new("field2", "value\nline2"),
                    Field::new("field3", "value"),
                ]))
            );
            assert_eq!(rest, "");
        }

        #[test]
        fn should_parse_one_of_multiple_paragraphs() {
            let (rest, item) = paragraph(indoc!(
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
                    Field::new("field", "value"),
                    Field::new("field", "value"),
                    Field::new("field", "value\nanother line"),
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
            let (rest, item) = paragraph(indoc!(
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
                    Field::new("field", "value"),
                    Field::new("field2", "value2\nline2"),
                ]))
            );
            assert_eq!(rest, "");
        }

        #[test]
        fn should_parse_paragraph_with_leading_comments() {
            let (rest, item) = paragraph(indoc!(
                "
                # comment
                field: value
                "
            ))
            .unwrap();
            assert_eq!(
                item,
                Some(Paragraph::new(vec![Field::new("field", "value")]))
            );
            assert_eq!(rest, "");
        }

        #[test]
        fn should_parse_paragraph_with_leading_whitespace_and_comments() {
            let (rest, item) = paragraph(indoc!(
                "
                
                \t
                # comment
                \t
                # comments
                
                field: value
                "
            ))
            .unwrap();
            assert_eq!(
                item,
                Some(Paragraph::new(vec![Field::new("field", "value")]))
            );
            assert_eq!(rest, "");
        }
    }
}
