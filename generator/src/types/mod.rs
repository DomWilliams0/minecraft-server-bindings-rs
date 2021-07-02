pub use array::{RestOfPacketByteArrayField, VarIntThenByteArrayField};
pub use position::PositionField;
pub use primitive::{
    BoolField, ByteField, DoubleField, FloatField, IntField, LongField, ShortField, UByteField,
    UShortField,
};
pub use string::{ChatField, IdentifierField, StringField};
pub use varint::VarIntField;

mod field;

mod array;
mod position;
mod primitive;
mod string;
mod varint;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::field::Field;
    use async_std::io::Cursor;

    #[test]
    fn sizes() {
        async_std::task::block_on(async {
            let mut cursor = Cursor::new(vec![0u8, 100]);

            let s = "cor blimey";

            let a = StringField::new(s.to_owned());
            let b = UShortField::new(10);
            let c = VarIntField::new(150);

            let expected_len = 1 + s.len() + 2 + 2;
            assert_eq!(expected_len, a.size() + b.size() + c.size());

            a.write_field(&mut cursor).await.unwrap();
            b.write_field(&mut cursor).await.unwrap();
            c.write_field(&mut cursor).await.unwrap();
            assert_eq!(cursor.position() as usize, expected_len);
        });
    }
}
