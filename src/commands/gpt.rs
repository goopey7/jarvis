use serenity::builder;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

use std::fs::File;
use std::io::prelude::*;

use tokio;
use reqwest;
use reqwest::header::*;

use serde_json::{Result, Value};

pub fn register(
	command: &mut builder::CreateApplicationCommand,
) -> &mut builder::CreateApplicationCommand
{
	command
		.name("gpt")
		.description("send a message to chatgpt")
		.create_option(|option|
		{
			option
				.name("prompt")
				.description("What do you want to tell chatgpt?")
				.kind(CommandOptionType::String)
				.required(true)
		})
}

fn get_auth_from_file() -> String
{
	let mut file = File::open("auth.txt").expect("Unable to open auth.txt");
	let mut contents = String::new();
	file.read_to_string(&mut contents)
		.expect("Unable to read auth.txt");

	contents.trim().to_string()
}

fn get_cookie_from_file() -> String
{
	let mut file = File::open("cookie.txt").expect("Unable to open cookie.txt");
	let mut contents = String::new();
	file.read_to_string(&mut contents)
		.expect("Unable to read cookie.txt");

	contents.trim().to_string()
}

/*
async fn get_response(prompt: &str) -> Result<reqwest::Response, reqwest::Error>
{
	let auth_token = get_auth_from_file();
	let cookie = get_cookie_from_file();
	let url = "https://chat.openai.com/backend-api/conversation";

	let client = reqwest::Client::new();

	let mut headers = HeaderMap::new();

	headers.insert(Host::new("chat.openai.com"));
	headers.insert(UserAgent::new("Mozilla/5.0 (X11; Linux x86_64; rv:107.0) Gecko/20100101 Firefox/107.0"));
	headers.insert(Accept::new("text/event-stream"));
	headers.insert(AcceptLanguage::new("en-US,en;q=0.5"));
	headers.insert(AcceptEncoding::new("gzip, deflate, br"));
	headers.insert(Referer::new("https://chat.openai.com/chat"));
	headers.insert(ContentType::json());
	headers.insert("X-OpenAI-Assistant-App-Id", HeaderValue::from_static(""));
	headers.insert("Authorization", HeaderValue::from_static(format!("Bearer {}", auth_token).as_str()));
	headers.insert("Content-Length", HeaderValue::from_str("329").unwrap());
	headers.insert("Origin", HeaderValue::from_static("https://chat.openai.com"));
	headers.insert("Alt-Used", HeaderValue::from_static("chat.openai.com"));
	headers.insert("Connection", HeaderValue::from_static("keep-alive"));
	headers.insert("Cookie", HeaderValue::from_static(cookie.as_str()));
	headers.insert("Sec-Fetch-Dest", HeaderValue::from_static("empty"));
	headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("cors"));
	headers.insert("Sec-Fetch-Site", HeaderValue::from_static("same-origin"));

	let response = client.post(url)
		.headers(headers)
		.body(body)
		.send()
		.await?;

	Ok(response)
}
*/

fn get_penultimate_data(input: String) -> Option<String>
{
	let data_count = input.matches("data:").count();

	if data_count >= 2
	{
		let data_lines: Vec<&str> = input.split("data:").collect();
		let penultimate_data = data_lines[data_count - 1];
		Some(format!("{}", penultimate_data))
	}
	else
	{
		None
	}
}

fn parse_response(response: String) -> String
{
	let data = get_penultimate_data(response);
	match data
	{
		Some(data) =>
		{
			let json = serde_json::from_str::<Value>(data.as_str()).unwrap();
			let response = json["message"]["content"]["parts"][0].to_string();
			response[1..response.len() - 1].to_string()
		}
		None => "No response".to_string(),
	}
}

pub fn run(_options: &[CommandDataOption]) -> String
{
	let message = _options[0].value.as_ref().unwrap().as_str().unwrap();
	//let response = get_response(message).await.unwrap();
	format!("{}", parse_response(message.to_string()).replace("\\n","\n"))
}

