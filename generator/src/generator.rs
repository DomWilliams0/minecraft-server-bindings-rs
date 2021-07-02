use crate::schema::{FieldType, Packet, PacketDirection, SchemaError};
use displaydoc::Display;
use inflector::Inflector;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug)]
pub struct ModuleGenerator {
    module_dir: PathBuf,
}

pub struct StateGenerator {
    file: File,
}

#[derive(Debug, Display, Error)]
pub enum GeneratorError {
    /// IO error: {0}
    Io(#[from] std::io::Error),

    /// Schema error: {0}
    Schema(#[from] SchemaError),

    /// Path given is not a directory
    NotADir,
}

pub type GeneratorResult<T> = Result<T, GeneratorError>;

// TODO need to use chat types instead of string types

/// Formattable as rust code
struct EmittableFieldType(FieldType);

impl ModuleGenerator {
    /// Module dir will be **deleted** and have mod.rs created
    pub fn new(module_dir: impl Into<PathBuf>) -> GeneratorResult<Self> {
        let module_dir = module_dir.into();
        if module_dir.exists() && !module_dir.is_dir() {
            return Err(GeneratorError::NotADir);
        }

        if module_dir.exists() {
            std::fs::remove_dir_all(&module_dir)?;
        }
        std::fs::create_dir_all(&module_dir)?;

        Ok(Self { module_dir })
    }

    pub fn emit_state(&mut self, state: &str) -> GeneratorResult<StateGenerator> {
        let mut mod_rs = self.append_to_mod_rs()?;
        writeln!(
            &mut mod_rs,
            "mod {state};\npub use {state}::*;",
            state = state
        )?;
        StateGenerator::new(&self.module_dir, state).map_err(Into::into)
    }

    fn append_to_mod_rs(&self) -> GeneratorResult<impl std::io::Write> {
        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.module_dir.join("mod.rs"))
            .map_err(Into::into)
    }
}

impl StateGenerator {
    fn new(mod_dir: &Path, state: &str) -> std::io::Result<Self> {
        let file_name = {
            let mut name = PathBuf::from(state.to_lowercase());
            name.set_extension("rs");
            name
        };
        let file = std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(mod_dir.join(file_name))?;

        // TODO includes

        Ok(Self { file })
    }

    pub fn emit_packet(&mut self, packet: &Packet) -> GeneratorResult<()> {
        let struct_name = packet.name.to_pascal_case();
        let derive = match packet.direction {
            PacketDirection::Clientbound => "ClientBoundPacket",
            PacketDirection::Serverbound => "ServerBoundPacket",
        };
        writeln!(
            &mut self.file,
            "#[derive({derive})]\n#[packet_id = {id:#04x}]\npub struct {name} {{",
            derive = derive,
            id = packet.id,
            name = struct_name
        )?;

        for field in &packet.fields {
            let ty = field_type(&field.r#type);
            let comment = if ty.is_none() { "// TODO " } else { "" };
            write!(
                &mut self.file,
                "\t{comment}pub {ident}: ",
                comment = comment,
                ident = field.name.to_snake_case(),
            )?;

            if let Some(ty) = ty {
                writeln!(&mut self.file, "{},", ty)?;
            } else {
                writeln!(&mut self.file, "{:?},", field.r#type)?;
            }
        }

        writeln!(&mut self.file, "}}\n")?;
        Ok(())
    }
}

fn field_type(ty: &FieldType) -> Option<&'static str> {
    use FieldType::*;
    Some(match ty {
        Varint => "VarIntField",
        U16 => "UShortField",
        U8 => "UByteField",
        I64 => "LongField",
        I32 => "IntField",
        I8 => "ByteField",
        I16 => "ShortField",
        F32 => "FloatField",
        F64 => "DoubleField",
        Bool => "BoolField",
        String => "StringField",
        Buffer { count_ty } if matches!(**count_ty, FieldType::Varint) => {
            "VarIntThenByteArrayField"
        }
        // Array { .. } => {}
        // Uuid => {}
        // EntityMetadata => {}
        // Position => {}
        RestOfBuffer => "RestOfPacketByteArrayField",
        // Nbt => {}
        // OptionalNbt => {}
        // Switch => {}
        // Slot => {}
        // ParticleData => {}
        // Option(_) => {}
        // Bitfield => {}
        // TopBitSetTerminatedArray => {}
        _ => return None,
    })
}
