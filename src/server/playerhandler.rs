use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::{raw_packet::{
    packet::{RawPacketCreator, RawPacketField},
    reader::RawPacketFieldRead,
    writer::RawPacketFieldWrite,
}, util::uid::UUID};

pub(super) struct PlayerHandler {
    stream: TcpStream,
    status: Status,
}

impl PlayerHandler {
    pub(super) fn new(stream: TcpStream, status: String) -> Self {
        PlayerHandler {
            stream,
            status: Status::Status(status),
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
                        Status::Status(status_message) => {
                            let status_packet = RawPacketCreator::new_reader()
                                .add_field(RawPacketFieldRead::VarInt)
                                .add_field(RawPacketFieldRead::String)
                                .add_field(RawPacketFieldRead::UShort)
                                .add_field(RawPacketFieldRead::VarInt)
                                .build(&buffer);

                            if let Some(RawPacketField::VarInt(status_or_login)) =
                                status_packet.get_field(3)
                            {
                                if *status_or_login == 0x01 {
                                    let packet = RawPacketCreator::new_writer(0x00)
                                        .add_field(RawPacketFieldWrite::String(
                                            status_message.clone(),
                                        ))
                                        .build();
                                    self.stream.write_all(&packet.to_bytes()).await.unwrap();
                                } else if *status_or_login == 0x02 {
                                    self.status = Status::Login;
                                    let length = status_packet.to_bytes().len(); //It's not the best way
                                    let login_request_packet = RawPacketCreator::new_reader()
                                        .add_field(RawPacketFieldRead::String)
                                        .add_field(RawPacketFieldRead::Bool)
                                        .add_field(RawPacketFieldRead::Uuid)
                                        .build(&buffer[length..]);

                                    let login_success_packet = RawPacketCreator::new_writer(0x02)
                                        .add_field(login_request_packet.get_field(2).unwrap().into())
                                        .add_field(login_request_packet.get_field(0).unwrap().into())
                                        .add_field(RawPacketFieldWrite::VarInt(0))
                                        .build();
                                    self.stream.write_all(&login_success_packet.to_bytes()).await.unwrap();
                                    if let Some(RawPacketField::Uuid(uuid)) = login_request_packet.get_field(2){
                                        self.status = Status::Play(*uuid);

                                        let spawn_enity_packet = RawPacketCreator::new_writer(0x28)
                                            .add_field(RawPacketFieldWrite::VarInt(0)) //Entity ID
                                            .add_field(RawPacketFieldWrite::Bool(false)) //Is hardcore
                                            .add_field(RawPacketFieldWrite::UByte(1)) //Game mode
                                            .add_field(RawPacketFieldWrite::Byte(0)) //Previous Game Mode
                                            .add_field(RawPacketFieldWrite::VarInt(1))//Dimension count
                                            .add_field(RawPacketFieldWrite::String(String::from("minecraft:overworld"))) //?Dimension?
                                            .add_field(RawPacketFieldWrite::Byte(0))
                                            .add_field(RawPacketFieldWrite::Byte(0))
                                            .add_field(RawPacketFieldWrite::String(String::from("World")))
                                            .add_field(RawPacketFieldWrite::Long(3619)) //First 8 bytes of world's seed
                                            .add_field(RawPacketFieldWrite::VarInt(5)) //Max players
                                            .add_field(RawPacketFieldWrite::VarInt(12)) //View distance
                                            .add_field(RawPacketFieldWrite::VarInt(12)) //Simulation distance
                                            .add_field(RawPacketFieldWrite::Bool(false)) //Reduced debug info
                                            .add_field(RawPacketFieldWrite::Bool(true)) //Enable respawn screen
                                            .add_field(RawPacketFieldWrite::Bool(false)) //Is debug
                                            .add_field(RawPacketFieldWrite::Bool(true)) //Is flat
                                            .add_field(RawPacketFieldWrite::Bool(false)) //Has death location
                                            .add_field(RawPacketFieldWrite::VarInt(20)) //Portal cooldown ticks
                                            .build();
                                        dbg!(&spawn_enity_packet.to_bytes());
                                        self.stream.write_all(&spawn_enity_packet.to_bytes()).await.unwrap();
                                    }
                                    
                                }
                            }
                        }
                        Status::Login => {
                            let packet = RawPacketCreator::new_reader()
                                .add_field(RawPacketFieldRead::String)
                                .add_field(RawPacketFieldRead::Bool)
                                .add_field(RawPacketFieldRead::Long)
                                .add_field(RawPacketFieldRead::Long)
                                .build(&buffer);
                            dbg!(packet);
                        }
                        Status::Play(uuid) =>{

                        },
                    },
                    0x01 => {
                        let packet = RawPacketCreator::new_reader()
                            .add_field(RawPacketFieldRead::Long)
                            .build(&buffer);
                        self.stream.write_all(&packet.to_bytes()).await.unwrap();
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
    Status(String),
    Login,
    Play(UUID),
}
