#[allow(non_snake_case)]
#[allow(unused_variables)]
#[allow(array_into_iter)]
#[allow(dead_code)]
use std::collections::HashMap;

use enet::Packet;
use json::JsonValue;

pub struct ClientData {
	id: u32,

	name: Option<String>,
}

pub fn send_game_list_packet(games: HashMap<u32, Game>, client: &mut enet::Peer<u32>) {
	let mut packet = JsonValue::new_array();

	for (id, game) in games {
		let mut gamejson = JsonValue::new_object();

		gamejson["lobbyname"] = JsonValue::String(game.lobby_name);
		gamejson["checksum"] = JsonValue::String(game.checksum);

		packet[json::stringify(id)] = gamejson.into();
	}

	let str = packet.pretty(1);

	let data_packet = Packet::new(str.as_bytes(), enet::PacketMode::ReliableSequenced).unwrap();

	client.send_packet(data_packet, 0).unwrap();
}

pub struct Game {
	id: u32,

	host_ip: std::net::Ipv4Addr,

	lobby_name: String,
	password: Option<String>,

	checksum: String,
}

impl Game {
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

	pub fn serialize(self) -> String {
		let mut string = String::new();

		string.push_str(self.lobby_name.as_str());
		string.push('\n');
		string.push_str(self.checksum.as_str());

		string
	}

	pub fn get_ip(self) -> std::net::Ipv4Addr {
		self.host_ip
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
