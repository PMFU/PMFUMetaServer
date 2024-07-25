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


pub fn send_game_list_packet(games: &HashMap<u32, Lobby>, client: &mut enet::Peer<u32>) {
    let mut packet = JsonValue::new_object();

    for (id, game) in games
    {
        packet["lobbies"].push(game.to_json()).unwrap();
    }

    // let mut str = format!("{:?}\n", PacketType::RequestServerList);
    // str.push_str(packet.dump().as_str());


    let data_packet = Packet::new(packet.dump().as_bytes(), enet::PacketMode::ReliableSequenced).unwrap();

    client.send_packet(data_packet, 0).unwrap();

    println!("\nGame Data List Packet: \n{}\n", packet.dump());
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
    checksum: String,
    save_or_scenario_name: String,

    password: Option<String>,
    player_count: u32
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
            checksum: chsum,
            save_or_scenario_name: "GC_NAME".to_owned(),
            password,
            player_count: 0
        }
    }

    pub fn serialize(&self) -> String {
        
        self.to_json().dump()
    }

    pub fn to_json(&self) -> JsonValue
    {
        let mut j = JsonValue::new_object();
        j["name"] = JsonValue::String(self.lobby_name.to_owned());
        j["ip"] = JsonValue::String(self.get_ip().to_string());
        j["checksum"] = JsonValue::String(self.checksum.to_owned());
        j["id"] = self.id.into();
        j["save_or_scenario_name"] = JsonValue::String(self.save_or_scenario_name.to_owned());
        j
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
                "Opening Port to run a Bathsalts Metaserver Matchmaker",
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
