use alloc::{string::String, vec::Vec};

use crate::Field;

pub mod complete;
pub mod streaming;

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
