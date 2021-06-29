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

pub struct ClientData {
    id: u32,

    name: Option<String>,
}


fn xyz()
{
	let port = 6969;
	let addr = enet::Address::new(std::net::Ipv4Addr::new(127, 0, 0, 1), port);
	let socket = std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(127, 0, 0, 1), port);


	let enetapi = enet::Enet::new().unwrap();

	let mut client = enetapi.create_host::<u32>(None, 4, enet::ChannelLimit::Maximum, enet::BandwidthLimit::Unlimited, enet::BandwidthLimit::Unlimited).unwrap();

	client.connect(&addr, 4, 1).unwrap();

	let e = client.service(1000).unwrap();

    
}
