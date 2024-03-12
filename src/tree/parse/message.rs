use lex::{exact, skip_white_line_comments, whitespace, whitespace_optional, ParseResult};

use crate::tree::parse::{message_field, symbol};
use crate::tree::Message;

/// Parses a message.
pub fn message(s: &str) -> ParseResult<Message, ()> {
    let (_, s) = exact(s, "message")?;
    let (_, s) = whitespace(s)?;
    let (message_name, s) = symbol(s)?;
    let (_, s) = whitespace_optional(s)?;
    let (_, s) = exact(s, "{")?;
    let mut message: Message = Message::from(message_name);
    let (_, s) = message_body(s, &mut message)?;
    Ok((message, s))
}

/// Parses a message body. (after the opening `{`)
fn message_body<'a>(s: &'a str, message: &mut Message) -> ParseResult<'a, (), ()> {
    let mut s: &str = s;
    loop {
        let (_, rest) = skip_white_line_comments(s, "//")?;
        s = rest;

        if let Ok((_, rest)) = exact(s, "}") {
            return Ok(((), rest));
        }

        let (message_field, rest) = message_field(s)?;
        s = rest;
        message.add_field(message_field);
    }
}
