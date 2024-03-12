use crate::tree::Message;

/// A source file declaration.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum SourceDec {
    /// A message declaration.
    MessageDec(Message),
}

impl From<Message> for SourceDec {
    fn from(message: Message) -> Self {
        Self::MessageDec(message)
    }
}
