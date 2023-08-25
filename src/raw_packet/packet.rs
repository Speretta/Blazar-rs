use crate::util::uid::UUID;

use super::{
    reader::RawPacketReader,
    types::{VarInt, VarLong},
    writer::RawPacketWriter,
};

pub struct RawPacketCreator {}

impl RawPacketCreator {
    pub fn new_reader() -> RawPacketReader {
        RawPacketReader::new()
    }

    pub fn new_writer(packet_id: i32) -> RawPacketWriter {
        RawPacketWriter::new(packet_id)
    }
}

#[derive(Debug)]
pub struct RawPacket {
    fields: Vec<RawPacketField>,
}

impl RawPacket {
    pub(super) fn new(fields: Vec<RawPacketField>) -> Self {
        RawPacket { fields }
    }

    pub fn get_packet_id(&self) -> u32 {
        if let Some(RawPacketField::VarInt(id)) = self.fields.get(0) {
            *id as u32
        } else {
            0
        }
    }

    pub fn get_field(&self, index: usize) -> Option<&RawPacketField> {
        self.fields.get(index + 1)
    }

    #[inline]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for field in &self.fields {
            match field {
                RawPacketField::Bool(value) => bytes.push(*value as u8),
                RawPacketField::Byte(value) => bytes.push(*value as u8),
                RawPacketField::UByte(value) => bytes.push(*value),
                RawPacketField::Short(value) => bytes.extend(value.to_be_bytes()),
                RawPacketField::UShort(value) => bytes.extend(value.to_be_bytes()),
                RawPacketField::Int(value) => bytes.extend(value.to_be_bytes()),
                RawPacketField::Long(value) => bytes.extend(value.to_be_bytes()),
                RawPacketField::Float(value) => bytes.extend(value.to_be_bytes()),
                RawPacketField::Double(value) => bytes.extend(value.to_be_bytes()),
                RawPacketField::String(value) => {
                    let value_bytes = value.as_bytes();
                    VarInt::write_varint(&mut bytes, value_bytes.len() as i32);
                    bytes.extend(value_bytes);
                }
                RawPacketField::VarInt(value) => VarInt::write_varint(&mut bytes, *value),
                RawPacketField::VarLong(value) => VarLong::write_varlong(&mut bytes, *value),
                RawPacketField::Uuid(value) => bytes.extend(value.to_be_bytes()),
            }
        }
        let mut tail_length = 0;
        if bytes.len() != 1{
            for byte in bytes.iter().rev() {
                if *byte != 0  {
                    break;
                }
                tail_length += 1;
            }
        }
        let mut buffer = Vec::new();
        VarInt::write_varint(&mut buffer, (bytes.len() - tail_length) as i32);
        [buffer, bytes[..bytes.len() - tail_length].to_vec()].concat() //[buffer, bytes[..bytes.len()-tail_length].to_vec()].concat()
    }
}

#[derive(Debug)]
pub enum RawPacketField {
    Bool(bool),
    Byte(i8),
    UByte(u8),
    Short(i16),
    UShort(u16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    VarInt(i32),
    VarLong(i64),
    Uuid(UUID),
}
