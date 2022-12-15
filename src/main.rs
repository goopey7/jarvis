mod commands;
use std::env;

use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler
{
	async fn interaction_create(&self, ctx: Context, interaction: Interaction)
	{
		if let Interaction::ApplicationCommand(command) = interaction
		{
			//println!("Received command interaction: {:#?}", command);

			let content = match command.data.name.as_str()
			{
				"update-cookie" => commands::update_cookie::run(&command.data.options),
				"update-authentication" => commands::update_authentication::run(&command.data.options),
				"gpt" =>
				{
					command.defer(&ctx.http).await.unwrap();
					let typing = ctx.http.start_typing(command.channel_id.0).unwrap();
					let response = commands::gpt::run(&command.data.options).await;
					command.create_followup_message(&ctx.http, |message|
						{
							message.content(response);
							message
						}).await.unwrap();
					typing.stop().unwrap();
					"".to_string()
				},
				_ => "not implemented :(".to_string(),
			};
		}
	}

	async fn ready(&self, ctx: Context, ready: Ready)
	{
		//println!("{} is connected!", ready.user.name);

		let guild_id = GuildId
		(
			env::var("GUILD_ID")
				.expect("Expected GUILD_ID in environment")
				.parse()
				.expect("GUILD_ID must be an integer"),
		);

		GuildId::set_application_commands(&guild_id, &ctx.http, |commands|
		{
			commands
				.create_application_command(|command| commands::update_cookie::register(command))
				.create_application_command(|command| commands::update_authentication::register(command))
				.create_application_command(|command| commands::gpt::register(command))
		})
		.await.unwrap();
	}
}

#[tokio::main]
async fn main()
{
	// Configure the client with your Discord bot token in the environment.
	let discord_token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

	// Build our client.
	let mut client = Client::builder(discord_token, GatewayIntents::empty())
		.event_handler(Handler)
		.await
		.expect("Error creating client");

	// Start client
	client.start().await.unwrap();
}

