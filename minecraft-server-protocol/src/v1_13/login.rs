// autogenerated file
#![allow(unused_imports)]

use crate::types::*;
use async_std::io::Cursor;
use minecraft_server_protocol_derive::{ClientBoundPacket, ServerBoundPacket};
use std::fmt::{Display, Formatter};

pub mod client {
    use super::*;

    #[derive(ClientBoundPacket)]
    #[packet_id = 0x00]
    pub struct Disconnect {
        pub reason: StringField,
    }

    #[derive(ClientBoundPacket)]
    #[packet_id = 0x01]
    pub struct EncryptionBegin {
        pub server_id: StringField,
        pub public_key: VarIntThenByteArrayField,
        pub verify_token: VarIntThenByteArrayField,
    }

    #[derive(ClientBoundPacket)]
    #[packet_id = 0x02]
    pub struct Success {
        pub uuid: StringField,
        pub username: StringField,
    }

    #[derive(ClientBoundPacket)]
    #[packet_id = 0x03]
    pub struct Compress {
        pub threshold: VarIntField,
    }

    #[derive(ClientBoundPacket)]
    #[packet_id = 0x04]
    pub struct LoginPluginRequest {
        pub message_id: VarIntField,
        pub channel: StringField,
        pub data: RestOfPacketByteArrayField,
    }
}

pub mod server {
    use super::*;

    #[derive(ServerBoundPacket)]
    #[packet_id = 0x00]
    pub struct LoginStart {
        pub username: StringField,
    }

    #[derive(ServerBoundPacket)]
    #[packet_id = 0x01]
    pub struct EncryptionBegin {
        pub shared_secret: VarIntThenByteArrayField,
        pub verify_token: VarIntThenByteArrayField,
    }

    /* TODO incomplete struct LoginPluginResponse
        #[derive(ServerBoundPacket)]
        #[packet_id = 0x02]
        pub struct LoginPluginResponse {
            pub message_id: VarIntField,
            // TODO pub data: Option(RestOfBuffer),
        }
    */
}
