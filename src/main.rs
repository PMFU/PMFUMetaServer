use std::borrow::BorrowMut;
use std::io::Read;
use std::collections::HashMap;

use enet::Enet;

#[allow(non_snake_case)]
#[allow(unused_variables)]
#[allow(array_into_iter)]
#[allow(dead_code)]

mod connection_routing;

fn main() 
{
	println!("Hello, world!");

	//  Actually start the basic testing

	let port = 6969;
	let socket = std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(127, 0, 0, 1), port);

	let mut peervec = Vec::new();

	/*
	*	Essentially, what I need to figure out here is both how to write decent Rust
	*	and how to use the networking libriary which is a decent library ngl
	*
	*	Here, what I am testing is if I can have a loop for looking for new connections
	*	and then "routing" them to something, or rather, an actual TCP socket connection I guess?
	*/

	//let clientdata: json::Array;
	
	//let enetapi = enet::Enet::new().unwrap();
	/*let server = enetapi.create_host(None, 32, 
		enet::ChannelLimit::Limited(2), enet::BandwidthLimit::Unlimited, 
		enet::BandwidthLimit::Unlimited).unwrap();

	*/

	let _input_stream = std::net::TcpListener::bind(socket).unwrap();



	for packet in _input_stream.incoming() 
	{
		match packet 
		{
			Ok(mut data) => 
			{
				println!("Connection Succeeded");

				let mut datareceived = String::new();
				let peeraddr = data.peer_addr().unwrap();
				//let peersocket = std::net::SocketAddr::new(peeraddr.ip(), port);

				data.read_to_string(&mut datareceived).unwrap();

				println!("{}", datareceived);

				
				peervec.push(std::net::TcpStream::connect(peeraddr).unwrap());

				//let player = connection_routing::Player::new(peersocket, "name", 0);
				
																//For some godforsaken reason this is a closure?
				//user_map.entry(PeerID::newWithName(get_user_id(datareceived))).or_insert_with(|| player);
			}

			Err(_e) => 
			{
				println!("ERROR! Connection Failed");
			}
		}

		for peer in peervec
		{
			// peer
		}
	}
}

fn get_user_id(str: String) -> String 
{
	let mut string = String::new();

	let whitespace = " \n\t";

	for character in str.chars()
	{
		if whitespace.contains(character) 
		{
			return string;
		} 
		else 
		{
			string.push(character);
		}
	}

	string
}
