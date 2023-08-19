use crate::raw_packet::packet::RawPacket;

#[test]
fn creation() {
    assert!(RawPacket::new(0).as_bytes() == [0]);
}

#[test]
fn write_varint() {
    assert!(RawPacket::new(0).write_varint(-31).as_bytes() == [0, 225, 255, 255, 255, 15]);
}

#[test]
fn write_varlong() {
    assert!(
        RawPacket::new(0).write_varlong(-31).as_bytes()
            == [0, 225, 255, 255, 255, 255, 255, 255, 255, 255, 1]
    );
}

#[test]
fn write_string() {
    assert!(
        RawPacket::new(0).write_string("blazar-rs").as_bytes()
            == [0, 9, 98, 108, 97, 122, 97, 114, 45, 114, 115]
    );
}

#[test]
fn write_bytes() {
    assert!(RawPacket::new(0).write_be_bytes(&[1, 2, 3]).as_bytes() == [0, 1, 2, 3]);
}
