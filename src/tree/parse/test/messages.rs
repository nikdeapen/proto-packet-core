use crate::tree::PrimitiveType::*;
use crate::tree::{parse, Message, MessageField, SourceDec, SourceFile};

#[test]
fn messages() -> Result<(), ()> {
    let s: &str = include_str!("messages.txt");
    let result: SourceFile = parse::source_file(s)?;

    let f1: MessageField = MessageField::from(("one", UnsignedInt8)).with_field_number(1);
    let f2: MessageField = MessageField::from(("two", UnsignedInt16)).with_field_number(2);
    let f3: MessageField = MessageField::from(("three", UnsignedInt32)).with_field_number(3);
    let f4: MessageField = MessageField::from(("four", UnsignedInt64)).with_field_number(4);
    let message: Message = Message::from("UnsignedInts")
        .with_field(f1)
        .with_field(f2)
        .with_field(f3)
        .with_field(f4);
    let expected: SourceFile = SourceFile::default().with_declaration(SourceDec::from(message));
    assert_eq!(result, expected);

    Ok(())
}
