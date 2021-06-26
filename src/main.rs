#[allow(non_snake_case)]
#[allow(unused_variables)]

fn main() 
{
	println!("Hello, world!");

//  Actually start the basic testing

	let socket = std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(127, 0, 0, 1), 8081);

/*
*	Essentially, what I need to figure out here is both how to write decent Rust
*	and how to use the networking libriary which is a decent library ngl
*	
*	Here, what I am testing is if I can have a loop for looking for new connections
*	and then "routing" them to something, or rather, an actual TCP socket connection I guess?
*/
	let inputStream = std::net::TcpListener::bind(socket).unwrap();

	for packet in inputStream.incoming()
	{
		match packet
		{
			Ok(packet) =>
			{
				println!("Connection Succeeded")

//				packet.peer_addr()
			}
			
			Err(e) =>
			{
				println!("ERROR! Connection Failed");
			}
		}
	}

}
