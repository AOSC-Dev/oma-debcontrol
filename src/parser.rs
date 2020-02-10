use crate::Field;
use alloc::{string::String, vec::Vec};

fn is_field_name_char(c: char) -> bool {
    c.is_ascii_graphic() && c != ':'
}

fn starts_with_valid_char(name: &str) -> bool {
    !name.starts_with('#') && !name.starts_with('-')
}

struct FieldDefinition<'a> {
    name: &'a str,
    value: &'a str,
}

enum Line<'a> {
    Continuation(&'a str),
    Comment,
    Blank,
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

macro_rules! parsers {
    ($name: ident) => {
        pub(crate) mod $name {
            use super::{
                field_from_parts, is_field_name_char, starts_with_valid_char, FieldDefinition, Line,
            };
            use crate::{Field, Paragraph};
            use nom::{
                branch::alt,
                bytes::$name::take_while1,
                character::$name::{char, line_ending, not_line_ending, space0, space1},
                combinator::{cut, map, opt, verify},
                error::{context, make_error, ErrorKind, ParseError},
                multi::{many0, many0_count, many1},
                sequence::{pair, preceded, separated_pair, terminated, tuple},
                Err::Error,
                IResult,
            };

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

            fn field_definition_line<'a, E>(
                input: &'a str,
            ) -> IResult<&'a str, FieldDefinition<'a>, E>
            where
                E: ParseError<&'a str>,
            {
                map(
                    separated_pair(field_name, cut(colon_and_whitespace), cut(field_value)),
                    |(name, value)| FieldDefinition { name, value },
                )(input)
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
                preceded(
                    many0_count(alt((blank_line, comment_line))),
                    terminated(
                        opt(map(many1(field_definition), Paragraph::new)),
                        context("paragraph terminator", alt((map(blank_line, |_| ()), eof))),
                    ),
                )(input)
            }
        }
    };
}

parsers!(streaming);
parsers!(complete);
