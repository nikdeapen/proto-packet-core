use crate::gen::GenError;

/// Responsible for naming things.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct Naming {
    _nothing: (),
}

impl Naming {
    //! Field Names

    /// Gets the field name for the declared field name.
    pub fn field_name<S>(&self, declared_name: S) -> Result<String, GenError>
    where
        S: Into<String>,
    {
        Ok(declared_name.into())
    }
}

impl Naming {
    //! Type Names

    /// Gets the type name for the declared type name.
    pub fn type_name<S>(&self, declared_name: S) -> Result<String, GenError>
    where
        S: Into<String>,
    {
        Ok(declared_name.into())
    }
}
