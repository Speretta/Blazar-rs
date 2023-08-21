use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::{
    raw_packet::{
        packet::{RawPacketCreator, RawPacketField},
        reader::RawPacketFieldRead,
        writer::RawPacketFieldWrite,
    },
    util::uid::UUID,
};

pub(super) struct PlayerHandler {
    stream: TcpStream,
    status: Status,
}

impl PlayerHandler {
    pub(super) fn new(stream: TcpStream, status: String) -> Self {
        PlayerHandler {
            stream,
            status: Status::STATUS(status),
        }
    }

    pub(super) async fn process(&mut self) {
        loop {
            let mut buffer = [0; 64];
            if let Ok(n) = self.stream.read(&mut buffer).await {
                if n < 3 {
                    continue;
                }
                let packet_id = RawPacketCreator::new_reader().build(&buffer);
                match packet_id.get_packet_id() {
                    0x00 => match &self.status {
                        Status::STATUS(status_message) => {
                            let status_packet = RawPacketCreator::new_reader()
                                .add_field(RawPacketFieldRead::VARINT)
                                .add_field(RawPacketFieldRead::STRING)
                                .add_field(RawPacketFieldRead::USHORT)
                                .add_field(RawPacketFieldRead::VARINT)
                                .build(&buffer);

                            if let Some(RawPacketField::VARINT(status_or_login)) =
                                status_packet.get_field(3)
                            {
                                if *status_or_login == 0x01 {
                                    let packet = RawPacketCreator::new_writer(0x00)
                                        .add_field(RawPacketFieldWrite::STRING(
                                            status_message.clone(),
                                        ))
                                        .build();
                                    self.stream.write(&packet.to_bytes()).await.unwrap();
                                } else if *status_or_login == 0x02 {
                                    self.status = Status::LOGIN;
                                    let length = status_packet.to_bytes().len(); //It's not the best way
                                    let login_request_packet = RawPacketCreator::new_reader()
                                        .add_field(RawPacketFieldRead::STRING)
                                        .add_field(RawPacketFieldRead::BOOL)
                                        .add_field(RawPacketFieldRead::UUID)
                                        .build(&buffer[length..]);

                                    let login_success_packet = RawPacketCreator::new_writer(0x02)
                                        .add_field(
                                            login_request_packet.get_field(2).unwrap().into(),
                                        )
                                        .add_field(
                                            login_request_packet.get_field(0).unwrap().into(),
                                        )
                                        .add_field(RawPacketFieldWrite::VARINT(0))
                                        .build();
                                    self.stream
                                        .write(&login_success_packet.to_bytes())
                                        .await
                                        .unwrap();
                                    if let Some(RawPacketField::UUID(uuid)) =
                                        login_request_packet.get_field(2)
                                    {
                                        self.status = Status::PLAY(*uuid);

                                        let spawn_enity_packet = RawPacketCreator::new_writer(0x28)
                                            .add_field(RawPacketFieldWrite::VARINT(0)) //Entity ID
                                            .add_field(RawPacketFieldWrite::BOOL(false)) //Is hardcore
                                            .add_field(RawPacketFieldWrite::UBYTE(1)) //Game mode
                                            .add_field(RawPacketFieldWrite::BYTE(0)) //Previous Game Mode
                                            .add_field(RawPacketFieldWrite::VARINT(1)) //Dimension count
                                            .add_field(RawPacketFieldWrite::STRING(String::from(
                                                "minecraft:world",
                                            ))) //?Dimension?
                                            .add_field(RawPacketFieldWrite::BYTE(0))
                                            .add_field(RawPacketFieldWrite::BYTE(0))
                                            .add_field(RawPacketFieldWrite::STRING(String::from(
                                                "World",
                                            )))
                                            .add_field(RawPacketFieldWrite::LONG(3619)) //First 8 bytes of world's seed
                                            .add_field(RawPacketFieldWrite::VARINT(5)) //Max players
                                            .add_field(RawPacketFieldWrite::VARINT(12)) //View distance
                                            .add_field(RawPacketFieldWrite::VARINT(12)) //Simulation distance
                                            .add_field(RawPacketFieldWrite::BOOL(false)) //Reduced debug info
                                            .add_field(RawPacketFieldWrite::BOOL(true)) //Enable respawn screen
                                            .add_field(RawPacketFieldWrite::BOOL(false)) //Is debug
                                            .add_field(RawPacketFieldWrite::BOOL(true)) //Is flat
                                            .add_field(RawPacketFieldWrite::BOOL(false)) //Has death location
                                            .add_field(RawPacketFieldWrite::VARINT(20)) //Portal cooldown ticks
                                            .build();
                                        dbg!(&spawn_enity_packet.to_bytes());
                                        self.stream
                                            .write(&spawn_enity_packet.to_bytes())
                                            .await
                                            .unwrap();
                                    }
                                }
                            }
                        }
                        Status::LOGIN => {
                            let packet = RawPacketCreator::new_reader()
                                .add_field(RawPacketFieldRead::STRING)
                                .add_field(RawPacketFieldRead::BOOL)
                                .add_field(RawPacketFieldRead::LONG)
                                .add_field(RawPacketFieldRead::LONG)
                                .build(&buffer);
                            dbg!(packet);
                        }
                        Status::PLAY(uuid) => {}
                    },
                    0x01 => {
                        let packet = RawPacketCreator::new_reader()
                            .add_field(RawPacketFieldRead::LONG)
                            .build(&buffer);
                        self.stream.write(&packet.to_bytes()).await.unwrap();
                    }

                    _ => {}
                }
            } else {
                break;
            }
        }
        println!(
            "Connection closed for {:?}",
            self.stream.peer_addr().unwrap()
        );
    }
}

#[derive(Debug)]
enum Status {
    STATUS(String),
    LOGIN,
    PLAY(UUID),
}
