use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct ServerProperties {
    //level_seed: usize,
    //gamemode: blabla,
    //enable_command_block: bool,
    //resource_pack: blabla,
    pub server_ip: SocketAddr,
    pub motd: String,
    pub max_player: u16,

    pub version_name: String,
    pub protocol_version: u16,
    pub brand_name: String,
}

impl Default for ServerProperties {
    fn default() -> Self {
        Self {
            server_ip: SocketAddr::from(([127, 0, 0, 1], 25565)),
            motd: String::from("A Minecraft Server from Rust Lang"),
            max_player: 20,
            version_name: String::from("1.20.1"),
            protocol_version: 763,
            brand_name: String::from("Blazar-rs"),
        }
    }
}
