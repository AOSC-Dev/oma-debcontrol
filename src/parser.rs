use crate::{Field, Paragraph};
use alloc::{string::String, vec::Vec};
use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{char, line_ending, not_line_ending, space0, space1},
    combinator::{complete, cut, map, opt, verify},
    error::{context, make_error, ErrorKind, ParseError},
    multi::{many0, many0_count, many1},
    sequence::{pair, preceded, separated_pair, terminated, tuple},
    Err::Error,
    IResult,
};

fn is_field_name_char(c: char) -> bool {
    c.is_ascii_graphic() && c != ':'
}

fn starts_with_valid_char(name: &str) -> bool {
    !name.starts_with('#') && !name.starts_with('-')
}

fn field_name<'a, E>(input: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    verify(take_while1(is_field_name_char), starts_with_valid_char)(input)
}

fn colon_and_whitespace<'a, E>(input: &'a str) -> IResult<&'a str, (), E>
where
    E: ParseError<&'a str>,
{
    map(pair(char(':'), space0), |_| ())(input)
}

fn field_value<'a, E>(input: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    terminated(not_line_ending, opt(line_ending))(input)
}

struct FieldDefinition<'a> {
    name: &'a str,
    value: &'a str,
}

fn field_definition_line<'a, E>(input: &'a str) -> IResult<&'a str, FieldDefinition<'a>, E>
where
    E: ParseError<&'a str>,
{
    map(
        separated_pair(field_name, cut(colon_and_whitespace), cut(field_value)),
        |(name, value)| FieldDefinition { name, value },
    )(input)
}

enum Line<'a> {
    Continuation(&'a str),
    Comment,
    Blank,
}

fn continuation_line<'a, E>(input: &'a str) -> IResult<&'a str, Line<'a>, E>
where
    E: ParseError<&'a str>,
{
    map(preceded(space1, field_value), |value| {
        Line::Continuation(value)
    })(input)
}

fn comment_line<'a, E>(input: &'a str) -> IResult<&'a str, Line<'a>, E>
where
    E: ParseError<&'a str>,
{
    map(
        tuple((char('#'), not_line_ending, opt(line_ending))),
        |_| Line::Comment,
    )(input)
}

fn blank_line<'a, E>(input: &'a str) -> IResult<&'a str, Line<'a>, E>
where
    E: ParseError<&'a str>,
{
    map(terminated(space0, line_ending), |_| Line::Blank)(input)
}

fn field_from_parts<'a>(parts: (FieldDefinition<'a>, Vec<Line<'a>>)) -> Field<'a> {
    let mut value = String::from(parts.0.value);
    for line in parts.1 {
        if let Line::Continuation(line) = line {
            value.push('\n');
            value.push_str(line);
        }
    }
    Field {
        name: parts.0.name,
        value,
    }
}

fn field_definition<'a, E>(input: &'a str) -> IResult<&'a str, Field<'a>, E>
where
    E: ParseError<&'a str>,
{
    context(
        "field definition",
        map(
            pair(
                field_definition_line,
                many0(alt((continuation_line, comment_line))),
            ),
            field_from_parts,
        ),
    )(input)
}

fn eof<'a, E>(input: &'a str) -> IResult<&'a str, (), E>
where
    E: ParseError<&'a str>,
{
    if input.is_empty() {
        Ok((input, ()))
    } else {
        Err(Error(make_error(input, ErrorKind::Eof)))
    }
}

pub(crate) fn paragraph<'a, E>(input: &'a str) -> IResult<&'a str, Option<Paragraph>, E>
where
    E: ParseError<&'a str>,
{
    complete(preceded(
        many0_count(alt((blank_line, comment_line))),
        terminated(
            opt(map(many1(field_definition), Paragraph::new)),
            context("paragraph terminator", alt((map(blank_line, |_| ()), eof))),
        ),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::field;
    use indoc::indoc;
    use nom::error::ErrorKind;

    type SimpleError<'a> = (&'a str, ErrorKind);

    mod field_name {
        use super::*;

        #[test]
        fn should_parse_field_name_terminated_by_colon() {
            let (rest, name) = field_name::<SimpleError>("field: rest").unwrap();
            assert_eq!(name, "field");
            assert_eq!(rest, ": rest");
        }

        #[test]
        fn should_parse_field_name_terminated_by_whitespace() {
            let (rest, name) = field_name::<SimpleError>("field 1").unwrap();
            assert_eq!(name, "field");
            assert_eq!(rest, " 1");
        }

        #[test]
        fn should_parse_field_name_terminated_by_non_ascii_character() {
            let (rest, name) = field_name::<SimpleError>("fieldä").unwrap();
            assert_eq!(name, "field");
            assert_eq!(rest, "ä");
        }

        #[test]
        fn should_not_parse_empty_field_name() {
            let result = field_name::<SimpleError>(": value");
            assert!(result.is_err());
        }

        #[test]
        fn should_not_parse_field_name_starting_with_comment_character() {
            let result = field_name::<SimpleError>("#field: value");
            assert!(result.is_err());
        }

        #[test]
        fn should_not_parse_field_name_starting_with_hyphen() {
            let result = field_name::<SimpleError>("-field: value");
            assert!(result.is_err());
        }
    }

    mod field_definition {
        use super::*;

        #[test]
        fn should_parse_field_definition() {
            let (rest, item) = field_definition::<SimpleError>("field: value").unwrap();
            assert_eq!(item, field("field", "value"));
            assert_eq!(rest, "");
        }

        #[test]
        fn should_parse_field_definition_with_trailing_newline() {
            let (rest, item) = field_definition::<SimpleError>("field: value\n").unwrap();
            assert_eq!(item, field("field", "value"));
            assert_eq!(rest, "");
        }

        #[test]
        fn should_parse_single_line_field_with_trailing_empty_lines() {
            let (rest, item) = field_definition::<SimpleError>(indoc!(
                "
                field: value  \n
                \t  \t

                "
            ))
            .unwrap();
            assert_eq!(item, field("field", "value  "));
            assert_eq!(rest, "\n\t  \t\n\n");
        }

        #[test]
        fn should_parse_multiline_field() {
            let (rest, item) = field_definition::<SimpleError>(indoc!(
                "
                field: value
                 line 2
                 line 3
                 line 4"
            ))
            .unwrap();
            assert_eq!(item, field("field", "value\nline 2\nline 3\nline 4"));
            assert_eq!(rest, "");
        }

        #[test]
        fn should_parse_multiline_field_with_trailing_empty_lines() {
            let (rest, item) = field_definition::<SimpleError>(indoc!(
                "
                field: 1
                \t2
                
                
                "
            ))
            .unwrap();
            assert_eq!(item, field("field", "1\n2"));
            assert_eq!(rest, "\n\n");
        }

        #[test]
        fn should_parse_field_definition_with_inline_comment() {
            let (rest, item) = field_definition::<SimpleError>(indoc!(
                "
                field: 1
                # comment
                 2
                # comment
                 3
                "
            ))
            .unwrap();
            assert_eq!(item, field("field", "1\n2\n3"));
            assert_eq!(rest, "");
        }

        #[test]
        fn should_parse_single_line_field_definition_with_trailing_comment() {
            let (rest, item) = field_definition::<SimpleError>(indoc!(
                "
                field: value
                # comment"
            ))
            .unwrap();
            assert_eq!(item, field("field", "value"));
            assert_eq!(rest, "");
        }
    }
}
