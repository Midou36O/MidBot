use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::prelude::{EventHandler, Context};
use serenity::framework::standard::{
	StandardFramework,
	CommandResult,
	macros::{
		command,
		group
	}
};

group!({
	name: "general",
	options: {},
	commands: [ping],
});

use std::env;

struct Handler;
impl EventHandler for Handler {}

fn main() {
    //Login with a bot token from the environment
    let mut client = Client::new(&env::var("insert a discord token here").expect("token"), Handler)
    .expect ("Error creating client");
    client.with_framework(StandardFramework::new()
    	.configute(|c|c.prefix(">>")) //this is the bot's prefix ">>"
    	.group(&GENERAL_GROUP));
    //Start listening for events by starting a single shard
    if let Err(why) = client.start(){
    	println!("An error occured while running the client: {:?}", why);
    }
}
#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
	msg.reply(ctx, "Pong!")?;

	ok(())
}