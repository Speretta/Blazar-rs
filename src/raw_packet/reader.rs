use crate::raw_packet::{
    packet::RawPacketField,
    types::{VarInt, VarLong},
};

use super::packet::RawPacket;

pub struct RawPacketReader {
    pub(super) fields: Vec<RawPacketFieldRead>,
}

impl RawPacketReader {
    pub(super) fn new() -> Self {
        RawPacketReader {
            fields: vec![RawPacketFieldRead::VARINT],
        }
    }

    pub fn add_field(mut self, field: RawPacketFieldRead) -> Self {
        self.fields.push(field);
        self
    }

    pub fn build(self, buffer: &[u8]) -> RawPacket {
        let mut position = 0usize;
        let packet_size = {
            let (length, var_int) = VarInt::read_varint(&buffer);
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
                    RawPacketFieldRead::BOOL => RawPacketField::BOOL(buffer[position] != 0),
                    RawPacketFieldRead::BYTE => RawPacketField::BYTE(buffer[position] as i8),
                    RawPacketFieldRead::UBYTE => RawPacketField::UBYTE(buffer[position]),
                    RawPacketFieldRead::SHORT => RawPacketField::SHORT(i16::from_be_bytes(
                        buffer[position..position + 2].try_into().unwrap(),
                    )),
                    RawPacketFieldRead::USHORT => RawPacketField::USHORT(u16::from_be_bytes(
                        buffer[position..position + 2].try_into().unwrap(),
                    )),
                    RawPacketFieldRead::INT => RawPacketField::INT(i32::from_be_bytes(
                        buffer[position..position + 4].try_into().unwrap(),
                    )),
                    RawPacketFieldRead::LONG => RawPacketField::LONG(i64::from_be_bytes(
                        buffer[position..position + 8].try_into().unwrap(),
                    )),
                    RawPacketFieldRead::FLOAT => RawPacketField::FLOAT(f32::from_be_bytes(
                        buffer[position..position + 4].try_into().unwrap(),
                    )),
                    RawPacketFieldRead::DOUBLE => RawPacketField::DOUBLE(f64::from_be_bytes(
                        buffer[position..position + 8].try_into().unwrap(),
                    )),
                    RawPacketFieldRead::STRING => {
                        let (length, var_int) = VarInt::read_varint(&buffer[position..]);
                        position += length + var_int as usize;
                        RawPacketField::STRING(
                            String::from_utf8_lossy(&buffer[position - var_int as usize..position])
                                .to_string(),
                        )
                    }
                    RawPacketFieldRead::VARINT => {
                        let (length, var_int) = VarInt::read_varint(&buffer[position..]);
                        position += length;
                        RawPacketField::VARINT(var_int)
                    }
                    RawPacketFieldRead::VARLONG => {
                        let (length, var_long) = VarLong::read_varlong(&buffer[position..]);
                        position += length;
                        RawPacketField::VARLONG(var_long)
                    }
                };
                position += get_size(&field);

                field
            })
            .collect();
        RawPacket::new(fields)
    }
}

pub enum RawPacketFieldRead {
    BOOL,
    BYTE,
    UBYTE,
    SHORT,
    USHORT,
    INT,
    LONG,
    FLOAT,
    DOUBLE,
    STRING,
    VARINT,
    VARLONG,
}

fn get_size(field: &RawPacketField) -> usize {
    match field {
        RawPacketField::BOOL(_) => 1,
        RawPacketField::BYTE(_) => 1,
        RawPacketField::UBYTE(_) => 1,
        RawPacketField::SHORT(_) => 2,
        RawPacketField::USHORT(_) => 2,
        RawPacketField::INT(_) => 4,
        RawPacketField::LONG(_) => 8,
        RawPacketField::FLOAT(_) => 4,
        RawPacketField::DOUBLE(_) => 8,
        RawPacketField::STRING(_) => 0,
        RawPacketField::VARINT(_) => 0,
        RawPacketField::VARLONG(_) => 0,
    }
}
