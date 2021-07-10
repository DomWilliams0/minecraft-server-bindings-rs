use crate::types::{Field, PacketResult};
use async_std::io::prelude::*;
use async_std::io::Cursor;
use async_trait::async_trait;
use mutf8::mstr;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

pub struct NbtField {
    bytes: Vec<u8>,
}

#[derive(Default)]
pub struct NbtBuilder(Cursor<Vec<u8>>);
pub struct NbtCompoundBuilder(NbtBuilder);
pub struct NbtTagBuilder<'a>(NbtCompoundBuilder, Option<&'a str>);

#[repr(u8)]
#[derive(Copy, Clone)]
enum Tag {
    End = 0,
    Byte = 1,
    Short = 2,
    Int = 3,
    Long = 4,
    Float = 5,
    Double = 6,
    ByteArray = 7,
    String = 8,
    List = 9,
    Compound = 10,
    IntArray = 11,
    LongArray = 12,
}

macro_rules! write_primitives {
    ($($int:ty)*) => {
        paste::paste! {
            $(
                /// No tag
                async fn [< write_ $int >](&mut self, int: $int) -> PacketResult<()> {
                    let bytes = int.to_be_bytes();
                    self.0.write_all(&bytes).await?;
                    Ok(())
                }
            )*
        }
    };
}

macro_rules! nbt_primitive {
    ($tag:expr, $name:ident, $int:ty) => {
        paste::paste! {
            pub async fn $name(mut self, val: $int) -> PacketResult<NbtCompoundBuilder> {
                self.write_tag($tag).await?;
                (self.0).0.[< write_ $int >](val).await?;
                Ok(self.0)
            }
        }
    };
}

impl NbtField {
    pub fn builder() -> NbtBuilder {
        NbtBuilder::default()
    }
}

impl NbtBuilder {
    pub fn new(storage: Vec<u8>) -> Self {
        Self(Cursor::new(storage))
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.get_ref()
    }

    pub fn into_field(self) -> NbtField {
        NbtField {
            bytes: self.0.into_inner(),
        }
    }

    async fn write_tag_raw(&mut self, tag: Tag) -> PacketResult<()> {
        let b = tag as u8;
        self.0.write_all(&[b]).await?;
        Ok(())
    }

    /// No tag
    async fn write_string_raw(&mut self, str: &str) -> PacketResult<()> {
        let len = (str.len() as u16).to_be_bytes();
        self.0.write_all(&len).await?;

        let mstring = mstr::from_utf8(str.as_bytes()).unwrap(); // str is utf8 always
        self.0.write_all(mstring.as_bytes()).await?;
        Ok(())
    }

    write_primitives!(i64 i32 i16 i8 f32 f64);

    pub async fn begin_named_compound(mut self, name: &str) -> PacketResult<NbtCompoundBuilder> {
        self.write_tag_raw(Tag::Compound).await?;
        self.write_string_raw(name).await?;
        Ok(NbtCompoundBuilder(self))
    }

    pub async fn begin_unnamed_compound(mut self) -> PacketResult<NbtCompoundBuilder> {
        self.write_tag_raw(Tag::Compound).await?;
        Ok(NbtCompoundBuilder(self))
    }
}
impl NbtCompoundBuilder {
    pub async fn begin_named_tag(self, name: &str) -> PacketResult<NbtTagBuilder<'_>> {
        Ok(NbtTagBuilder(self, Some(name)))
    }

    pub async fn begin_unnamed_tag<'a>(self) -> PacketResult<NbtTagBuilder<'a>> {
        Ok(NbtTagBuilder(self, None))
    }

    pub async fn end(mut self) -> PacketResult<NbtBuilder> {
        self.0.write_tag_raw(Tag::End).await?;
        Ok(self.0)
    }
}

impl NbtTagBuilder<'_> {
    async fn write_tag(&mut self, tag: Tag) -> PacketResult<()> {
        (self.0).0.write_tag_raw(tag).await?;
        if let Some(name) = self.1 {
            (self.0).0.write_string_raw(name).await?;
        }

        Ok(())
    }

    pub async fn long_array(mut self, val: &[i64]) -> PacketResult<NbtCompoundBuilder> {
        self.write_tag(Tag::LongArray).await?;

        let len = i32::try_from(val.len()).expect("array is too long");
        (self.0).0.write_i32(len).await?;

        for val in val {
            let bytes = val.to_be_bytes();
            ((self.0).0).0.write_all(&bytes).await?;
        }

        Ok(self.0)
    }

    pub async fn string(mut self, val: &str) -> PacketResult<NbtCompoundBuilder> {
        self.write_tag(Tag::String).await?;
        (self.0).0.write_string_raw(val).await?;
        Ok(self.0)
    }

    nbt_primitive!(Tag::Long, long, i64);
    nbt_primitive!(Tag::Int, int, i32);
    nbt_primitive!(Tag::Short, short, i16);
    nbt_primitive!(Tag::Byte, byte, i8);
    nbt_primitive!(Tag::Double, double, f64);
    nbt_primitive!(Tag::Float, float, f32);
}

impl Display for NbtField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x?}", self.bytes)
    }
}

#[async_trait]
impl Field for NbtField {
    type Displayable = Self;

    fn value(&self) -> &Self::Displayable {
        self
    }

    fn size(&self) -> usize {
        self.bytes.len()
    }

    async fn read_field<R: Read + Unpin + Send>(r: &mut R) -> PacketResult<Self> {
        todo!("nbt parsing")
    }

    async fn write_field<W: Write + Unpin + Send>(&self, w: &mut W) -> PacketResult<()> {
        w.write_all(&self.bytes).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_await_test::async_test;

    #[async_test]
    async fn nbt_hello_world() -> PacketResult<()> {
        let nbt = NbtBuilder::default()
            .begin_named_compound("hello world")
            .await?
            .begin_named_tag("name")
            .await?
            .string("Bananrama")
            .await?
            .end()
            .await?;

        assert_eq!(
            nbt.as_bytes(),
            b"\x0a\x00\x0b\x68\x65\x6c\x6c\x6f\x20\x77\x6f\x72\x6c\x64\x08\x00\x04\x6e\x61\x6d\x65\
            \x00\x09\x42\x61\x6e\x61\x6e\x72\x61\x6d\x61\x00"
        );
        Ok(())
    }
}
