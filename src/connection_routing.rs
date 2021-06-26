
#[derive(Debug, Hash)]
pub struct Player {
	socket: std::net::SocketAddr,
	name: String,
	id: u64,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct PeerID 
{
	name: String,
}

impl Player 
{
	pub fn new(socket: std::net::SocketAddr, name: &str, id: u64) -> Self 
	{
		let string : String = name.into();
		Self 
		{
			socket, 
			name : string, 
			id 
		}
	}
}

impl PeerID
{
	pub fn new() -> Self
	{
		Self
		{
			name :	"name".to_string(),
		}
	}
	
	pub fn newWithName(name: String) -> Self
	{
		// let name2 = name.to_owned();
		Self
		{
			name
		}
	}
}