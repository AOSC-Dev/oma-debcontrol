use super::*;
use alloc::{string::ToString, vec};
use assert_matches::assert_matches;
use indoc::indoc;

pub(crate) fn field<'a>(name: &'a str, value: &'a str) -> Field<'a> {
    Field {
        name,
        value: value.to_string(),
    }
}

mod parse_streaming {
    use super::*;

    #[test]
    fn should_parse_completed_paragraph() {
        let result = parse_streaming(indoc!(
            "
            field: value
            field2: value2
            # comment 1
            field3: line1
             line2
            # comment 2
             line3
            
            "
        ));
        assert_eq!(
            result.unwrap(),
            Streaming::Item((
                "",
                Paragraph::new(vec![
                    field("field", "value"),
                    field("field2", "value2"),
                    field("field3", "line1\nline2\nline3"),
                ])
            ))
        );
    }

    #[test]
    fn should_parse_completed_paragraph_followed_by_partial_paragraph() {
        let result = parse_streaming(indoc!(
            "
            
            # comment
            field: value
             cont
            
            # comment
            
            field2: value2
            # comment
            "
        ));
        assert_eq!(
            result.unwrap(),
            Streaming::Item((
                indoc!(
                    "
                    # comment
                    
                    field2: value2
                    # comment
                    "
                ),
                Paragraph::new(vec![field("field", "value\ncont"),])
            ))
        );
    }

    #[test]
    fn should_return_incomplete_on_incomplete_field_definition() {
        let result = parse_streaming(indoc!(
            "
            field"
        ));
        assert_matches!(result, Ok(Streaming::Incomplete));
    }

    #[test]
    fn should_return_incomplete_on_field_definition_without_trailing_newline() {
        let result = parse_streaming(indoc!(
            "
            field: value"
        ));
        assert_matches!(result, Ok(Streaming::Incomplete));
    }

    #[test]
    fn should_return_incomplete_on_paragraph_without_trailing_empty_line() {
        let result = parse_streaming(indoc!(
            "
            field: value
             continuation
            "
        ));
        assert_matches!(result, Ok(Streaming::Incomplete));
    }

    #[test]
    fn should_return_incomplete_on_paragraph_without_trailing_newline() {
        let result = parse_streaming(indoc!(
            "
            field: value
             continuation"
        ));
        assert_matches!(result, Ok(Streaming::Incomplete));
    }

    #[test]
    fn should_return_incomplete_on_empty_string() {
        let result = parse_streaming("");
        assert_matches!(result, Ok(Streaming::Incomplete));
    }

    #[test]
    fn should_return_incomplete_on_input_without_paragraph() {
        let result = parse_streaming(indoc!(
            "
            
            \t\t
            
            # comment
            # comment
            
            \t
            
            
            # comment"
        ));
        assert_matches!(result, Ok(Streaming::Incomplete));
    }

    #[test]
    fn should_return_error_on_unexpected_continuation() {
        let result = parse_streaming(indoc!(
            "
            \tunexpected continuation
            "
        ));
        assert_matches!(result, Err(_));
    }

    #[test]
    fn should_return_error_on_incomplete_field_definition() {
        let result = parse_streaming(indoc!(
            "
            field
            
            "
        ));
        assert_matches!(result, Err(_));
    }

    #[test]
    fn should_return_error_on_field_name_starting_with_hyphen() {
        let result = parse_streaming(indoc!(
            "
            -field: value"
        ));
        assert_matches!(result, Err(_));
    }

    #[test]
    fn should_return_error_on_invalid_field_name() {
        let result = parse_streaming(indoc!(
            "
            field äöü: value
            
            "
        ));
        assert_matches!(result, Err(_));
    }
}

mod parse_finish {
    use super::*;

    #[test]
    fn should_parse_paragraph_with_trailing_whitespace() {
        let item = parse_finish(indoc!(
            "
            field: value
            field2: value
            
            # comment
            
            
            
            "
        ));
        assert_eq!(
            item.unwrap(),
            Some(Paragraph::new(vec![
                field("field", "value"),
                field("field2", "value")
            ]))
        );
    }

    #[test]
    fn should_parse_paragraph_without_trailing_newline() {
        let item = parse_finish(indoc!(
            "
            field: value
            field2: line1
            # comment
            \tline2"
        ));
        assert_eq!(
            item.unwrap(),
            Some(Paragraph::new(vec![
                field("field", "value"),
                field("field2", "line1\nline2")
            ]))
        );
    }

    #[test]
    fn should_return_error_on_incomplete_field_definition() {
        let result = parse_finish("field");
        assert_matches!(result, Err(_));
    }
}

mod parse_str {
    use super::*;

    #[test]
    fn should_parse_multiple_paragraphs() {
        let items = parse_str(indoc!(
            "
            # comment
            
            field: value
            field2: line1
             line2
            # comment
            
            field3: value3
            field4: value4
            
            # comment
            
            field5: value5
            field6: value6"
        ));
        assert_eq!(
            items.unwrap(),
            vec![
                Paragraph::new(vec![
                    field("field", "value"),
                    field("field2", "line1\nline2"),
                ]),
                Paragraph::new(vec![field("field3", "value3"), field("field4", "value4"),]),
                Paragraph::new(vec![field("field5", "value5"), field("field6", "value6"),]),
            ]
        )
    }

    #[test]
    fn should_parse_empty_input() {
        let items = parse_str(indoc!(
            "
            
            \t\t
            
            # comment
            \t
            
                \n
            "
        ));
        assert_eq!(items.unwrap(), vec![]);
    }

    #[test]
    fn should_return_error_on_invalid_syntax() {
        let result = parse_str(indoc!(
            "
            field 15: value
            "
        ));
        assert_matches!(result, Err(_));
    }
}
