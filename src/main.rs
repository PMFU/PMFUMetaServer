#[allow(non_snake_case)]
#[allow(unused_variables)]
#[allow(array_into_iter)]
#[allow(dead_code)]
mod connection_routing;
mod packet_enums;

use std::{
    collections::HashMap,
    io::{stdout, Read, Write},
};

use connection_routing::Game;
use enet::{Event, Packet, PeerState};

use crate::packet_enums::PacketType;

fn main() {
    println!("Hello, world!");

    stdout().flush().unwrap();

    println!("{:?}", PacketType::RequestServerList);

    //client_run();
    server_run();
}

fn do_update(server: &mut enet::Host<u32>, top_id: &mut u32, game_map: &mut HashMap<u32, Game>) {
    let event = server.service(100).unwrap();

    if event.is_none() {
        return;
    }

    match &mut event.unwrap() {
        Event::Connect(peer) => {
            println!(
                "Connection from peer! IP: {}",
                peer.address().ip().to_string()
            );

            *top_id += 1;

            peer.set_data(Some(1));

            //let addr = peer.address();
        }

        Event::Disconnect(peer, id) => {
            println!(
                "User {}, from IP {}, disconnected",
                id,
                peer.address().ip().to_string()
            );

            *top_id -= 1;

            peer.set_data(None);
        }

        Event::Receive {
            sender,
            channel_id,
            packet,
        } => {
            let mut str = String::new();
            packet.data().read_to_string(&mut str).unwrap();

            str = str.trim_end().to_string();

            // sender.address().ip().to_string()
            println!("From channel: {} ", channel_id);

            println!(
                "Data: {} \nFrom IP: {}",
                str,
                sender.address().ip().to_string()
            );
        }
    };
}

//Client Run
fn client_run() {
    let port = 42069;
    let mut ipaddr = std::net::Ipv4Addr::LOCALHOST;
    ipaddr = std::net::Ipv4Addr::new(10, 0, 1, 64);
    let remote_addr = enet::Address::new(ipaddr, port);

    println!("Connecting to {}", ipaddr.to_string());

    let enetapi = enet::Enet::new().unwrap();

    //Create a connector (DOESN'T NEED IP) | Connects to server over network
    let mut client = enetapi
        .create_host::<u32>(
            None,
            1,
            enet::ChannelLimit::Limited(4),
            enet::BandwidthLimit::Unlimited,
            enet::BandwidthLimit::Unlimited,
        )
        .unwrap();

    let peer = client.connect(&remote_addr, 4, 1).unwrap();

    if peer.state() == PeerState::Connected {
        println!("State is connected!");
    }

    let e = client.service(1000).unwrap();

    if e.is_none() {
        println!("The Connection Was Unsuccessful");
        return;
    }

    match &mut e.unwrap() {
        Event::Connect(peer) => {
            println!(
                "Connection from peer! IP: {}",
                peer.address().ip().to_string()
            );

            peer.set_data(Some(1));
        }

        Event::Disconnect(peer, id) => {}

        Event::Receive {
            sender,
            channel_id,
            packet,
        } => {}
    };

    let mut id = 0;

    let mut game_map = HashMap::<u32, Game>::new();

    loop {
        do_update(&mut client, &mut id, &mut game_map);

        //Get console input
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{} bytes read", n);
                //println!("{}", input);
            }
            Err(error) => println!("error: {}", error),
        }

        input.truncate(input.len() - 2);

        if input.is_empty() || input == "STOP" {
            break;
        }

        //Send Packet
        for mut peer in client.peers().into_iter() {
            peer.send_packet(
                Packet::new(input.as_bytes(), enet::PacketMode::ReliableSequenced).unwrap(),
                0,
            )
            .unwrap();
        }
    }

    println!("Disconnected!");
}

//Server Run
fn server_run() {
    let port = 42069;
    let ipaddr = std::net::Ipv4Addr::UNSPECIFIED;
    let local_addr = enet::Address::new(ipaddr, port);

    let enetapi = enet::Enet::new().unwrap();

    //Server init

    //OPEN THE PORTS
    match igd::search_gateway(Default::default()) {
       Err(ref err) => println!("Ln 186: Error: {}", err),
       Ok(gateway) => {
           
           let local_address = match std::env::args().nth(1) {
               Some(local_address) => local_address,
               None => panic!("Expected IP address (cargo run <your IP here>)"),
           };
           let local_address = local_address.parse::<std::net::Ipv4Addr>().unwrap();
           let local_address = std::net::SocketAddrV4::new(local_address, port);

           match gateway.add_port(igd::PortMappingProtocol::UDP, port, local_address, 60, "add_port example") {
               Err(ref err) => {
                   println!("There was an error! {}", err);
               }
               Ok(()) => {
                   println!("It worked");
               }
           }
       }
    }

    let max_peers_count = 32;

    //Create a server on the localhost
    let mut server = enetapi
        .create_host::<u32>(
            Some(&local_addr),
            max_peers_count,
            enet::ChannelLimit::Limited(4),
            enet::BandwidthLimit::Unlimited,
            enet::BandwidthLimit::Unlimited,
        )
        .unwrap();
    server.flush();

    //Data Hash Maps
    //let mut client_map = HashMap::<u32, ClientData>::new();
    let mut game_map = HashMap::<u32, Game>::new();

    //
    //for mut peer in server.peers() {}

    game_map.insert(0, Game::new(ipaddr, "lobby_name".to_owned(), None));

    //Start loop
    let mut id = 0;

    loop {
        do_update(&mut server, &mut id, &mut game_map);

        send_packet(&mut server);
    }
}

fn send_packet(h: &mut enet::Host<u32>)
{
    for mut peer in h.peers().into_iter() {
        if peer.data().is_none()
        {
            continue;
        }
        peer.send_packet(
            Packet::new("TESTING BIG MIGUEL".as_bytes(), enet::PacketMode::ReliableSequenced).unwrap(),
            0,
        )
        .unwrap();
    }

}