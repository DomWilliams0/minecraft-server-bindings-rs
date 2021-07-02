use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::io::Read;
use std::str::FromStr;

use displaydoc::Display;
use serde::*;
use serde_json::Value;
use thiserror::Error;

use crate::schema::raw::PacketDefinition;

#[derive(Debug, Display, Error)]
pub enum SchemaError {
    /// IO error: {0}
    Io(#[from] std::io::Error),

    /// Invalid packet ID {0:#x}
    InvalidPacketId(i32),

    /// Missing 'packet' key
    MissingPacketKey,

    /// Bad structure: {0}
    BadStructure(&'static str),

    /// Failed to deserialize {1}: {0}
    Deserializing(#[source] serde_json::Error, &'static str),

    /// Duplicate object '{0}'
    Duplicate(&'static str),

    /// Unknown mapper type '{0}'
    UnknownMapper(String),

    /// Unknown packet definition '{0}'
    UnknownDefinition(String),

    /// Missing switch or mapper definition for packets
    BadPacketDefinition,

    /// Unknown field type '{0}'
    UnknownFieldType(String),

    /// Missing key '{0}' in switch {1}
    MissingSwitchKey(String, &'static str),
}

pub type SchemaResult<T> = Result<T, SchemaError>;

mod raw {
    #![allow(non_snake_case)]

    use super::*;

    #[derive(Deserialize)]
    pub struct ProtocolRoot {
        pub types: BTreeMap<String, Value>,

        pub handshaking: ProtocolState,
        pub status: ProtocolState,
        pub login: ProtocolState,
        pub play: ProtocolState,
    }

    #[derive(Deserialize, Debug)]
    pub struct ProtocolState {
        pub toClient: ProtocolUnidirectional,
        pub toServer: ProtocolUnidirectional,
    }

    #[derive(Deserialize, Debug)]
    pub struct ProtocolUnidirectional {
        pub types: PacketTypes,
    }

    #[derive(Deserialize, Debug)]
    pub struct PacketTypes(pub BTreeMap<String, Value>);

    #[derive(Deserialize, Debug)]
    pub struct PacketDefinition<'a> {
        pub name: &'a str,
        pub r#type: Value,
    }

    #[derive(Deserialize, Debug)]
    pub struct Switch {
        #[serde(rename = "compareTo")]
        pub compare_to: String,
        pub fields: BTreeMap<String, Value>,
    }

    #[derive(Deserialize, Debug)]
    pub struct Mapper<'a> {
        pub r#type: &'a str,
        pub mappings: BTreeMap<&'a str, Value>,
    }
    #[derive(Deserialize, Debug)]
    pub struct Field<'a> {
        pub name: Option<&'a str>,
        #[serde(default)]
        pub anon: bool,
        pub r#type: Value,
    }

    #[derive(Deserialize, Debug)]
    pub struct BufferType<'a> {
        pub countType: &'a str,
    }

    #[derive(Deserialize, Debug)]
    pub struct ArrayType<'a> {
        pub countType: &'a str,
        pub r#type: Value,
    }
}

#[derive(Debug)]
struct VarintMappings(BTreeMap<i32, String>);

#[derive(Debug)]
enum DefinitionType {
    Mapper(VarintMappings),
    Switch(raw::Switch),
}

pub struct Schema {
    root: raw::ProtocolRoot,
}

pub struct State<'a> {
    root: &'a raw::ProtocolState,
}

impl Schema {
    pub fn new(json: impl Read) -> serde_json::Result<Self> {
        let root = serde_json::from_reader(json)?;
        Ok(Self { root })
    }

    pub fn per_state<E>(&self, mut f: impl FnMut(&str, State) -> Result<(), E>) -> Result<(), E> {
        f("handshaking", State::new(&self.root.handshaking))?;
        f("status", State::new(&self.root.status))?;
        f("login", State::new(&self.root.login))?;
        f("play", State::new(&self.root.play))?;
        Ok(())
    }
}

impl<'a> State<'a> {
    fn new(root: &'a raw::ProtocolState) -> Self {
        Self { root }
    }

