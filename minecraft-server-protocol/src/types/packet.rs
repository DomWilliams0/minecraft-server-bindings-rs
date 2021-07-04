use crate::types::*;
use async_std::io;
use async_std::io::Cursor;
use async_trait::async_trait;
use displaydoc::Display;
use std::string::FromUtf8Error;
use thiserror::Error;

pub type PacketId = i32;

pub type PacketResult<T> = Result<T, PacketError>;

#[derive(Debug, Error, Display)]
pub enum PacketError {
    /// IO error: {0}
    Io(#[from] io::Error),

    // used in macros
    /// Expected packet ID {expected:#x} but got {actual:#x}
    UnexpectedPacket {
        expected: PacketId,
        actual: PacketId,
    },

    // used in macros
    /// Failed to read packet of length {length}, read {read} bytes
    FullPacketNotRead { length: usize, read: usize },

    /// Invalid unicode string: {0}
    BadString(#[from] FromUtf8Error),

    /// Varint is longer than the max of 5 bytes (got {0} bytes)
    BadVarInt(usize),

    /// Bad bool value, must be 0 or 1 (got {0})
    BadBool(u8),
}

pub struct PacketBody {
    pub id: PacketId,
    pub body: Vec<u8>,
}

#[async_trait]
pub trait ClientBound: Send + Sync {
    async fn write_packet(&self, w: &mut Cursor<&mut [u8]>) -> PacketResult<()>;

    fn length(&self) -> usize;

    fn full_size(&self) -> usize {
        let len = VarIntField::new(self.length() as i32);
        len.value() as usize + len.size()
    }
}

#[async_trait]
pub trait ServerBound: Sized + Send + Sync {
    // TODO make this sync and block on reading
    async fn read_packet(body: PacketBody) -> PacketResult<Self>;
}
