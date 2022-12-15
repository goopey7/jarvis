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
		.name("update-authentication")
		.description("Update the chatgpt auth token")
		.create_option(|option|
		{
			option
				.name("token")
				.description("chatgpt auth token from browser")
				.kind(CommandOptionType::String)
				.required(true)
		})
}

fn write_auth_to_file(token: &str) -> Result<(), std::io::Error>
{
	let mut file = File::create("auth.txt")?;
	file.write_all(token.as_bytes())?;
	Ok(())
}

pub fn run(_options: &[CommandDataOption]) -> String
{
	let auth_token = _options[0].value.as_ref().unwrap().as_str().unwrap();
	write_auth_to_file(auth_token).unwrap();
	format!("Updated auth token to {}", auth_token)
}