    pub fn per_packet<E: From<SchemaError>>(
        &self,
        mut f: impl FnMut(Packet) -> Result<(), E>,
    ) -> Result<(), E> {
        Self::per_packet_with_direction(
            &self.root.toClient.types,
            PacketDirection::Clientbound,
            &mut f,
        )?;
        Self::per_packet_with_direction(
            &self.root.toServer.types,
            PacketDirection::Serverbound,
            &mut f,
        )?;
        Ok(())
    }

    fn per_packet_with_direction<E: From<SchemaError>>(
        types: &raw::PacketTypes,
        dir: PacketDirection,
        mut f: impl FnMut(Packet) -> Result<(), E>,
    ) -> Result<(), E> {
        use SchemaError::*;

        let packet = types.0.get("packet").ok_or(MissingPacketKey)?;
        let container = extract_specific("container", packet)
            .and_then(|v| v.as_array())
            .ok_or(BadStructure("packet"))?;

        let mut packet_id_mapper = None;
        let mut packet_body_switch = None;
        for def in container.iter() {
            let def = PacketDefinition::deserialize(def)
                .map_err(|e| Deserializing(e, "packet definition"))?;
            let (def_type, value) =
                extract(&def.r#type).ok_or(BadStructure("packet definition"))?;
            match def_type {
                "switch" => {
                    let switch =
                        raw::Switch::deserialize(value).map_err(|e| Deserializing(e, "switch"))?;
                    if packet_body_switch.is_some() {
                        return Err(Duplicate("switch").into());
                    }
                    packet_body_switch = Some(switch);
                }
                "mapper" => {
                    let mapper =
                        raw::Mapper::deserialize(value).map_err(|e| Deserializing(e, "mapper"))?;
                    let mappings = match mapper.r#type {
                        "varint" => VarintMappings::from_values(mapper.mappings.into_iter())
                            .ok_or(BadStructure("packet mappings"))?,
                        ty => return Err(UnknownMapper(ty.to_owned()).into()),
                    };
                    if packet_body_switch.is_some() {
                        return Err(Duplicate("mapper").into());
                    }
                    packet_id_mapper = Some(mappings);
                }
                ty => return Err(UnknownDefinition(ty.to_owned()).into()),
            };
        }

        let (packet_body_switch, packet_id_mapper) = match packet_body_switch.zip(packet_id_mapper)
        {
            Some((a, b)) => (a, b),
            _ => return Err(BadPacketDefinition.into()),
        };

        for (packet_id, packet_name) in packet_id_mapper.0.iter() {
            let key = packet_body_switch
                .fields
                .get(packet_name)
                .and_then(|v| v.as_str())
                .ok_or_else(|| MissingSwitchKey(packet_name.clone(), "packet lookup"))?;
            let body = types.0.get(key).ok_or(BadStructure("packet body"))?;
            let container = extract_specific("container", body)
                .and_then(|v| v.as_array())
                .ok_or(BadStructure("container"))?;

            let mut packet = Packet {
                id: u8::try_from(*packet_id)
                    .map_err(|_| SchemaError::InvalidPacketId(*packet_id))?,
                direction: dir,
                name: packet_name,
                fields: Vec::with_capacity(container.len()),
            };

            for field in container {
                let field =
                    raw::Field::deserialize(field).map_err(|e| Deserializing(e, "field"))?;
                let field_ty = FieldType::try_from(&field.r#type)?;
                assert!(field.name.is_some() || field.anon); // TODO result

                // TODO multiple anons?
                packet.fields.push(Field {
                    name: field.name.unwrap_or("anon"),
                    r#type: field_ty,
                })
            }

            f(packet)?;
        }
        Ok(())
    }
}

/// `[$type, $val]`
fn extract(val: &Value) -> Option<(&str, &Value)> {
    match val {
        Value::Array(array) if array.len() == 2 && array[0].is_string() => {
            Some((&array[0].as_str().unwrap(), &array[1]))
        }
        _ => None,
    }
}

/// `[$type, $val]`
fn extract_specific<'a>(key: &'static str, val: &'a Value) -> Option<&'a Value> {
    extract(val).and_then(|(k, v)| if k == key { Some(v) } else { None })
}

#[derive(Debug)]
pub enum FieldType {
    Varint,
    U16,
    U8,
    I64,
    String,
    Buffer {
        count_ty: Box<FieldType>,
    },
    Array {
        count_ty: Box<FieldType>,
        // TODO elem_ty
    },
    I32,
    I8,
    Bool,
    I16,
    F32,
    F64,
    Uuid,
    EntityMetadata,
    Position,
    RestOfBuffer,
    Nbt,
    OptionalNbt,
    Switch,       // TODO
    Slot,         // TODO
    ParticleData, // TODO
    Option(Box<FieldType>),
    Bitfield,                 // TODO
    TopBitSetTerminatedArray, // TODO
    Tags,                     // TODO
}

#[derive(Debug)]
pub struct Field<'a> {
    pub name: &'a str,
    pub r#type: FieldType,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PacketDirection {
    Clientbound,
    Serverbound,
}

