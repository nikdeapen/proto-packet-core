use std::fmt::{Display, Formatter};

use code_gen::ToStaticStr;

use crate::tree::TypeTag;

/// A primitive type.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum PrimitiveType {
    /// An unsigned 8-bit integer.
    UnsignedInt8,

    /// An unsigned 16-bit integer.
    UnsignedInt16,

    /// An unsigned 32-bit integer.
    UnsignedInt32,

    /// An unsigned 64-bit integer.
    UnsignedInt64,
}

impl PrimitiveType {
    //! Conversions

    /// Converts the primitive type to a type tag.
    pub const fn to_type_tag(&self) -> TypeTag {
        TypeTag::Primitive(*self)
    }
}

impl ToStaticStr for PrimitiveType {
    fn to_static_str(&self) -> &'static str {
        match self {
            Self::UnsignedInt8 => "u8",
            Self::UnsignedInt16 => "u16",
            Self::UnsignedInt32 => "u32",
            Self::UnsignedInt64 => "u64",
        }
    }
}

impl Display for PrimitiveType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_static_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::PrimitiveType::*;
    use crate::tree::TypeTag;

    #[test]
    fn to_type_tag() {
        let result: TypeTag = UnsignedInt32.to_type_tag();
        let expected: TypeTag = TypeTag::Primitive(UnsignedInt32);
        assert_eq!(result, expected);
    }

    #[test]
    fn display() {
        assert_eq!(UnsignedInt8.to_string(), "u8");
        assert_eq!(UnsignedInt16.to_string(), "u16");
        assert_eq!(UnsignedInt32.to_string(), "u32");
        assert_eq!(UnsignedInt64.to_string(), "u64");
    }
}
