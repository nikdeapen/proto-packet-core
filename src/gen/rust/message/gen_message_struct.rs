use code_gen::rust::Access::Public;
use code_gen::rust::{
    Struct, StructField, TypeTag as RustType, WithAccess, WithComments as WithRustComments,
    WithDerives, WithStructFields,
};
use code_gen::WithName;

use crate::gen::rust::{Naming, Typing};
use crate::gen::GenError;
use crate::tree::{Message, MessageField, WithComments, WithTypeTag};

/// Responsible for generating struct declarations for message types.
#[derive(Copy, Clone, Debug)]
pub struct GenMessageStruct<'a> {
    naming: &'a Naming,
    typing: &'a Typing,
}

impl<'a> GenMessageStruct<'a> {
    //! Construction

    /// Creates a new gen message struct.
    pub const fn new(naming: &'a Naming, typing: &'a Typing) -> Self {
        Self { naming, typing }
    }
}

impl<'a> GenMessageStruct<'a> {
    //! Gen

    /// Generates the struct declaration for the message.
    pub fn gen_struct(&self, message: &Message) -> Result<Struct, GenError> {
        let mut s: Struct = self.naming.type_name(message.name())?.into();
        s.set_access(Public);

        self.gen_comments(&mut s, message)?;
        self.gen_derives(&mut s, message)?;

        for field in message.fields() {
            self.gen_field(&mut s, field)?;
        }

        Ok(s)
    }

    /// Generates the comments for the struct.
    fn gen_comments(&self, s: &mut Struct, message: &Message) -> Result<(), GenError> {
        for comment in message.comments() {
            s.add_comment(format!("// {}", comment.as_str()));
        }

        if message.fields().is_empty() {
            s.add_comment(format!("message {} {{}}", message.name()));
        } else {
            s.add_comment(format!("message {} {{", message.name()));
            for field in message.fields() {
                s.add_comment("");
                for comment in field.comments() {
                    s.add_comment(format!("    // {}", comment));
                }
                s.add_comment(format!("    {}", field));
            }
            s.add_comment("}");
        }

        Ok(())
    }

    /// Generates the derives for the struct.
    fn gen_derives(&self, s: &mut Struct, message: &Message) -> Result<(), GenError> {
        if self.typing.all_copy(message)? {
            s.add_derive("Copy");
        }

        s.add_derive("Clone");
        s.add_derive("Ord");
        s.add_derive("PartialOrd");
        s.add_derive("Eq");
        s.add_derive("PartialEq");
        s.add_derive("Hash");
        s.add_derive("Debug");

        Ok(())
    }

    /// Generates the code for the field.
    fn gen_field(&self, s: &mut Struct, field: &MessageField) -> Result<(), GenError> {
        let name: String = self.naming.field_name(field.name())?;
        let tag: RustType = self.typing.field_type(field.type_tag())?.to_option();
        let field: StructField = (name, tag).into();
        s.add_field(field);
        Ok(())
    }
}