#[derive(Debug)]
pub struct Packet<'a> {
    pub id: u8,
    pub direction: PacketDirection,
    /// Snake case
    pub name: &'a str,
    pub fields: Vec<Field<'a>>,
}

impl VarintMappings {
    fn from_values<'a>(values: impl Iterator<Item = (&'a str, Value)>) -> Option<Self> {
        values
            .map(|(k, v)| {
                let int = i32::from_str_radix(k.trim_start_matches("0x"), 16).ok()?;
                let str = match v {
                    Value::String(s) => s,
                    _ => return None,
                };
                Some((int, str))
            })
            .collect::<Option<BTreeMap<_, _>>>()
            .map(Self)
    }
}

impl TryFrom<&Value> for FieldType {
    type Error = SchemaError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if let Some(str) = value.as_str() {
            str.parse()
        } else if let Some(kv) = extract(value) {
            match kv {
                ("buffer", obj) => {
                    let buffer = raw::BufferType::deserialize(obj)
                        .map_err(|e| SchemaError::Deserializing(e, "buffer"))?;
                    let resolved_ty = buffer.countType.parse()?;
                    Ok(FieldType::Buffer {
                        count_ty: Box::new(resolved_ty),
                    })
                }
                ("switch", obj) => Ok(FieldType::Switch),
                ("array", obj) => {
                    let array = raw::ArrayType::deserialize(obj)
                        .map_err(|e| SchemaError::Deserializing(e, "array"))?;
                    let resolved_ty = array.countType.parse()?;
                    Ok(FieldType::Array {
                        count_ty: Box::new(resolved_ty),
                    })
                }
                ("particleData", obj) => Ok(FieldType::ParticleData),
                ("option", obj) => {
                    let resolved_ty = Self::try_from(obj)?;
                    Ok(FieldType::Option(Box::new(resolved_ty)))
                }
                ("bitfield", obj) => Ok(FieldType::Bitfield),
                ("topBitSetTerminatedArray", obj) => Ok(FieldType::TopBitSetTerminatedArray),
                (k, _) => Err(SchemaError::UnknownFieldType(k.into())),
            }
        } else {
            Err(SchemaError::UnknownFieldType(format!("{:?}", value)))
        }
    }
}

impl FromStr for FieldType {
    type Err = SchemaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "varint" => Self::Varint,
            "u16" => Self::U16,
            "u8" => Self::U8,
            "i64" => Self::I64,
            "string" => Self::String,
            "i32" => Self::I32,
            "i8" => Self::I8,
            "bool" => Self::Bool,
            "i16" => Self::I16,
            "f32" => Self::F32,
            "f64" => Self::F64,
            "UUID" => Self::Uuid,
            "entityMetadata" => Self::EntityMetadata,
            "position" => Self::Position,
            "restBuffer" => Self::RestOfBuffer,
            "nbt" => Self::Nbt,
            "optionalNbt" => Self::OptionalNbt,
            "slot" => Self::Slot,
            "tags" => Self::Tags,
            _ => return Err(SchemaError::UnknownFieldType(s.into())),
        })
    }
}
