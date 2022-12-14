use std::fs::File;
use std::io::prelude::*;

fn get_auth_from_file() -> String
{
	let mut file = File::open("auth.txt").expect("Failed to open file");
	let mut contents = String::new();
	file.read_to_string(&mut contents)
		.expect("Failed to read from file");

	contents.trim().to_string()
}

fn get_cookie_from_file() -> String
{
	let mut file = File::open("cookie.txt").expect("Failed to open file");
	let mut contents = String::new();
	file.read_to_string(&mut contents)
		.expect("Failed to read from file");

	contents.trim().to_string()
}

fn main()
{
	let auth_token = get_auth_from_file();
	let cookie = get_cookie_from_file();
	println!("The auth token is: {}", auth_token);
	println!("The cookie is: {}", cookie);
}

