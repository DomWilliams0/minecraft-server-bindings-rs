use std::convert::TryFrom;

use async_std::io::prelude::*;

use crate::types::field::Field;
use crate::types::{PacketError, PacketResult, VarIntField};
use async_trait::async_trait;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct StringField {
    value: String,

    /// String length
    length: VarIntField,
}

impl StringField {
    pub fn new(value: String) -> Self {
        assert!(i32::try_from(value.len()).is_ok());

        let len = value.len();
        Self {
            value,
            length: VarIntField::new(len as i32),
        }
    }

    pub fn new_chat(value: impl Display) -> Self {
        Self::new(format!(r#"{{"text": "{}"}}"#, value))
    }

    pub fn take(self) -> String {
        self.value
    }
}

#[async_trait]
impl Field for StringField {
    type Displayable = String;

    fn value(&self) -> &Self::Displayable {
        &self.value
    }

    fn size(&self) -> usize {
        self.length.size() + self.length.value() as usize
    }

    async fn read_field<R: Read + Unpin + Send>(r: &mut R) -> PacketResult<Self> {
        let length = VarIntField::read_field(r).await?.value() as usize;
        let value = {
            let mut vec = vec![0u8; length];
            r.read_exact(&mut vec).await.map_err(PacketError::Io)?;
            String::from_utf8(vec)?
        };

        Ok(Self::new(value))
    }

    async fn write_field<W: Write + Unpin + Send>(&self, w: &mut W) -> PacketResult<()> {
        self.length.write_field(w).await?;

        w.write_all(self.value.as_bytes())
            .await
            .map_err(PacketError::Io)
    }
}

pub struct IdentifierField {
    string: StringField,
    colon: Option<usize>,
}

impl IdentifierField {
    pub fn new(s: String) -> Self {
        let colon = s.find(':');

        Self {
            string: StringField::new(s),
            colon,
        }
    }

    pub fn namespace(&self) -> &str {
        match self.colon {
            Some(idx) => &self.string.value[..idx],
            None => "minecraft",
        }
    }

    pub fn location(&self) -> &str {
        match self.colon {
            Some(idx) => &self.string.value[idx + 1..],
            None => &self.string.value,
        }
    }
}

impl From<StringField> for IdentifierField {
    fn from(s: StringField) -> Self {
        Self::new(s.take())
    }
}

impl Debug for IdentifierField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace(), self.location())
    }
}

#[async_trait]
impl Field for IdentifierField {
    type Displayable = String;

    fn value(&self) -> &Self::Displayable {
        self.string.value()
    }

    fn size(&self) -> usize {
        self.string.size()
    }

    async fn read_field<R: Read + Unpin + Send>(r: &mut R) -> PacketResult<Self> {
        StringField::read_field(r)
            .await
            .map(|s| Self::new(s.take()))
    }

    async fn write_field<W: Write + Unpin + Send>(&self, w: &mut W) -> PacketResult<()> {
        self.string.write_field(w).await
    }
}

#[cfg(test)]
mod test {
    use crate::types::IdentifierField;

    #[test]
    fn identifier() {
        let default = IdentifierField::new("bonbon".to_owned());
        let custom = IdentifierField::new("colon:sunglass".to_lowercase());
        let bad = IdentifierField::new("ohno:".to_lowercase());

        assert_eq!(default.namespace(), "minecraft");
        assert_eq!(default.location(), "bonbon");

        assert_eq!(custom.namespace(), "colon");
        assert_eq!(custom.location(), "sunglass");

        assert_eq!(bad.namespace(), "ohno");
        assert_eq!(bad.location(), "");
    }
}
