#![allow(non_snake_case)]

use crate::schema::raw::PacketDefinition;
use serde::*;
use serde_json::Value;
use smallvec::SmallVec;
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::io::Read;
use std::str::FromStr;

mod raw {
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
        pub name: &'a str,
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
struct VarintMappings(BTreeMap<u8, String>); // TODO varint

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

    pub fn per_state(&self, mut f: impl FnMut(&str, State)) {
        f("handshaking", State::new(&self.root.handshaking));
        f("status", State::new(&self.root.status));
        f("login", State::new(&self.root.login));
        f("play", State::new(&self.root.play));
    }
}

impl<'a> State<'a> {
    fn new(root: &'a raw::ProtocolState) -> Self {
        Self { root }
    }

    pub fn per_clientbound(&self, f: impl FnMut(Packet)) {
        self.root.toClient.types.per_packet(f);
    }

    pub fn per_serverbound(&self, f: impl FnMut(Packet)) {
        self.root.toServer.types.per_packet(f);
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
}

#[derive(Debug)]
pub struct Field<'a> {
    name: &'a str,
    r#type: FieldType,
}

#[derive(Debug)]
pub struct Packet<'a> {
    /// Snake case
    name: &'a str,
    fields: Vec<Field<'a>>,
}

impl raw::PacketTypes {
    pub fn per_packet(&self, mut f: impl FnMut(Packet)) {
        // TODO results instead of asserts and panics

        let packet = self.0.get("packet").expect("missing packet key");
        let container = extract_specific("container", packet)
            .and_then(|v| v.as_array())
            .expect("bad packet");

        let mut packet_id_mapper = None;
        let mut packet_body_switch = None;
        for def in container.iter() {
            let def = PacketDefinition::deserialize(def).expect("bad definition");
            let (def_type, value) = extract(&def.r#type).expect("bad definition");
            match def_type {
                "switch" => {
                    let switch = raw::Switch::deserialize(value).expect("bad switch");
                    assert!(packet_body_switch.is_none(), "duplicate switch");
                    packet_body_switch = Some(switch);
                }
                "mapper" => {
                    let mapper = raw::Mapper::deserialize(value).expect("bad mapper");
                    let mappings = match mapper.r#type {
                        "varint" => VarintMappings::from_values(mapper.mappings.into_iter())
                            .expect("bad mappings"),
                        ty => panic!("unknown mapper type {:?}", ty),
                    };
                    assert!(packet_id_mapper.is_none(), "duplicate switch");
                    packet_id_mapper = Some(mappings);
                }
                ty => panic!("unknown type {:?}", ty),
            };
        }

        let (packet_body_switch, packet_id_mapper) = match packet_body_switch.zip(packet_id_mapper)
        {
            Some((a, b)) => (a, b),
            _ => panic!("packet definitions missing switch or mapper"),
        };

        for (packet_id, packet_name) in packet_id_mapper.0.iter() {
            let key = packet_body_switch
                .fields
                .get(packet_name)
                .and_then(|v| v.as_str())
                .expect("bad packet");
            let body = self.0.get(key).expect("missing packet body");
            let container = extract_specific("container", body)
                .expect("non-container packet body")
                .as_array()
                .unwrap();

            let mut packet = Packet {
                name: packet_name,
                fields: Vec::with_capacity(container.len()),
            };

            for field in container {
                let field = raw::Field::deserialize(field).expect("bad field");
                let field_ty = FieldType::try_from(&field.r#type)
                    .unwrap_or_else(|_| panic!("bad field type {:?}", field));

                packet.fields.push(Field {
                    name: field.name,
                    r#type: field_ty,
                })
            }

            f(packet);
        }
    }
}

impl VarintMappings {
    fn from_values<'a>(values: impl Iterator<Item = (&'a str, Value)>) -> Option<Self> {
        values
            .map(|(k, v)| {
                let int = u8::from_str_radix(k.trim_start_matches("0x"), 16).ok()?;
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
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if let Some(str) = value.as_str() {
            str.parse()
        } else if let Some(kv) = extract(value) {
            match kv {
                ("buffer", obj) => {
                    let buffer = raw::BufferType::deserialize(obj).map_err(|_| ())?;
                    let resolved_ty = buffer.countType.parse()?;
                    assert!(!matches!(resolved_ty, FieldType::Buffer{..})); // no nesting
                    Ok(FieldType::Buffer {
                        count_ty: Box::new(resolved_ty),
                    })
                }
                ("switch", obj) => Ok(FieldType::Switch),
                ("array", obj) => {
                    let array = raw::ArrayType::deserialize(obj).map_err(|_| ())?;
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
                (k, _) => unimplemented!("field type {:?}", k),
            }
        } else {
            unreachable!()
        }
    }
}

impl FromStr for FieldType {
    type Err = ();

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
            _ => return Err(()),
        })
    }
}
