use crate::types::field::{Field, FieldError, FieldResult};
use crate::types::VarIntField;
use async_std::io::prelude::*;
use async_trait::async_trait;
use std::fmt::{Display, Formatter};

pub struct VarIntThenByteArrayField {
    length: VarIntField,
    array: ByteArray,
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

    async fn read_field<R: Read + Unpin + Send>(r: &mut R) -> FieldResult<Self> {
        let length = VarIntField::read_field(r).await?;
        let mut array = vec![0u8; length.value() as usize];
        r.read_exact(&mut array).await.map_err(FieldError::Io)?;

        Ok(Self {
            length,
            array: ByteArray(array),
        })
    }

    async fn write_field<W: Write + Unpin + Send>(&self, w: &mut W) -> FieldResult<()> {
        self.length.write_field(w).await?;
        w.write_all(&self.array.0).await.map_err(FieldError::Io)?;
        Ok(())
    }
}

pub struct ByteArray(pub Vec<u8>);

impl Display for ByteArray {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
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

    async fn read_field<R: Read + Unpin + Send>(r: &mut R) -> FieldResult<Self> {
        let mut vec = Vec::new();
        let n = r.read_to_end(&mut vec).await.map_err(FieldError::Io)?;
        debug_assert_ne!(n, 0);
        Ok(Self(ByteArray(vec)))
    }

    async fn write_field<W: Write + Unpin + Send>(&self, _w: &mut W) -> FieldResult<()> {
        unimplemented!()
    }
}