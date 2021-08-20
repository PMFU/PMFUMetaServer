#[allow(non_snake_case)]
#[allow(unused_variables)]
#[allow(array_into_iter)]
#[allow(dead_code)]
#[allow(unused_imports)]
use std::collections::HashMap;

use enet::Packet;
use igd::SearchOptions;
use json::JsonValue;

use crate::packet_enums::{packet_to_type, PacketType};

#[derive(Clone, Debug)]
pub struct ClientData {
    id: u32,

    name: Option<String>,
}

pub fn send_game_list_packet(games: &HashMap<u32, Lobby>, client: &mut enet::Peer<u32>) {
    let mut packet = JsonValue::new_object();

    for (id, game) in games {
        let mut gamejson = JsonValue::new_object();

        gamejson["lobbyname"] = JsonValue::String(game.lobby_name.to_owned());
        gamejson["checksum"] = JsonValue::String(game.checksum.to_owned());

        packet[json::stringify(id.to_owned())] = gamejson.into();
    }

    let mut str = format!("{:?}\n", PacketType::RequestServerList);
    str.push_str(packet.pretty(1).as_str());

    let data_packet = Packet::new(str.as_bytes(), enet::PacketMode::ReliableSequenced).unwrap();

    client.send_packet(data_packet, 0).unwrap();

    println!("\nGame Data List Packet: \n{}\n", str);
}

pub fn handle_packet(
    sender: &mut enet::Peer<u32>,
    packet: &mut Packet,
    channel_id: u8,
    games: &HashMap<u32, Lobby>,
) {
    let PacketType = packet_to_type(packet);

    match PacketType {
        PacketType::None => {}

        PacketType::RequestServerList => {
            send_game_list_packet(games, sender);
        }

        PacketType::LobbyData => {}

        PacketType::SyncData => {}

        PacketType::NumTypes => {}
    }
}

#[derive(Clone)]
pub struct Lobby {
    id: u32,

    host_ip: std::net::Ipv4Addr,

    lobby_name: String,
    password: Option<String>,

    checksum: String,
}

impl Lobby {
    pub fn new(host_ip: std::net::Ipv4Addr, lobby_name: String, password: Option<String>) -> Self {
        let generated_id;
        generated_id = 321684210;

        let chsum = "1111".to_owned();

        Self {
            id: generated_id,
            host_ip,
            lobby_name,
            password,
            checksum: chsum,
        }
    }

    pub fn serialize(&mut self) -> String {
        /*let mut string = String::new();

        string.push_str(self.lobby_name.as_str());
        string.push('\n');
        string.push_str(self.checksum.as_str());*/

        let mut j = JsonValue::new_object();

        j["lobbyname"] = JsonValue::String(self.lobby_name.to_owned());
        j["ip"] = JsonValue::String(self.get_ip().to_string());
        j["checksum"] = JsonValue::String(self.checksum.to_owned());
        j["id"].as_u32().insert(self.id);

        j.dump()
    }

    pub fn get_ip(&self) -> std::net::Ipv4Addr {
        self.host_ip
    }

    pub fn get_id(&mut self) -> &mut u32 {
        &mut self.id
    }
}

//Other stuff

pub fn get_user_id(str: String) -> String {
    let mut string = String::new();

    let whitespace = " \n\t";

    for character in str.chars() {
        if whitespace.contains(character) {
            return string;
        } else {
            string.push(character);
        }
    }

    string
}

pub fn open_port(port: u16) {
    //OPEN THE PORTS
    match igd::search_gateway(Default::default()) {
        Err(ref err) => println!("Error: {}", err),
        Ok(gateway) => {
            let local_address = match std::env::args().nth(1) {
                Some(local_address) => local_address,
                None => panic!("Expected IP address (cargo run <your IP here>)"),
            };
            let local_address = local_address.parse::<std::net::Ipv4Addr>().unwrap();
            let local_address = std::net::SocketAddrV4::new(local_address, port);

            match gateway.add_port(
                igd::PortMappingProtocol::UDP,
                port,
                local_address,
                600000,
                "add_port example",
            ) {
                Err(ref err) => {
                    println!("There was an error! {}", err);
                }
                Ok(()) => {
                    println!("It worked");
                }
            }
        }
    }
}
