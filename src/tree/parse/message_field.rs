use lex::{exact, value_u32, whitespace_optional, ParseResult};

use crate::tree::parse::{symbol, type_tag};
use crate::tree::MessageField;

/// Parses a message field.
pub fn message_field(s: &str) -> ParseResult<MessageField, ()> {
    let (name, s) = symbol(s)?;
    let (_, s) = whitespace_optional(s)?;
    let (type_tag, s) = type_tag(s)?;
    let mut message_field: MessageField = MessageField::from((name, type_tag));
    let (field_number, s) = field_number(s)?;
    if let Some(field_number) = field_number {
        message_field.set_field_number(field_number);
    }
    let (_, s) = whitespace_optional(s)?;
    let (_, s) = exact(s, ";")?;
    Ok((message_field, s))
}

/// Parses the optional field number.
fn field_number(s: &str) -> ParseResult<Option<u32>, ()> {
    let (_, s) = whitespace_optional(s)?;
    if let Ok((_, s)) = exact(s, "=") {
        let (_, s) = whitespace_optional(s)?;
        let (field_number, s) = value_u32(s)?;
        Ok((Some(field_number), s))
    } else {
        Ok((None, s))
    }
}
