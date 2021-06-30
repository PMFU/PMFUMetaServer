use std::io::{BufRead, Read};

use enet::Packet;

#[allow(non_snake_case)]
#[allow(unused_variables)]
#[allow(array_into_iter)]
#[allow(dead_code)]
#[derive(enum_utils::FromStr, Debug)]
pub enum PacketType {
    None = 0,
    RequestServerList,
    LobbyData,

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

        PacketType::NumTypes => {
            return PacketType::NumTypes;
        }
    }
}
