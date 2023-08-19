const SEGMENT_BITS: u32 = 0x7F;
const CONTINUE_BIT: u32 = 0x80;




pub(super) struct VarInt{}

impl VarInt{
    pub(super) fn write_varint(buffer:&mut Vec<u8>, integer32: i32){
        let mut value = integer32 as u32;
        for _ in 0..=4{
            if (value & !SEGMENT_BITS) == 0 {
                buffer.push(value as u8);
                break;
            }
            buffer.push(((value & SEGMENT_BITS) | CONTINUE_BIT) as u8);
            value >>= 7;
        }
    }
}


pub(super) struct VarLong{}

impl VarLong{
    pub(super) fn write_varlong(buffer:&mut Vec<u8>, integer64: i64){
        let mut value = integer64 as u64;
        for _ in 0..=9{
            if (value & !SEGMENT_BITS as u64) == 0 {
                buffer.push(value as u8);
                break;
            }
            buffer.push(((value & SEGMENT_BITS as u64) | CONTINUE_BIT as u64) as u8);
            value >>= 7;
        }
    }
}