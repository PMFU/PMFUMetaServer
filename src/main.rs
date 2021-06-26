use std::io::Read;

#[allow(non_snake_case)]
#[allow(unused_variables)]
#[allow(array_into_iter)]
#[allow(dead_code)]

mod connection_routing;

fn main() {
	println!("Hello, world!");

	//  Actually start the basic testing

	let port = 6969;
	let socket = std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(127, 0, 0, 1), port);

	

	/*
	*	Essentially, what I need to figure out here is both how to write decent Rust
	*	and how to use the networking libriary which is a decent library ngl
	*
	*	Here, what I am testing is if I can have a loop for looking for new connections
	*	and then "routing" them to something, or rather, an actual TCP socket connection I guess?
	*/
	let _input_stream = std::net::TcpListener::bind(socket).unwrap();

	_input_stream.accept().unwrap();

	for packet in _input_stream.incoming() 
	{
		match packet 
		{
			Ok(mut data) => 
			{
				println!("Connection Succeeded");

				let mut datareceived = String::new();

				data.read_to_string(&mut datareceived).unwrap();

				let peeraddr = data.peer_addr().unwrap();

				let peersocket = std::net::SocketAddr::new(peeraddr.ip(), port);

				println!("{}", datareceived);

				let player = connection_routing::Player::new(peersocket, "name", 0);
			}

			Err(e) => 
			{
				println!("ERROR! Connection Failed");
			}
		}
	}
}

fn getUserID(str: String) -> String 
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
