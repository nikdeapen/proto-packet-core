use code_gen::rust::{
    Function, ImplBlock, Match, MatchCase, Reference, Signature, TypeTag as RustType, WhileLet,
    WithFnGenerics, WithFunctions, WithResult, WithVarParams,
};
use code_gen::{EmptyLine, Literal, Semi, WithName, WithStatements};

use crate::gen::rust::{Naming, Typing};
use crate::gen::GenError;
use crate::tree::PrimitiveType::*;
use crate::tree::TypeTag::Primitive;
use crate::tree::{Message, WithTypeTag};

/// Responsible for generating struct impl blocks for message decoding.
#[derive(Copy, Clone, Debug)]
pub struct GenMessageDecode<'a> {
    naming: &'a Naming,
    _typing: &'a Typing,
}

impl<'a> GenMessageDecode<'a> {
    //! Construction

    /// Creates a new gen message decode.
    pub const fn new(naming: &'a Naming, typing: &'a Typing) -> Self {
        Self {
            naming,
            _typing: typing,
        }
    }
}

impl<'a> GenMessageDecode<'a> {
    //! DecodeFromReadLengthPrefixed

    /// Generates the impl block for implementing the `DecodeFromReadLengthPrefixed` trait.
    pub fn gen_impl_decode_from_read_length_prefixed(
        &self,
        message: &Message,
    ) -> Result<ImplBlock, GenError> {
        let mut block: ImplBlock = self.naming.type_name(message.name())?.into();
        block.set_for_trait("DecodeFromReadLengthPrefixed");
        Ok(block)
    }
}

impl<'a> GenMessageDecode<'a> {
    //! DecodeFromRead

    /// Generates the impl block for implementing the `DecodeFromRead` trait.
    pub fn gen_impl_decode_from_read(&self, message: &Message) -> Result<ImplBlock, GenError> {
        let mut block: ImplBlock = self.naming.type_name(message.name())?.into();
        block.set_for_trait("DecodeFromRead");
        block.add_function(self.gen_fn(message)?);
        Ok(block)
    }

    fn gen_fn(&self, message: &Message) -> Result<Function, GenError> {
        let result_type: RustType = RustType::from("Result")
            .with_generic("Self")
            .with_generic("io::Error");
        let signature: Signature = Signature::from("decode_from_read")
            .with_generic(("R", "io::Read"))
            .with_param(("r", RustType::from("R").to_reference(Reference::MUT)))
            .with_result(result_type);
        let mut function: Function = Function::from(signature);
        self.gen_decode_from_read_statements(message, &mut function)?;
        Ok(function)
    }

    /// Generates the statements in the `decode_from_read` function.
    fn gen_decode_from_read_statements(
        &self,
        message: &Message,
        function: &mut Function,
    ) -> Result<(), GenError> {
        function.add_semi("let len: usize = VarIntSize::decode_from_read_prefix(r)?.value");
        function.add_semi("let mut r: ReadLimit<R> = ReadLimit { read: r, limit: len }");
        function.add_statement(EmptyLine::default());
        function.add_semi("let result: Self = Self::default()");
        function.add_semi("let mut field_number: u32 = 0");

        let while_let_exp: Literal =
            Literal::from("FieldHeader::decode_from_read_prefix_optional(&mut r)?");
        let mut while_let: WhileLet = WhileLet::new("Some", "header", while_let_exp);
        self.gen_while_let_decode_statements(message, &mut while_let)?;
        function.add_statement(while_let);

        function.add_literal("Ok(result)");
        Ok(())
    }

    /// Generates the statements in the while-let decoding loop.
    fn gen_while_let_decode_statements(
        &self,
        message: &Message,
        while_let: &mut WhileLet,
    ) -> Result<(), GenError> {
        while_let.add_semi("field_number += header.field_number()");
        let mut match_statement: Match = Match::from(Literal::from("field_number"));
        self.gen_decode_match_cases(message, &mut match_statement)?;
        while_let.add_statement(match_statement);
        Ok(())
    }

    /// Generates the match cases for the decoding match statement.
    fn gen_decode_match_cases(
        &self,
        message: &Message,
        match_statement: &mut Match,
    ) -> Result<(), GenError> {
        for field in message.fields() {
            if let Some(field_number) = field.field_number() {
                let mut match_case: MatchCase = MatchCase::from(format!("{}", field_number));
                match field.type_tag() {
                    Primitive(primitive) => {
                        let read_fn_name: &str = match primitive {
                            UnsignedInt8 => "u8",
                            UnsignedInt16 => "u16_var",
                            UnsignedInt32 => "u32_var",
                            UnsignedInt64 => "u64_var",
                        };
                        let field_name = self.naming.field_name(field.name())?;
                        let result_set: String = format!(
                            "result.set_{}(Some(read::read_{}(&mut r)?))",
                            field_name, read_fn_name
                        );
                        match_case.add_statement(Semi::from(result_set));
                    }
                }
                match_statement.add_match_case(match_case);
            } else {
                unimplemented!("required fields not yet supported")
            }
        }
        let match_case: MatchCase =
            MatchCase::from("_").with_semi("unimplemented!(\"unknown fields not yet supported\")");
        match_statement.add_match_case(match_case);
        Ok(())
    }
}
