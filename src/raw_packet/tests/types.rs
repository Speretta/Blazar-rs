mod var_int {
    use crate::raw_packet::types::VarInt;

    #[test]
    fn small() {
        let mut buffer = Vec::with_capacity(3);
        VarInt::write_varint(&mut buffer, 0);
        VarInt::write_varint(&mut buffer, 2);
        VarInt::write_varint(&mut buffer, 127);
        assert!(buffer.as_slice() == [0, 2, 127]);
    }

    #[test]
    fn big() {
        let mut buffer = Vec::with_capacity(10);
        VarInt::write_varint(&mut buffer, 255);
        VarInt::write_varint(&mut buffer, 25565);
        VarInt::write_varint(&mut buffer, 2147483647);
        assert!(buffer.as_slice() == [255, 1, 221, 199, 1, 255, 255, 255, 255, 7]);
    }

    #[test]
    fn negative_big() {
        let mut buffer = Vec::with_capacity(15);
        VarInt::write_varint(&mut buffer, -1);
        VarInt::write_varint(&mut buffer, -2);
        VarInt::write_varint(&mut buffer, -127);
        assert!(buffer.as_slice() == [255, 255, 255, 255, 15, 254, 255, 255, 255, 15, 129, 255, 255, 255, 15]);
    }

    #[test]
    fn negative_small() {
        let mut buffer = Vec::with_capacity(15);
        VarInt::write_varint(&mut buffer, -255);
        VarInt::write_varint(&mut buffer, -25565);
        VarInt::write_varint(&mut buffer, -2147483648);
        assert!(buffer.as_slice() == [129, 254, 255, 255, 15, 163, 184, 254, 255, 15, 128, 128, 128, 128, 8]);
    }
    
}
mod var_long {
    use crate::raw_packet::types::VarLong;
    #[test]
    fn small() {
        let mut buffer = Vec::with_capacity(3);
        VarLong::write_varlong(&mut buffer, 0);
        VarLong::write_varlong(&mut buffer, 2);
        VarLong::write_varlong(&mut buffer, 127);
        assert!(buffer.as_slice() == [0, 2, 127]);
    }

    #[test]
    fn big() {
        let mut buffer = Vec::with_capacity(10);
        VarLong::write_varlong(&mut buffer, 255);
        VarLong::write_varlong(&mut buffer, 25565);
        VarLong::write_varlong(&mut buffer, 2147483647);
        assert!(buffer.as_slice() == [255, 1, 221, 199, 1, 255, 255, 255, 255, 7]);
    }

    #[test]
    fn negative_big() {
        let mut buffer = Vec::with_capacity(30);
        VarLong::write_varlong(&mut buffer, -1);
        VarLong::write_varlong(&mut buffer, -2);
        VarLong::write_varlong(&mut buffer, -127);
        assert!(buffer.as_slice() == [255, 255, 255, 255, 255, 255, 255, 255, 255, 1, 254, 255, 255, 255, 255, 255, 255, 255, 255, 1, 129, 255, 255, 255, 255, 255, 255, 255, 255, 1]);
    }

    #[test]
    fn negative_small() {
        let mut buffer = Vec::with_capacity(30);
        VarLong::write_varlong(&mut buffer, -255);
        VarLong::write_varlong(&mut buffer, -25565);
        VarLong::write_varlong(&mut buffer, -2147483648);
        assert!(buffer.as_slice() == [129, 254, 255, 255, 255, 255, 255, 255, 255, 1, 163, 184, 254, 255, 255, 255, 255, 255, 255, 1, 128, 128, 128, 128, 248, 255, 255, 255, 255, 1]);
    }
    
}