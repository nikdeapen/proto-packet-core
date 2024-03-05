use crate::tree::source::source_dec::SourceDec;

/// A source file.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct SourceFile {
    declarations: Vec<SourceDec>,
}

impl SourceFile {
    //! Declarations

    /// Gets the declarations.
    pub fn declarations(&self) -> &[SourceDec] {
        self.declarations.as_slice()
    }

    /// Adds a declaration.
    pub fn add_declaration<D>(&mut self, declaration: D)
    where
        D: Into<SourceDec>,
    {
        self.declarations.push(declaration.into());
    }

    /// Adds a declaration.
    pub fn with_declaration<D>(mut self, declaration: D) -> Self
    where
        D: Into<SourceDec>,
    {
        self.add_declaration(declaration);
        self
    }
}
