use std::sync::Arc;

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use crate::raw_packet::packet::{RawPacket, RawPacketCreator, RawPacketField};
use crate::raw_packet::reader::RawPacketFieldRead;
use crate::raw_packet::writer::RawPacketFieldWrite;

pub struct MinecraftServer {}

impl MinecraftServer {
    pub fn new() -> Self {
        MinecraftServer {}
    }

    pub async fn start<S: std::fmt::Display>(self, ip_adress: S, port: u16) {
        let listener = TcpListener::bind(format!("{}:{}", ip_adress, port))
            .await
            .unwrap();

        loop {
            // The second item contains the IP and port of the new connection.
            let (socket, _) = listener.accept().await.unwrap();

            let ip_adress = format!("{}", ip_adress);
            tokio::spawn(async move {
                dbg!(&socket);
                MinecraftServer::process(socket, ip_adress, port).await;
            });
        }

        //todo!()
    }

    async fn process<S: std::fmt::Display>(mut socket: TcpStream, ip_adress: S, port: u16) {
        loop {
            let mut buffer = [0; 64];
            if let Ok(n) = socket.read(&mut buffer).await {
                if n < 5 {
                    break;
                }
                let packet = RawPacketCreator::new_reader()
                    .add_field(RawPacketFieldRead::VARINT)
                    .add_field(RawPacketFieldRead::STRING)
                    .add_field(RawPacketFieldRead::USHORT)
                    .add_field(RawPacketFieldRead::VARINT)
                    .build(&buffer);
                if let Some(RawPacketField::VARINT(value)) = packet.get_field(3) {
                    match value {
                        0x01 => {
                            //Client wants status info
                            let packet = RawPacketCreator::new_writer(0x00)
                                .add_field(RawPacketFieldWrite::STRING(String::from(
                                    r#"{
                                    "version": {
                                        "name": "1.20.1",
                                        "protocol": 763
                                    },
                                    "players": {
                                        "max": 100,
                                        "online": 5,
                                        "sample": [
                                            {
                                                "name": "thinkofdeath",
                                                "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d20"
                                            }
                                        ]
                                    },
                                    "description": {
                                        "text": "Hello world"
                                    },
                                    "favicon": "data:image/png;base64,<data>",
                                    "enforcesSecureChat": true,
                                    "previewsChat": true
                                }"#,
                                )))
                                .build();
                            socket.write(&packet.to_bytes()).await.unwrap();
                            socket.read(&mut buffer).await.unwrap();
                        }
                        0x02 => {
                            //Client wants login

                            socket.read(&mut buffer).await.unwrap();
                            let packet = RawPacketCreator::new_reader()
                                .add_field(RawPacketFieldRead::STRING)
                                .add_field(RawPacketFieldRead::BOOL)
                                .add_field(RawPacketFieldRead::LONG) //FOR UUID 1
                                .add_field(RawPacketFieldRead::LONG) //FOR UUID 2
                                .build(&buffer);
                            dbg!(packet);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
