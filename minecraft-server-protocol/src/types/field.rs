use crate::types::PacketResult;
use async_std::io::prelude::*;
use async_trait::async_trait;
use std::fmt::Display;

#[async_trait]
pub trait Field: Sized {
    type Displayable: Display;
    fn value(&self) -> &Self::Displayable;

    /// Used to calculate total packet size
    fn size(&self) -> usize;
    async fn read_field<R: Read + Unpin + Send>(r: &mut R) -> PacketResult<Self>;
    async fn write_field<W: Write + Unpin + Send>(&self, w: &mut W) -> PacketResult<()>;
}
