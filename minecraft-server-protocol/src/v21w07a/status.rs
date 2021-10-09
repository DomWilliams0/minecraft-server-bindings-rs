// autogenerated file
#![allow(unused_imports)]

use crate::types::*;
use async_std::io::{prelude::*, Cursor};
use async_trait::async_trait;
use minecraft_server_protocol_derive::{ClientBoundPacket, ServerBoundPacket};
use std::fmt::{Display, Formatter};

pub mod client {
    use super::*;

    #[derive(ClientBoundPacket)]
    #[packet_id = 0x00]
    pub struct ServerInfo {
        pub response: StringField,
    }

    #[derive(ClientBoundPacket)]
    #[packet_id = 0x01]
    pub struct Ping {
        pub time: LongField,
    }
}

pub mod server {
    use super::*;

    #[derive(ServerBoundPacket)]
    #[packet_id = 0x00]
    pub struct PingStart {}

    #[derive(ServerBoundPacket)]
    #[packet_id = 0x01]
    pub struct Ping {
        pub time: LongField,
    }
}
