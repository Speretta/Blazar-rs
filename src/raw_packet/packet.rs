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
        if let Some(RawPacketField::VARINT(id)) = self.fields.get(0) {
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
                RawPacketField::BOOL(value) => bytes.push(*value as u8),
                RawPacketField::BYTE(value) => bytes.push(*value as u8),
                RawPacketField::UBYTE(value) => bytes.push(*value as u8),
                RawPacketField::SHORT(value) => bytes.extend(value.to_be_bytes()),
                RawPacketField::USHORT(value) => bytes.extend(value.to_be_bytes()),
                RawPacketField::INT(value) => bytes.extend(value.to_be_bytes()),
                RawPacketField::LONG(value) => bytes.extend(value.to_be_bytes()),
                RawPacketField::FLOAT(value) => bytes.extend(value.to_be_bytes()),
                RawPacketField::DOUBLE(value) => bytes.extend(value.to_be_bytes()),
                RawPacketField::STRING(value) => {
                    let value_bytes = value.as_bytes();
                    VarInt::write_varint(&mut bytes, value_bytes.len() as i32);
                    bytes.extend(value_bytes);
                }
                RawPacketField::VARINT(value) => VarInt::write_varint(&mut bytes, *value),
                RawPacketField::VARLONG(value) => VarLong::write_varlong(&mut bytes, *value),
            }
        }
        let mut buffer = Vec::new();
        VarInt::write_varint(&mut buffer, bytes.len() as i32);
        [buffer, bytes].concat()
    }
}

#[derive(Debug)]
pub enum RawPacketField {
    BOOL(bool),
    BYTE(i8),
    UBYTE(u8),
    SHORT(i16),
    USHORT(u16),
    INT(i32),
    LONG(i64),
    FLOAT(f32),
    DOUBLE(f64),
    STRING(String),
    VARINT(i32),
    VARLONG(i64),
}
