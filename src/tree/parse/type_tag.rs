use lex::ParseResult;

use crate::tree::parse::util::symbol;
use crate::tree::PrimitiveType::*;
use crate::tree::{PrimitiveType, TypeTag};

/// Parses a type tag.
pub fn type_tag(s: &str) -> ParseResult<TypeTag, ()> {
    if let Ok((primitive, s)) = primitive_type(s) {
        Ok((primitive.to_type_tag(), s))
    } else {
        Err(())
    }
}

/// Parses a primitive type.
pub fn primitive_type(s: &str) -> ParseResult<PrimitiveType, ()> {
    let (name, s) = symbol(s)?;
    let primitive: PrimitiveType = match name {
        "u8" => UnsignedInt8,
        "u16" => UnsignedInt16,
        "u32" => UnsignedInt32,
        "u64" => UnsignedInt64,
        _ => return Err(()),
    };
    Ok((primitive, s))
}
