use crate::{
    raw_packet::{
        packet::RawPacketField,
        types::{VarInt, VarLong},
    },
    util::uid::UUID,
};

use super::packet::RawPacket;

pub struct RawPacketReader {
    pub(super) fields: Vec<RawPacketFieldRead>,
}

impl RawPacketReader {
    pub(super) fn new() -> Self {
        RawPacketReader {
            fields: vec![RawPacketFieldRead::VarInt],
        }
    }

    pub fn add_field(mut self, field: RawPacketFieldRead) -> Self {
        self.fields.push(field);
        self
    }

    pub fn build(self, buffer: &[u8]) -> RawPacket {
        let mut position = 0usize;
        let packet_size = {
            let (length, var_int) = VarInt::read_varint(buffer);
            position += length;
            var_int as usize + length
        };
        let fields = self
            .fields
            .into_iter()
            .map(|field| {
                assert!(
                    position < buffer.len(),
                    "Buffer size doesn't match fields size"
                );
                assert!(
                    position < packet_size,
                    "Packet size doesn't match fields size: {} !< {} | {:?}",
                    position,
                    packet_size,
                    buffer
                );

                let field = match field {
                    RawPacketFieldRead::Bool => RawPacketField::Bool(buffer[position] != 0),
                    RawPacketFieldRead::Byte => RawPacketField::Byte(buffer[position] as i8),
                    RawPacketFieldRead::UByte => RawPacketField::UByte(buffer[position]),
                    RawPacketFieldRead::Short => RawPacketField::Short(i16::from_be_bytes(
                        buffer[position..position + 2].try_into().unwrap(),
                    )),
                    RawPacketFieldRead::UShort => RawPacketField::UShort(u16::from_be_bytes(
                        buffer[position..position + 2].try_into().unwrap(),
                    )),
                    RawPacketFieldRead::Int => RawPacketField::Int(i32::from_be_bytes(
                        buffer[position..position + 4].try_into().unwrap(),
                    )),
                    RawPacketFieldRead::Long => RawPacketField::Long(i64::from_be_bytes(
                        buffer[position..position + 8].try_into().unwrap(),
                    )),
                    RawPacketFieldRead::Float => RawPacketField::Float(f32::from_be_bytes(
                        buffer[position..position + 4].try_into().unwrap(),
                    )),
                    RawPacketFieldRead::Double => RawPacketField::Double(f64::from_be_bytes(
                        buffer[position..position + 8].try_into().unwrap(),
                    )),
                    RawPacketFieldRead::String => {
                        let (length, var_int) = VarInt::read_varint(&buffer[position..]);
                        position += length + var_int as usize;
                        RawPacketField::String(
                            String::from_utf8_lossy(&buffer[position - var_int as usize..position])
                                .to_string(),
                        )
                    }
                    RawPacketFieldRead::VarInt => {
                        let (length, var_int) = VarInt::read_varint(&buffer[position..]);
                        position += length;
                        RawPacketField::VarInt(var_int)
                    }
                    RawPacketFieldRead::VarLong => {
                        let (length, var_long) = VarLong::read_varlong(&buffer[position..]);
                        position += length;
                        RawPacketField::VarLong(var_long)
                    }
                    RawPacketFieldRead::Uuid => RawPacketField::Uuid(UUID::from_be_bytes(
                        buffer[position..position + 16].try_into().unwrap(),
                    )),
                };
                position += get_size(&field);

                field
            })
            .collect();
        RawPacket::new(fields)
    }
}

pub enum RawPacketFieldRead {
    Bool,
    Byte,
    UByte,
    Short,
    UShort,
    Int,
    Long,
    Float,
    Double,
    String,
    VarInt,
    VarLong,
    Uuid,
}

fn get_size(field: &RawPacketField) -> usize {
    match field {
        RawPacketField::Bool(_) => 1,
        RawPacketField::Byte(_) => 1,
        RawPacketField::UByte(_) => 1,
        RawPacketField::Short(_) => 2,
        RawPacketField::UShort(_) => 2,
        RawPacketField::Int(_) => 4,
        RawPacketField::Long(_) => 8,
        RawPacketField::Float(_) => 4,
        RawPacketField::Double(_) => 8,
        RawPacketField::String(_) => 0,
        RawPacketField::VarInt(_) => 0,
        RawPacketField::VarLong(_) => 0,
        RawPacketField::Uuid(_) => 16,
    }
}
