use super::types::{VarInt, VarLong};




pub struct RawPacket{
    vec: Vec<u8>
}


impl RawPacket{
    pub fn new(packet_id: i32) -> Self{
        assert!(packet_id >= 0, "Packet ID must not be negative");
        let builder = RawPacket { vec: Vec::new() };
        builder.write_varint(packet_id)
    }

    pub fn write_be_bytes(mut self, bytes: &[u8]) -> Self{
        self.vec.extend_from_slice(bytes);
        self
    }

    pub fn write_string<S: Into<String>>(mut self, text: S) -> Self{
        let text: String = text.into();
        VarInt::write_varint(&mut self.vec, text.len() as i32);
        self.write_be_bytes(text.as_bytes())
    }

    pub fn write_varint(mut self, integer32: i32) -> Self{
        VarInt::write_varint(&mut self.vec, integer32);
        self
    }

    pub fn write_varlong(mut self, integer64: i64) -> Self{
        VarLong::write_varlong(&mut self.vec, integer64);
        self
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.vec
    }

}