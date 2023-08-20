use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::raw_packet::{
    packet::{RawPacketCreator, RawPacketField},
    reader::RawPacketFieldRead,
    writer::RawPacketFieldWrite,
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
                if let Status::LOGIN = self.status {
                    dbg!(&self.status);
                }
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
                                    let packet = RawPacketCreator::new_reader()
                                        .add_field(RawPacketFieldRead::STRING)
                                        .add_field(RawPacketFieldRead::BOOL)
                                        .add_field(RawPacketFieldRead::UUID)
                                        .build(&buffer[length..]);
                                    dbg!(&packet);
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
}
