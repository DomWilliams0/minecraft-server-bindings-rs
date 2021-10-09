use crate::types::field::Field;
use crate::types::{NbtField, PacketError, PacketResult, VarIntField};
use async_std::io::prelude::*;
use async_trait::async_trait;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

pub struct VarIntThenByteArrayField {
    length: VarIntField,
    array: ByteArray,
}

pub struct PrefixedArrayField<C, T> {
    count: C,
    array: ByteArray,
    phantom: PhantomData<T>,
}

impl VarIntThenByteArrayField {
    pub fn new(buf: Vec<u8>) -> Self {
        Self {
            length: VarIntField::new(buf.len() as i32),
            array: ByteArray(buf),
        }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.array.0
    }
}

#[async_trait]
impl Field for VarIntThenByteArrayField {
    type Displayable = ByteArray;

    fn value(&self) -> &Self::Displayable {
        &self.array
    }

    fn size(&self) -> usize {
        self.length.size() + self.length.value() as usize
    }

    async fn read_field<R: Read + Unpin + Send>(r: &mut R) -> PacketResult<Self> {
        let length = VarIntField::read_field(r).await?;
        let mut array = vec![0u8; length.value() as usize];
        r.read_exact(&mut array).await.map_err(PacketError::Io)?;

        Ok(Self {
            length,
            array: ByteArray(array),
        })
    }

    async fn write_field<W: Write + Unpin + Send>(&self, w: &mut W) -> PacketResult<()> {
        self.length.write_field(w).await?;
        w.write_all(&self.array.0).await.map_err(PacketError::Io)?;
        Ok(())
    }
}

pub struct ByteArray(pub Vec<u8>);

impl Display for ByteArray {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x?}", self.0)
    }
}

pub struct RestOfPacketByteArrayField(ByteArray);

#[async_trait]
impl Field for RestOfPacketByteArrayField {
    type Displayable = ByteArray;

    fn value(&self) -> &Self::Displayable {
        &self.0
    }

    fn size(&self) -> usize {
        (self.0).0.len()
    }

    async fn read_field<R: Read + Unpin + Send>(r: &mut R) -> PacketResult<Self> {
        let mut vec = Vec::new();
        let n = r.read_to_end(&mut vec).await.map_err(PacketError::Io)?;
        debug_assert_ne!(n, 0);
        Ok(Self(ByteArray(vec)))
    }

    async fn write_field<W: Write + Unpin + Send>(&self, _w: &mut W) -> PacketResult<()> {
        unimplemented!()
    }
}

#[async_trait]
impl<C: Field + Send + Sync, T: Field + Send + Sync> Field for PrefixedArrayField<C, T> {
    type Displayable = ByteArray; // TODO do better

    fn value(&self) -> &Self::Displayable {
        &self.array
    }

    fn size(&self) -> usize {
        todo!()
    }

    async fn read_field<R: Read + Unpin + Send>(r: &mut R) -> PacketResult<Self> {
        todo!()
    }

    async fn write_field<W: Write + Unpin + Send>(&self, w: &mut W) -> PacketResult<()> {
        todo!()
    }
}
