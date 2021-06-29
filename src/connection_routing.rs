

pub fn get_user_id(str: String) -> String 
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

pub struct ClientData
{
	id: u32,

	name : Option<String>,
}

