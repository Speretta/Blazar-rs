use crate::raw_packet::{
    packet::RawPacketCreator, reader::RawPacketFieldRead, writer::RawPacketFieldWrite,
};

#[test]
fn creation_reader() {
    let buffer = [1, 0];
    let packet = RawPacketCreator::new_reader().build(&buffer);
    assert!(packet.to_bytes() == buffer)
}

#[test]
fn big_reader() {
    let buffer = [
        16, 0, 251, 5, 9, 108, 111, 99, 97, 108, 104, 111, 115, 116, 99, 221, 1,
    ];
    let packet = RawPacketCreator::new_reader()
        .add_field(RawPacketFieldRead::VarInt)
        .add_field(RawPacketFieldRead::String)
        .add_field(RawPacketFieldRead::UShort)
        .add_field(RawPacketFieldRead::VarInt)
        .build(&buffer);
    assert!(packet.to_bytes() == buffer)
}

#[test]
fn creation_writer() {
    let buffer = [1, 0];
    let packet = RawPacketCreator::new_writer(0).build();
    assert!(packet.to_bytes() == buffer)
}

#[test]
fn big_writer() {
    let buffer = [
        16, 0, 251, 5, 9, 108, 111, 99, 97, 108, 104, 111, 115, 116, 99, 221, 1,
    ];
    let packet = RawPacketCreator::new_writer(0)
        .add_field(RawPacketFieldWrite::VarInt(763))
        .add_field(RawPacketFieldWrite::String(String::from("localhost")))
        .add_field(RawPacketFieldWrite::UShort(25565))
        .add_field(RawPacketFieldWrite::VarInt(1))
        .build();
    assert!(packet.to_bytes() == buffer)
}
