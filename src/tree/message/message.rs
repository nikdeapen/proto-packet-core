use std::fmt::{Display, Formatter};

use code_gen::WithName;

use crate::tree::{MessageField, WithComments};

/// A message declaration.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Message {
    comments: Vec<String>,
    name: String,
    fields: Vec<MessageField>,
}

impl<S: Into<String>> From<S> for Message {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            name: name.into(),
            fields: Vec::default(),
        }
    }
}

impl WithComments for Message {
    fn comments(&self) -> &[String] {
        self.comments.as_slice()
    }

    fn add_comment<S>(&mut self, comment: S)
    where
        S: Into<String>,
    {
        self.comments.push(comment.into());
    }
}

impl WithName for Message {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl Message {
    //! Fields

    /// Gets the fields.
    pub fn fields(&self) -> &[MessageField] {
        self.fields.as_slice()
    }

    /// Adds the field.
    pub fn with_field<F>(mut self, field: F) -> Self
    where
        F: Into<MessageField>,
    {
        self.add_field(field);
        self
    }

    /// Adds the field.
    pub fn add_field<F>(&mut self, field: F)
    where
        F: Into<MessageField>,
    {
        self.fields.push(field.into());
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for comment in &self.comments {
            write!(f, "// {}\n", comment)?;
        }
        write!(f, "message {} {{", self.name)?;
        if self.fields.is_empty() {
            write!(f, "}}")?;
        } else {
            write!(f, "\n")?;
            for field in &self.fields {
                write!(f, "    {}\n", field)?;
            }
            write!(f, "}}")?;
        }
        write!(f, "\n")
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::Message;
    use crate::tree::PrimitiveType::{UnsignedInt16, UnsignedInt8};

    #[test]
    fn display() {
        let message: Message = "MyMessage".into();
        let result: String = message.to_string();
        let expected: &str = "message MyMessage {}\n";
        assert_eq!(result, expected);

        let message: Message = message.with_field(("one", UnsignedInt8)).into();
        let result: String = message.to_string();
        let expected: &str = "message MyMessage {\n    one: u8;\n}\n";
        assert_eq!(result, expected);

        let message: Message = message.with_field(("two", UnsignedInt16)).into();
        let result: String = message.to_string();
        let expected: &str = "message MyMessage {\n    one: u8;\n    two: u16;\n}\n";
        assert_eq!(result, expected);
    }
}
