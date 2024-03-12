use lex::{skip_white_line_comments, ParseResult};

use crate::tree::parse::message;
use crate::tree::{SourceDec, SourceFile};

/// Parses a source file.
pub fn source_file(s: &str) -> Result<SourceFile, ()> {
    let mut s: &str = s;

    let mut source_file: SourceFile = SourceFile::default();
    loop {
        let (_, rest) = skip_white_line_comments(s, "//")?;
        s = rest;

        if let (Some(source_dec), rest) = source_dec_optional(s)? {
            source_file.add_declaration(source_dec);
            s = rest;
        } else {
            return Ok(source_file);
        }
    }
}

/// Parses an optional source declaration.
fn source_dec_optional(s: &str) -> ParseResult<Option<SourceDec>, ()> {
    if s.is_empty() {
        Ok((None, s))
    } else {
        let (message, s) = message(s)?;
        Ok((Some(message.into()), s))
    }
}
