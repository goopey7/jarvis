use serenity::builder;
use serenity::json::Value;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

use std::fs::File;
use std::io::prelude::*;

use reqwest;
use reqwest::header::*;

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

fn build_body(prompt: &str) -> String
{
	let mut json = serde_json::json!(
		{
			"action": "next",
			"messages": [{
				"id": "7c21185b-5d05-479e-8277-b10adb64e0e4",
				"role": "user",
				"content": {
					"content_type": "text",
					"parts": [prompt]
				}
			}],
			"parent_message_id": "6b9ed3ec-9e12-4294-a710-75acedae7546",
			"model": "text-davinci-002-render"
		}
	);
	println!("json: {}", json);
	json["messages"][0]["content"]["parts"][0] = serde_json::Value::String(prompt.to_string());
	json.to_string()
}

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

pub async fn run(_options: &[CommandDataOption]) -> String
{
	let message = _options[0].value.as_ref().unwrap().as_str().unwrap();

	let auth_token = get_auth_from_file();
	let cookie = get_cookie_from_file();
	let url = "https://chat.openai.com/backend-api/conversation";
	let body: String = build_body(message);

	let client = reqwest::Client::new();
	println!("AYYYYYYYYYY WE MADE IT");

	let mut headers = HeaderMap::new();

	// Add the headers to the HeaderMap
	headers.insert("Host", "chat.openai.com".parse().unwrap());
	headers.insert("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:107.0) Gecko/20100101 Firefox/107.0".parse().unwrap());
	headers.insert("Accept", "text/event-stream".parse().unwrap());
	headers.insert("Accept-Language", "en-US,en;q=0.5".parse().unwrap());
	headers.insert("Accept-Encoding", "gzip, deflate, br".parse().unwrap());
	headers.insert("Referer", "https://chat.openai.com/chat".parse().unwrap());
	headers.insert("Content-Type", "application/json".parse().unwrap());
	headers.insert("X-OpenAI-Assistant-App-Id", "".parse().unwrap());
	headers.insert("Authorization", auth_token.parse().unwrap());
	headers.insert("Content-Length", body.len().to_string().parse().unwrap());
	headers.insert("Origin", "https://chat.openai.com".parse().unwrap());
	headers.insert("Alt-Used", "chat.openai.com".parse().unwrap());
	headers.insert("Connection", "keep-alive".parse().unwrap());
	headers.insert("Cookie", cookie.parse().unwrap());
	headers.insert("Sec-Fetch-Dest", "empty".parse().unwrap());
	headers.insert("Sec-Fetch-Mode", "cors".parse().unwrap());
	headers.insert("Sec-Fetch-Site", "same-origin".parse().unwrap());
	headers.insert("TE", "trailers".parse().unwrap());

	let response = client.post(url)
		.headers(headers)
		.body(body)
		.send()
		.await
		.expect("Unable to send request");
	
	let response_body = response.text().await.expect("Unable to read response body");
	println!("RESPONSE! {}", response_body);
	println!("{}", response_body);
	let data = get_penultimate_data(response_body);
	match data
	{
		Some(data) =>
		{
			let json = serde_json::from_str::<Value>(data.as_str()).unwrap();
			println!("{:?}", json);
			let response = json["message"]["content"]["parts"][0].to_string();
			response[1..response.len() - 1].to_string().replace("\\n","\n")
		}
		None => "No response".to_string(),
	}
}

