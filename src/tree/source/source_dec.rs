use crate::tree::Message;

/// A source file declaration.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum SourceDec {
    /// A message declaration.
    MessageDec(Message),
}
