use code_gen::rust::{
    gen_builder_copy, gen_getter_copy, gen_setter_copy, Function, ImplBlock, TypeTag as RustType,
    WithComments, WithFunctions,
};
use code_gen::WithName;

use crate::gen::rust::{Naming, Typing};
use crate::gen::GenError;
use crate::tree::{Message, MessageField, WithTypeTag};

/// Responsible for generating struct impl blocks for message fields.
#[derive(Copy, Clone, Debug)]
pub struct GenMessageField<'a> {
    naming: &'a Naming,
    typing: &'a Typing,
}

impl<'a> GenMessageField<'a> {
    //! Construction

    /// Creates a new gen message field.
    pub const fn new(naming: &'a Naming, typing: &'a Typing) -> Self {
        Self { naming, typing }
    }
}

impl<'a> GenMessageField<'a> {
    //! Gen

    /// Generates the impl block for the message field.
    pub fn gen_field(
        &self,
        message: &Message,
        field: &MessageField,
    ) -> Result<ImplBlock, GenError> {
        let mut block: ImplBlock = self.naming.type_name(message.name())?.into();
        block.add_comment(format!("Field: {}", field));

        self.gen_getters(&mut block, field)?;
        self.gen_setters(&mut block, field)?;
        self.gen_builders(&mut block, field)?;

        Ok(block)
    }
}

impl<'a> GenMessageField<'a> {
    //! Get

    /// Generates the getter functions for the field.
    fn gen_getters(&self, b: &mut ImplBlock, field: &MessageField) -> Result<(), GenError> {
        if self.typing.is_copy(field.type_tag())? {
            let name: String = self.naming.field_name(field.name())?;
            let tag: RustType = self.typing.field_type(field.type_tag())?.to_option();
            let function: Function = gen_getter_copy(name, tag)
                .with_comment(format!("Gets the field: `{}`.", field.name()));
            b.add_function(function);
        } else {
            unreachable!()
        }
        Ok(())
    }
}

impl<'a> GenMessageField<'a> {
    //! Set

    /// Generates the setter functions for the field.
    fn gen_setters(&self, b: &mut ImplBlock, field: &MessageField) -> Result<(), GenError> {
        if self.typing.is_copy(field.type_tag())? {
            let name: String = self.naming.field_name(field.name())?;
            let tag: RustType = self.typing.field_type(field.type_tag())?.to_option();
            let function: Function = gen_setter_copy(name, tag).with_comment(format!(
                "Sets the field: `{}`. Returns the previous value.",
                field.name()
            ));
            b.add_function(function);
        } else {
            unreachable!()
        }
        Ok(())
    }
}

impl<'a> GenMessageField<'a> {
    //! Build

    /// Generates the builder functions for the field.
    fn gen_builders(&self, b: &mut ImplBlock, field: &MessageField) -> Result<(), GenError> {
        if self.typing.is_copy(field.type_tag())? {
            let name: String = self.naming.field_name(field.name())?;
            let tag: RustType = self.typing.field_type(field.type_tag())?.to_option();
            let function: Function = gen_builder_copy(name, tag).with_comment(format!(
                "Builds the field: `{}`. Returns the struct itself.",
                field.name()
            ));
            b.add_function(function);
        } else {
            unreachable!()
        }
        Ok(())
    }
}
