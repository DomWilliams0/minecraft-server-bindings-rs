use async_std::io::prelude::*;
use async_trait::async_trait;
use displaydoc::Display;
use std::fmt::Display;
use std::fmt::Formatter;
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Debug, Display, Error)]
pub enum FieldError {
    /// IO error: {0}
    Io(#[from] async_std::io::Error),

    /// Varint is longer than the max of 5 bytes (got {0} bytes)
    BadVarInt(usize),

    /// Bad bool value, must be 0 or 1 (got {0})
    BadBool(u8),

    /// Invalid unicode string: {0}
    BadString(#[from] FromUtf8Error),
}

pub type FieldResult<T> = Result<T, FieldError>;

#[async_trait]
pub trait Field: Sized {
    type Displayable: Display;
    fn value(&self) -> &Self::Displayable;

    fn size(&self) -> usize;
    async fn read_field<R: Read + Unpin + Send>(r: &mut R) -> FieldResult<Self>;
    async fn write_field<W: Write + Unpin + Send>(&self, w: &mut W) -> FieldResult<()>;
}

pub struct DisplayableField<'a, T: Display>(pub &'a T);

impl<'a, T: Display> Display for DisplayableField<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
