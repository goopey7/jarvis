use serenity::builder;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

use std::fs::File;
use std::io::prelude::*;

pub fn register(
	command: &mut builder::CreateApplicationCommand,
) -> &mut builder::CreateApplicationCommand
{
	command
		.name("update-cookie")
		.description("Update the chatgpt cookie")
		.create_option(|option|
		{
			option
				.name("cookie")
				.description("chatgpt cookie from browser")
				.kind(CommandOptionType::String)
				.required(true)
		})
}

fn write_cookie_to_file(cookie: &str) -> Result<(), std::io::Error>
{
	let mut file = File::create("cookie.txt")?;
	file.write_all(cookie.as_bytes())?;
	Ok(())
}

pub fn run(_options: &[CommandDataOption]) -> String
{
	let cookie = _options[0].value.as_ref().unwrap().as_str().unwrap();
	write_cookie_to_file(cookie).unwrap();
	format!("Cookie updated to {}", cookie)
}

