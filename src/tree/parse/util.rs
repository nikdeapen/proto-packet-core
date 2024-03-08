use lex::{until_not, ParseResult};

/// Parses a non-empty symbol string.
pub fn symbol(s: &str) -> ParseResult<&str, ()> {
    until_not(s, |c| !c.is_ascii_alphanumeric() && c != '_')
}
