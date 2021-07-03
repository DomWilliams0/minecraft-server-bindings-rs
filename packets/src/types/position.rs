use crate::types::field::Field;
use crate::types::{LongField, PacketResult};
use async_std::io::prelude::*;
use async_trait::async_trait;
use std::fmt::{Display, Formatter};

/// < is pre-1.14 and uses old format, >= is post-1.14 and uses new format
const NEW_FORMAT_CUTOFF: u32 = 477;

#[derive(Debug)]
pub struct PositionField<const VERSION: u32> {
    x: i32,
    y: i32,
    z: i32,
}

trait ToSigned: Sized + Copy {
    type Signed;
    fn to_signed(self) -> Self::Signed;
}

trait ToUnsigned: Sized + Copy {
    type Unsigned;
    fn to_unsigned(self) -> Self::Unsigned;
}

#[async_trait]
impl<const VERSION: u32> Field for PositionField<VERSION> {
    type Displayable = Self;

    fn value(&self) -> &Self::Displayable {
        self
    }

    fn size(&self) -> usize {
        8
    }

    #[allow(clippy::branches_sharing_code)]
    async fn read_field<R: Read + Unpin + Send>(r: &mut R) -> PacketResult<Self> {
        let long = LongField::read_field(r).await?;
        let val = long.value().to_unsigned();

        let x;
        let y;
        let z;
        if VERSION >= NEW_FORMAT_CUTOFF {
            x = val >> 38;
            y = val & 0xFFF;
            z = val << 26 >> 38;
        } else {
            x = val >> 38;
            y = (val >> 26) & 0xFFF;
            z = val << 38 >> 38;
        }

        let mut x = x.to_signed();
        let mut y = y.to_signed();
        let mut z = z.to_signed();

        if x >= 2i64.pow(25) {
            x -= 2i64.pow(26)
        }
        if y >= 2i64.pow(11) {
            y -= 2i64.pow(12)
        }
        if z >= 2i64.pow(25) {
            z -= 2i64.pow(26)
        }

        Ok(PositionField {
            x: x as i32,
            y: y as i32,
            z: z as i32,
        })
    }

    async fn write_field<W: Write + Unpin + Send>(&self, w: &mut W) -> PacketResult<()> {
        let x = self.x.to_unsigned() as u64;
        let y = self.y.to_unsigned() as u64;
        let z = self.z.to_unsigned() as u64;
        let value = if VERSION >= NEW_FORMAT_CUTOFF {
            ((x & 0x3FFFFFF) << 38) | ((z & 0x3FFFFFF) << 12) | (y & 0xFFF)
        } else {
            ((x & 0x3FFFFFF) << 38) | ((y & 0xFFF) << 26) | (z & 0x3FFFFFF)
        };

        LongField::new(value.to_signed()).write_field(w).await
    }
}

impl<const VERSION: u32> Display for PositionField<VERSION> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl<const VERSION: u32> PositionField<VERSION> {
    pub fn new((x, y, z): (i32, i32, i32)) -> Option<Self> {
        const MAX_XZ: i32 = 2i32.pow(25) - 1;
        const MIN_XZ: i32 = -(2i32.pow(25));

        const MAX_Y: i32 = 2i32.pow(11) - 1;
        const MIN_Y: i32 = -(2i32.pow(11));

        if x < MIN_XZ || x > MAX_XZ || z < MIN_XZ || z > MAX_XZ || y < MIN_Y || y > MAX_Y {
            None
        } else {
            Some(PositionField { x, y, z })
        }
    }
}

impl ToSigned for u64 {
    type Signed = i64;

    fn to_signed(self) -> Self::Signed {
        Self::Signed::from_ne_bytes(self.to_ne_bytes())
    }
}

impl ToSigned for u32 {
    type Signed = i32;

    fn to_signed(self) -> Self::Signed {
        Self::Signed::from_ne_bytes(self.to_ne_bytes())
    }
}

impl ToUnsigned for i64 {
    type Unsigned = u64;

    fn to_unsigned(self) -> Self::Unsigned {
        Self::Unsigned::from_ne_bytes(self.to_ne_bytes())
    }
}

impl ToUnsigned for i32 {
    type Unsigned = u32;

    fn to_unsigned(self) -> Self::Unsigned {
        Self::Unsigned::from_ne_bytes(self.to_ne_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_std::io::Cursor;

    fn check<const PROTOCOL: u32>(pos: (i32, i32, i32)) {
        async_std::task::block_on(async {
            let field = PositionField::<PROTOCOL>::new(pos).expect("position is out of range");

            let mut buffer = Cursor::new(vec![0u8; 8]);
            field.write_field(&mut buffer).await.expect("write failed");

            buffer.set_position(0);
            let parsed = PositionField::<PROTOCOL>::read_field(&mut buffer)
                .await
                .expect("read failed");
            assert_eq!(pos, (parsed.x, parsed.y, parsed.z));
        });
    }

    #[test]
    fn position() {
        const NEW: u32 = NEW_FORMAT_CUTOFF + 10;
        const OLD: u32 = NEW_FORMAT_CUTOFF - 10;

        check::<NEW>((1, 2, 3));
        check::<OLD>((1, 2, 3));

        check::<NEW>((-10, 150, 40000));
        check::<OLD>((-10, 150, 40000));

        check::<NEW>((-20000, -2000, 200000));
        check::<OLD>((-20000, -2000, 200000));
    }
}
