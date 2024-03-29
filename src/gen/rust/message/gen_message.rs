use code_gen::rust::Source;
use code_gen::WithStatements;

use crate::gen::rust::{
    GenMessageDecode, GenMessageEncode, GenMessageField, GenMessageStruct, Naming, Typing,
};
use crate::gen::GenError;
use crate::tree::Message;

/// Responsible for generating code for message types.
#[derive(Clone, Debug, Default)]
pub struct GenMessage {
    naming: Naming,
    typing: Typing,
}

impl GenMessage {
    //! Gen

    /// Generates the source code for the message.
    pub fn gen(&self, message: &Message) -> Result<Source, GenError> {
        let mut source: Source = Source::default();

        let gen: GenMessageStruct = GenMessageStruct::new(&self.naming, &self.typing);
        source.add_statement(gen.gen_struct(message)?);

        let gen: GenMessageField = GenMessageField::new(&self.naming, &self.typing);
        for field in message.fields() {
            source.add_statement(gen.gen_field(message, field)?);
        }

        let gen: GenMessageEncode = GenMessageEncode::new(&self.naming, &self.typing);
        source.add_statement(gen.gen_impl_encoded_len(message)?);
        source.add_statement(gen.gen_impl_encode_to_slice(message)?);
        source.add_statement(gen.gen_impl_encode_to_write(message)?);

        let gen: GenMessageDecode = GenMessageDecode::new(&self.naming, &self.typing);
        source.add_statement(gen.gen_impl_decode_from_read_length_prefixed(message)?);
        source.add_statement(gen.gen_impl_decode_from_read(message)?);

        Ok(source)
    }
}
