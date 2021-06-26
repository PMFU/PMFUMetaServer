mod connection_routing
{


	pub struct Player
	{
		socket  :   std::net::SocketAddr,

		name    :   String,
		id      :   u64,

	}

	pub struct PeerID
	{
		name    :   String,
		id      :   Option<u64>,
	}


	impl Player
	{
		pub fn new(socket: std::net::SocketAddr, name: String, id: u64) -> Self
		{
			Self
			{
				socket,
				name,
				id,
			}
		}
	}

}