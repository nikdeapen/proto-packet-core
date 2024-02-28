use crate::tree::Var;

/// An element with a variable.
pub trait WithVar {
    /// Gets the variable.
    fn var(&self) -> &Var;
}
