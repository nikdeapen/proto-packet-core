use code_gen::rust::Source;
use code_gen::{CodeBuffer, Statement};

use crate::gen::rust::GenMessage;
use crate::gen::GenError;
use crate::tree::PrimitiveType::*;
use crate::tree::{Message, MessageField, WithComments};

#[test]
fn unsigned_ints() -> Result<(), GenError> {
    let mut message: Message = "UnsignedInts".into();

    message.add_comment("A message with unsigned integer fields.");

    message.add_field(
        MessageField::from(("one", UnsignedInt8))
            .with_field_number(1)
            .with_comment("The first field."),
    );
    message.add_field(
        MessageField::from(("two", UnsignedInt16))
            .with_field_number(2)
            .with_comment("The second field."),
    );
    message.add_field(
        MessageField::from(("three", UnsignedInt32))
            .with_field_number(3)
            .with_comment("The third field."),
    );
    message.add_field(
        MessageField::from(("four", UnsignedInt64))
            .with_field_number(4)
            .with_comment("The fourth field."),
    );

    let source: Source = GenMessage::default().gen(&message)?;

    let mut b: CodeBuffer = CodeBuffer::new("    ", "\n", 4 * 1024);
    source.write(&mut b, 0);
    let result: String = b.export();
    let result: String = result
        .split("\n")
        .map(|s| s.trim_end())
        .collect::<Vec<&str>>()
        .join("\n");

    let expected: &str = include_str!("unsigned_ints.txt");

    assert_eq!(result, expected);

    Ok(())
}
