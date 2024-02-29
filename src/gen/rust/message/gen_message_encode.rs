use code_gen::rust::{
    Function, ImplBlock, PrimitiveType as RustPrimitive, Receiver, Signature, WithFunctions,
    WithReceiver, WithResult,
};
use code_gen::{Literal, Semi, WithName, WithStatements};

use crate::gen::rust::{Naming, Typing};
use crate::gen::GenError;
use crate::tree::{Message, MessageField, PrimitiveType, TypeTag, WithTypeTag};

/// Responsible for generating struct impl blocks for message fields.
#[derive(Copy, Clone, Debug)]
pub struct GenMessageEncode<'a> {
    naming: &'a Naming,
    _typing: &'a Typing,
}

impl<'a> GenMessageEncode<'a> {
    //! Construction

    /// Creates a new gen message field.
    pub const fn new(naming: &'a Naming, typing: &'a Typing) -> Self {
        Self {
            naming,
            _typing: typing,
        }
    }
}

impl<'a> GenMessageEncode<'a> {
    //! EncodedLength

    /// Generates the impl block for implementing `EncodedLen`.
    pub fn gen_impl_encoded_len(&self, message: &Message) -> Result<ImplBlock, GenError> {
        let mut block: ImplBlock = self.naming.type_name(message.name())?.into();
        block.set_for_trait("EncodedLen");

        let signature: Signature = Signature::from("encoded_len")
            .with_receiver(Receiver::Borrowed)
            .with_result(RustPrimitive::UnsignedIntSize);
        let mut function: Function = Function::from(signature);
        function.add_statement(Semi::from("let mut encoded_len: usize = 0"));

        for field in message.fields() {
            function.add_statement(self.gen_encoded_len_statement(field)?);
        }

        function.add_expression_statement(Literal::from("encoded_len"));
        block.add_function(function);

        Ok(block)
    }

    fn gen_encoded_len_statement(&self, field: &MessageField) -> Result<Semi<Literal>, GenError> {
        if let Some(field_number) = field.field_number() {
            let field_exp: String = match field.type_tag() {
                TypeTag::Primitive(primitive) => match primitive {
                    PrimitiveType::UnsignedInt8 => {
                        Self::field_exp_int(false, 8, None, field_number)
                    }
                    PrimitiveType::UnsignedInt16 => {
                        // todo -- supported fixed fields
                        Self::field_exp_int(false, 16, Some(false), field_number)
                    }
                    PrimitiveType::UnsignedInt32 => {
                        // todo -- supported fixed fields
                        Self::field_exp_int(false, 32, Some(false), field_number)
                    }
                    PrimitiveType::UnsignedInt64 => {
                        // todo -- supported fixed fields
                        Self::field_exp_int(false, 64, Some(false), field_number)
                    }
                },
            };
            // todo -- remove literal
            Ok(Semi::from(Literal::from(format!(
                "encoded_len += {}.encoded_len()",
                field_exp
            ))))
        } else {
            unimplemented!("required fields not yet supported")
        }
    }

    /// Gets the field constructor expression string.
    fn field_exp_int(signed: bool, bits: u32, fixed: Option<bool>, field_number: u32) -> String {
        let signed: &str = if signed { "Signed" } else { "Unsigned" };
        if let Some(fixed) = fixed {
            format!(
                "{}Int{}Field::new({}, {}, self.value)",
                signed, bits, field_number, fixed
            )
        } else {
            format!(
                "{}Int{}Field::new({}, self.value)",
                signed, bits, field_number
            )
        }
    }
}
