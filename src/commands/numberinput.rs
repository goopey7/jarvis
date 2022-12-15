use serenity::builder;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn register(
	command: &mut builder::CreateApplicationCommand,
) -> &mut builder::CreateApplicationCommand
{
	command
		.name("numberinput")
		.description("Test command for number input")
		.create_option(|option| {
			option
				.name("int")
				.description("Any positive integer")
				.kind(CommandOptionType::Integer)
				.min_int_value(0)
				.required(true)
		})
		.create_option(|option| {
			option
				.name("float")
				.description("Any floating point number")
				.kind(CommandOptionType::Number)
				.required(true)
		})
}

pub fn run(_options: &[CommandDataOption]) -> String
{
	let num1 = _options[0].value.as_ref().unwrap().as_i64().unwrap();
	let num2 = _options[1].value.as_ref().unwrap().as_f64().unwrap();

	format!("You entered {} and {}", num1, num2)
}

