//use std::collections::HashMap;
use std::io::{Read, Write};

use enet::Event;

#[allow(non_snake_case)]
#[allow(unused_variables)]
#[allow(array_into_iter)]
#[allow(dead_code)]
mod connection_routing;

fn main() {
    println!("Hello, world!");

    //  Actually start the basic testing

    let port = 6969;
    let addr = enet::Address::new(std::net::Ipv4Addr::new(127, 0, 0, 1), port);
    let _socket = std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(127, 0, 0, 1), port);

    let enetapi = enet::Enet::new().unwrap();

    let mut server = enetapi
        .create_host::<u32>(
            Some(&addr),
            100,
            enet::ChannelLimit::Maximum,
            enet::BandwidthLimit::Unlimited,
            enet::BandwidthLimit::Unlimited,
        )
        .unwrap();

    /*
    	*	Essentially, what I need to figure out here is both how to write decent Rust
    	*	and how to use the networking libriary which is a decent library ngl
    	*
    	*	Here, what I am testing is if I can have a loop for looking for new connections
    	*	and then "routing" them to something, or rather, an actual TCP socket connection I guess?
    	*/

    //let clientdata: json::Array;

    let mut id = 0;

    do_update(&mut server, &mut id);
}

fn do_update(server: &mut enet::Host<u32>, id: &mut u32) {
    let event = server.service(1000).unwrap();
	
    match &mut event.unwrap() {
        Event::Connect(peer) => {
            println!(
                "Connection from peer! IP: {}",
                peer.address().ip().to_string()
            );

            let id_local = id.to_owned();
            id.checked_add(1);

            let addr = peer.address();

            server.connect(&addr, 4, id_local);
        }

        Event::Disconnect(peer, id) => {
            let local_id = id.to_owned();
            peer.disconnect(local_id);
        }

        Event::Receive {
            sender,
            channel_id,
            packet,
        } => {
            let mut str = String::new();
            packet.data().read_to_string(&mut str);

            // sender.address().ip().to_string()

            println!(
                "Data: {} | from IP: {}",
                str,
                sender.address().ip().to_string()
            );
        }
    };
}
