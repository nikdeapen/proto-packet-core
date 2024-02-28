use std::fmt::{Display, Formatter};

use crate::tree::PrimitiveType;

/// A type tag.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum TypeTag {
    /// A primitive type.
    Primitive(PrimitiveType),
}

impl From<PrimitiveType> for TypeTag {
    fn from(primitive: PrimitiveType) -> Self {
        Self::Primitive(primitive)
    }
}

impl Display for TypeTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Primitive(primitive) => write!(f, "{}", primitive),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::PrimitiveType::UnsignedInt8;
    use crate::tree::TypeTag;

    #[test]
    fn display() {
        let tag: TypeTag = UnsignedInt8.into();
        assert_eq!(tag.to_string(), "u8");
    }
}
