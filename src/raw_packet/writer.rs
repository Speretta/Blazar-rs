use crate::{raw_packet::{
    packet::RawPacketField,
    types::{VarInt, VarLong},
}, util::uid::UUID};

use super::packet::RawPacket;

pub struct RawPacketWriter {
    pub(super) fields: Vec<RawPacketFieldWrite>,
}

impl RawPacketWriter {
    pub(super) fn new(packet_id: i32) -> Self {
        assert!(packet_id >= 0, "Packet ID must not be negative");
        RawPacketWriter { fields: Vec::new() }.add_field(RawPacketFieldWrite::VARINT(packet_id))
    }

    pub fn add_field(mut self, field: RawPacketFieldWrite) -> Self {
        self.fields.push(field);
        self
    }

    pub fn build(self) -> RawPacket {
        let fields = self
            .fields
            .into_iter()
            .map(|field| match field {
                RawPacketFieldWrite::BOOL(value) => RawPacketField::BOOL(value),
                RawPacketFieldWrite::BYTE(value) => RawPacketField::BYTE(value),
                RawPacketFieldWrite::UBYTE(value) => RawPacketField::UBYTE(value),
                RawPacketFieldWrite::SHORT(value) => RawPacketField::SHORT(value),
                RawPacketFieldWrite::USHORT(value) => RawPacketField::USHORT(value),
                RawPacketFieldWrite::INT(value) => RawPacketField::INT(value),
                RawPacketFieldWrite::LONG(value) => RawPacketField::LONG(value),
                RawPacketFieldWrite::FLOAT(value) => RawPacketField::FLOAT(value),
                RawPacketFieldWrite::DOUBLE(value) => RawPacketField::DOUBLE(value),
                RawPacketFieldWrite::STRING(value) => RawPacketField::STRING(value),
                RawPacketFieldWrite::VARINT(value) => RawPacketField::VARINT(value),
                RawPacketFieldWrite::VARLONG(value) => RawPacketField::VARLONG(value),
                RawPacketFieldWrite::UUID(value) => RawPacketField::UUID(value),
            })
            .collect();

        RawPacket::new(fields)
    }
}

pub enum RawPacketFieldWrite {
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
    UUID(UUID),
}
