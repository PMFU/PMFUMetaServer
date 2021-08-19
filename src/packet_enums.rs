#[allow(unused_imports)]
#[allow(non_snake_case)]
#[allow(unused_variables)]
#[allow(array_into_iter)]
#[allow(dead_code)]
use std::io::{BufRead, Read};

use enet::Packet;
use json::JsonValue;

#[derive(enum_utils::FromStr, Debug)]
pub enum PacketType {
    None = 0,
    RequestServerList, //Has one field, "id", if 0 then give full serverlist, else, check if the ID is valid and send the lobby info
    LobbyData,         //If received, set up a new game, serde's to the vars:
    // "lobbyname", "checksum"
    SyncData = 5, //Saves and such idk fully yet
    NumTypes,
}

pub fn packet_to_type(packet: &mut Packet) -> PacketType {
    let mut data = String::new();

    packet.data().read_line(&mut data).unwrap();

    match data.parse::<PacketType>().unwrap() {
        PacketType::None => {
            return PacketType::None;
        }

        PacketType::RequestServerList => {
            return PacketType::RequestServerList;
        }

        PacketType::LobbyData => {
            return PacketType::LobbyData;
        }

        PacketType::SyncData => {
            return PacketType::SyncData;
        }

        PacketType::NumTypes => {
            return PacketType::NumTypes;
        }
    }
}

pub fn packet_to_json(packet: &mut Packet) -> JsonValue {
    let mut data = String::new();
    let num_chars = packet.data().read_line(&mut data).unwrap();

    //I just wanna remove the first line of the string from the packet data
    //And turn that into a JSON

    let (_left, right) = packet.data().split_at(num_chars);

    let packet_stream = right.escape_ascii().to_string();

    let j = json::parse(&packet_stream).expect("Packet Stream failed to convert to JSON!");

    j
}
