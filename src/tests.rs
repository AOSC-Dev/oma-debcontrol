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

mod parse {
    use super::*;

    #[test]
    fn should_parse_completed_paragraph() {
        let (rest, item) = parse(indoc!(
            "
            field: value
            field2: value2
            # comment 1
            field3: line1
             line2
            # comment 2
             line3
            
            "
        ))
        .unwrap();
        assert_eq!(
            item,
            Paragraph::new(vec![
                field("field", "value"),
                field("field2", "value2"),
                field("field3", "line1\nline2\nline3"),
            ])
        );
        assert_eq!(rest, "");
    }

    #[test]
    fn should_parse_completed_paragraph_followed_by_partial_paragraph() {
        let (rest, item) = parse(indoc!(
            "
            
            # comment
            field: value
             cont
            
            # comment
            
            field2: value2
            # comment
            "
        ))
        .unwrap();
        assert_eq!(item, Paragraph::new(vec![field("field", "value\ncont"),]));
        assert_eq!(
            rest,
            indoc!(
                "
                # comment
                
                field2: value2
                # comment
                "
            )
        );
    }

    #[test]
    fn should_return_incomplete_on_incomplete_field_definition() {
        let result = parse(indoc!(
            "
            field"
        ));
        assert_matches!(result, Err(StreamingErr::Incomplete));
    }

    #[test]
    fn should_return_incomplete_on_field_definition_without_trailing_newline() {
        let result = parse(indoc!(
            "
            field: value"
        ));
        assert_matches!(result, Err(StreamingErr::Incomplete));
    }

    #[test]
    fn should_return_incomplete_on_paragraph_without_trailing_empty_line() {
        let result = parse(indoc!(
            "
            field: value
             continuation
            "
        ));
        assert_matches!(result, Err(StreamingErr::Incomplete));
    }

    #[test]
    fn should_return_incomplete_on_paragraph_without_trailing_newline() {
        let result = parse(indoc!(
            "
            field: value
             continuation"
        ));
        assert_matches!(result, Err(StreamingErr::Incomplete));
    }

    #[test]
    fn should_return_incomplete_on_empty_string() {
        let result = parse("");
        assert_matches!(result, Err(StreamingErr::Incomplete));
    }

    #[test]
    fn should_return_incomplete_on_input_without_paragraph() {
        let result = parse(indoc!(
            "
            
            \t\t
            
            # comment
            # comment
            
            \t
            
            
            # comment"
        ));
        assert_matches!(result, Err(StreamingErr::Incomplete));
    }

    #[test]
    fn should_return_error_on_unexpected_continuation() {
        let result = parse(indoc!(
            "
            \tunexpected continuation
            "
        ));
        assert_matches!(result, Err(StreamingErr::InvalidSyntax(_)));
    }

    #[test]
    fn should_return_error_on_incomplete_field_definition() {
        let result = parse(indoc!(
            "
            field
            
            "
        ));
        assert_matches!(result, Err(StreamingErr::InvalidSyntax(_)));
    }

    #[test]
    fn should_return_error_on_field_name_starting_with_hyphen() {
        let result = parse(indoc!(
            "
            -field: value"
        ));
        assert_matches!(result, Err(StreamingErr::InvalidSyntax(_)));
    }

    #[test]
    fn should_return_error_on_invalid_field_name() {
        let result = parse(indoc!(
            "
            field äöü: value
            
            "
        ));
        assert_matches!(result, Err(StreamingErr::InvalidSyntax(_)));
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
        ))
        .unwrap();
        assert_eq!(
            item,
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
        ))
        .unwrap();
        assert_eq!(
            item,
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

mod parse_complete {
    use super::*;

    #[test]
    fn should_parse_multiple_paragraphs() {
        let items = parse_complete(indoc!(
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
        ))
        .unwrap();
        assert_eq!(
            items,
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
        let items = parse_complete(indoc!(
            "
            
            \t\t
            
            # comment
            \t
            
                \n
            "
        ))
        .unwrap();
        assert_eq!(items, vec![]);
    }

    #[test]
    fn should_return_error_on_invalid_syntax() {
        let result = parse_complete(indoc!(
            "
            field 15: value
            "
        ));
        assert_matches!(result, Err(_));
    }
}
