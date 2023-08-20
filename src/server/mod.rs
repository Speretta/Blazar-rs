mod playerhandler;
pub mod properties;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;

use crate::raw_packet::packet::{RawPacketCreator, RawPacketField};
use crate::raw_packet::reader::RawPacketFieldRead;
use crate::raw_packet::writer::RawPacketFieldWrite;

use self::playerhandler::PlayerHandler;
use self::properties::ServerProperties;

static mut LOGIN: bool = false;

#[derive(Debug, Clone)]
pub struct MinecraftServer {
    server_properties: ServerProperties,
}

impl MinecraftServer {
    pub fn new(server_properties: ServerProperties) -> Self {
        MinecraftServer { server_properties }
    }

    pub async fn start(self) {
        let listener = TcpListener::bind(self.server_properties.server_ip)
            .await
            .unwrap();

        loop {
            let (socket, _) = listener.accept().await.unwrap();
            let mut player_handler = PlayerHandler::new(socket, self.get_status_text());
            tokio::spawn(async move {
                println!("Connection created!");
                player_handler.process().await;
            });
        }
    }

    fn get_status_text(&self) -> String {
        format!(
            r#"{{
            "version": {{
                "name": "{}",
                "protocol": {}
            }},
            "players": {{
                "max": {},
                "online": {},
                "sample": [
                    {{
                        "name": "{}",
                        "id": "{}"
                    }}
                ]
            }},
            "description": {{
                "text": "{}"
            }},
            "favicon": "data:image/png;base64,<data>",
            "enforcesSecureChat": true,
            "previewsChat": true
        }}"#,
            self.server_properties.version_name,
            self.server_properties.protocol_version,
            self.server_properties.max_player,
            1,
            "Speretta",
            "4566e69f-c907-48ee-8d71-d7ba5aa00d20",
            self.server_properties.motd
        )
    }
}
