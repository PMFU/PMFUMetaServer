#![feature(inherent_ascii_escape)]
#[allow(non_snake_case)]
#[allow(unused_variables)]
#[allow(array_into_iter)]
#[allow(dead_code)]
#[allow(unused_imports)]
mod connection_routing;
mod packet_enums;

use std::{
    collections::HashMap,
    io::{stdout, Read, Write},
};

use connection_routing::Lobby;
use enet::{Event, Packet};

use crate::packet_enums::{packet_to_json, PacketType};

fn main() {
    println!("Hello, world!");

    stdout().flush().unwrap();

    println!("{:?}", PacketType::RequestServerList);

    //client_run();
    server_run();
}

fn do_update(server: &mut enet::Host<u32>, top_id: &mut u32, game_map: &mut HashMap<u32, Lobby>) {
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

            peer.set_data(Some(*top_id));

            *top_id += 1;
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

            println!("From channel: {} ", channel_id);

            println!(
                "Data: {} \nFrom IP: {}",
                str,
                sender.address().ip().to_string()
            );

            match packet_enums::packet_to_type(packet) {
                PacketType::None => {
                    //Nothing
                }

                PacketType::RequestServerList => {
                    //Send the server list to the requesting client
                    println!(
                        "Sending Lobby/Lobby List to user: {}",
                        sender.data().expect("No User Here")
                    );

                    let j = packet_to_json(packet);
                    if j["id"].is_empty() {
                        connection_routing::send_game_list_packet(game_map, sender);
                    } else {
                        let lobbyopt = game_map.get_mut(&j["id"].as_u32().unwrap());
                        match lobbyopt {
                            None => {
                                println!("Requested a lobby id that doesn't exist!");
                            }

                            Some(lobby) => {
                                let mut str = format!("{:?}\n", PacketType::LobbyData);

                                str.push_str(&lobby.serialize());

                                let packet_data = Packet::new(
                                    str.as_bytes(),
                                    enet::PacketMode::ReliableSequenced,
                                )
                                .unwrap();

                                sender.send_packet(packet_data, 0).unwrap();
                            }
                        }
                    }
                }

                PacketType::LobbyData => {
                    //Get the lobby data from the host sending it, and add it to the map
                    let j = packet_to_json(packet);
                    let lobbyname = j["lobbyname"].as_str().unwrap();
                    let lobbyid = j["id"].as_u32().unwrap();

                    let mut lobby = Lobby::new(*sender.address().ip(), lobbyname.to_string(), None);

                    *lobby.get_id() = lobbyid;

                    game_map.insert(*sender.data().expect("Host went missing idk"), lobby);
                }

                PacketType::SyncData => {
                    //Something with saves idk
                }

                PacketType::NumTypes => {
                    //Does nothing
                }
            }
        }
    };
}

/*//Client Run
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

    let mut id = 0;

    let mut game_map = HashMap::<u32, Lobby>::new();

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
}*/

//Server Run
fn server_run() {
    let port = 42069;
    let ipaddr = std::net::Ipv4Addr::UNSPECIFIED;
    let local_addr = enet::Address::new(ipaddr, port);

    let enetapi = enet::Enet::new().unwrap();

    //Server init
    //open_port(port);

    let max_peers_count = 64;

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

    let mut game_map = HashMap::<u32, Lobby>::new();

    //game_map.insert(0, Lobby::new(ipaddr, "lobby_name".to_owned(), None));

    //Start loop
    let mut id = 0;

    loop {
        do_update(&mut server, &mut id, &mut game_map);
    }
}
