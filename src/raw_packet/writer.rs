use crate::{
    raw_packet::{
        packet::RawPacketField,
        types::{VarInt, VarLong},
    },
    util::uid::UUID,
};

use super::packet::RawPacket;

pub struct RawPacketWriter {
    pub(super) fields: Vec<RawPacketFieldWrite>,
}

impl RawPacketWriter {
    pub(super) fn new(packet_id: i32) -> Self {
        assert!(packet_id >= 0, "Packet ID must not be negative");
        RawPacketWriter { fields: Vec::new() }.add_field(RawPacketFieldWrite::VarInt(packet_id))
    }

    pub fn add_field(mut self, field: RawPacketFieldWrite) -> Self {
        self.fields.push(field);
        self
    }

    pub fn build(self) -> RawPacket {
        let fields = self.fields.into_iter().map(|field| field.into()).collect();

        RawPacket::new(fields)
    }
}

pub enum RawPacketFieldWrite {
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

impl From<RawPacketFieldWrite> for RawPacketField {
    fn from(value: RawPacketFieldWrite) -> Self {
        let field = value;
        match field {
            RawPacketFieldWrite::Bool(value) => RawPacketField::Bool(value),
            RawPacketFieldWrite::Byte(value) => RawPacketField::Byte(value),
            RawPacketFieldWrite::UByte(value) => RawPacketField::UByte(value),
            RawPacketFieldWrite::Short(value) => RawPacketField::Short(value),
            RawPacketFieldWrite::UShort(value) => RawPacketField::UShort(value),
            RawPacketFieldWrite::Int(value) => RawPacketField::Int(value),
            RawPacketFieldWrite::Long(value) => RawPacketField::Long(value),
            RawPacketFieldWrite::Float(value) => RawPacketField::Float(value),
            RawPacketFieldWrite::Double(value) => RawPacketField::Double(value),
            RawPacketFieldWrite::String(value) => RawPacketField::String(value),
            RawPacketFieldWrite::VarInt(value) => RawPacketField::VarInt(value),
            RawPacketFieldWrite::VarLong(value) => RawPacketField::VarLong(value),
            RawPacketFieldWrite::Uuid(value) => RawPacketField::Uuid(value),
        }
    }
}

impl From<RawPacketField> for RawPacketFieldWrite {
    fn from(value: RawPacketField) -> Self {
        let field = value;
        match field {
            RawPacketField::Bool(value) => RawPacketFieldWrite::Bool(value),
            RawPacketField::Byte(value) => RawPacketFieldWrite::Byte(value),
            RawPacketField::UByte(value) => RawPacketFieldWrite::UByte(value),
            RawPacketField::Short(value) => RawPacketFieldWrite::Short(value),
            RawPacketField::UShort(value) => RawPacketFieldWrite::UShort(value),
            RawPacketField::Int(value) => RawPacketFieldWrite::Int(value),
            RawPacketField::Long(value) => RawPacketFieldWrite::Long(value),
            RawPacketField::Float(value) => RawPacketFieldWrite::Float(value),
            RawPacketField::Double(value) => RawPacketFieldWrite::Double(value),
            RawPacketField::String(value) => RawPacketFieldWrite::String(value),
            RawPacketField::VarInt(value) => RawPacketFieldWrite::VarInt(value),
            RawPacketField::VarLong(value) => RawPacketFieldWrite::VarLong(value),
            RawPacketField::Uuid(value) => RawPacketFieldWrite::Uuid(value),
        }
    }
}

impl From<&RawPacketField> for RawPacketFieldWrite {
    fn from(value: &RawPacketField) -> Self {
        let field = value;
        match field {
            RawPacketField::Bool(value) => RawPacketFieldWrite::Bool(*value),
            RawPacketField::Byte(value) => RawPacketFieldWrite::Byte(*value),
            RawPacketField::UByte(value) => RawPacketFieldWrite::UByte(*value),
            RawPacketField::Short(value) => RawPacketFieldWrite::Short(*value),
            RawPacketField::UShort(value) => RawPacketFieldWrite::UShort(*value),
            RawPacketField::Int(value) => RawPacketFieldWrite::Int(*value),
            RawPacketField::Long(value) => RawPacketFieldWrite::Long(*value),
            RawPacketField::Float(value) => RawPacketFieldWrite::Float(*value),
            RawPacketField::Double(value) => RawPacketFieldWrite::Double(*value),
            RawPacketField::String(value) => RawPacketFieldWrite::String(value.clone()),
            RawPacketField::VarInt(value) => RawPacketFieldWrite::VarInt(*value),
            RawPacketField::VarLong(value) => RawPacketFieldWrite::VarLong(*value),
            RawPacketField::Uuid(value) => RawPacketFieldWrite::Uuid(*value),
        }
    }
}
