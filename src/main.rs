mod commands;
use std::env;

use serenity::async_trait;
use serenity::builder::CreateInteractionResponseFollowup;
//use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::prelude::ChannelId;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
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
				"ping" => commands::ping::run(&command.data.options),
				"numberinput" => commands::numberinput::run(&command.data.options),
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
				//"id" => commands::id::run(&command.data.options),
				//"attachmentinput" => commands::attachmentinput::run(&command.data.options),
				_ => "not implemented :(".to_string(),
			};

			if let Err(why) = command
				.create_interaction_response(&ctx.http, |response| {
					response
						.kind(InteractionResponseType::ChannelMessageWithSource)
						.interaction_response_data(|message| message.content(content))
				})
				.await
			{
				//println!("Cannot respond to slash command: {}", why);
			}
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

		let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands|
		{
			commands
				.create_application_command(|command| commands::ping::register(command))
				//.create_application_command(|command| commands::id::register(command))
				//.create_application_command(|command| commands::welcome::register(command))
				.create_application_command(|command| commands::numberinput::register(command))
				//.create_application_command(|command| commands::attachmentinput::register(command))
				.create_application_command(|command| commands::update_cookie::register(command))
				.create_application_command(|command| commands::update_authentication::register(command))
				.create_application_command(|command| commands::gpt::register(command))
		})
		.await;

		match commands
		{
			Ok(commands) =>
			{
				//println!("Slash commands registered: {:#?}", commands);
			}
			Err(why) =>
			{
				//println!("Cannot register slash commands: {}", why);
			}
		}

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

	// Finally, start a single shard, and start listening to events.
	//
	// Shards will automatically attempt to reconnect, and will perform
	// exponential backoff until it reconnects.
	if let Err(why) = client.start().await
	{
		//println!("Client error: {:?}", why);
	}
}

