#[macro_use] extern crate log;
#[macro_use] extern crate serenity;   
#[macro_use] extern crate env_logger;




use serenity::model::gateway::Ready;



use serenity::model::channel::ReactionType;
use serenity::model::id::EmojiId;
use serenity::model::gateway::Game;
use serenity::client::bridge::gateway::ShardId;
use serenity::client::bridge::gateway::ShardManager;

use std::ops::Add;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io;
use serenity::model::channel::Channel;
use serenity::framework::standard::StandardFramework;
use serenity::utils::{content_safe, ContentSafeOptions};
use serenity::model::id::ChannelId;
use std::{thread, time};
use serenity::prelude::*;
use serenity::model::event::ResumedEvent;

use std::{env, sync::Arc};

use serenity::{client::{Context}, prelude::Mutex};
use typemap::Key;


/*
This is my bot i made 5 months ago, recovered from deletion, it's shit code so expect to have your eyes bleeding from this mess.
Oh also, most of the code is taken from the serenity bot examples, don't worry i'll rewrite the bot code if i get some motivation.
*/

struct Handler;

struct ShardManagerContainer;

impl Key for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}



impl EventHandler for Handler { 
    fn ready(&self, ctx:Context, ready:Ready ) {
     ctx.set_game(Game::listening("prefix:  >>"));
     info!("{} is connected!", ready.user.name); }


    fn resume(&self, _: Context, resume: ResumedEvent) {
   
        debug!("Resumed; trace: {:?}", resume.trace);
    }
}


fn main() {
   
	 env_logger::init().expect("Unable to init env_logger");


    let mut client = Client::new("Njc4NjU3Mjg2NjY2ODQ2MjQ5.Xkl-8Q.pw8VMWAa2-md6mr53rJ9nB__2-o", Handler)
        .expect("Error creating client");  
      {
        let mut data = client.data.lock();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix(">>"))

        .cmd("ping", latency)
        .cmd("test", ferris)
        .cmd("work?", bot_working)
        .cmd("pong_me", pingme)
        .cmd("help", help)
        .cmd("react_party", react_party)
        .cmd("react_thonkspin", react_thonk)
        .cmd("simon", simon)
        .cmd("help ping", help_ping)
        .cmd("about", about)
        .cmd("🏓", latency)
        .cmd("lenny", lenny)
        .cmd("save?", save)
        .cmd("test?", channel_name)
        .cmd("say", say)
        .cmd("wav", piing)
        .cmd("report", report)
        .cmd("everyone", everyfake));
    
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}


	command!(join())
    let guild = match msg.guild(&ctx.cache) {
        Some(guild) => guild,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "Groups and DMs not supported"));

            return Ok(());
        }
    };

    let guild_id = guild.read().id;

    let channel_id = guild
        .read()
        .voice_states.get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);


    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(msg.reply(&ctx, "Not in a voice channel"));

            return Ok(());
        }
    };

    let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();

    if manager.join(guild_id, connect_to).is_some() {
        check_msg(msg.channel_id.say(&ctx.http, &format!("Joined {}", connect_to.mention())));
    } else {
        check_msg(msg.channel_id.say(&ctx.http, "Error joining the channel"));
    }

    Ok(())
}


command!(lenny(_ctx, msg) {
    let _ = msg.channel_id.say("( ͡° ͜ʖ ͡°)");
});
command!(bot_working(ctx, msg) {
    let _ = msg.channel_id.say("yes i'm alive <@471785433232179210>");
});
command!(help_ping(_ctx, msg) {
    let _ = msg.channel_id.say("pings you!(test)");
});



command!(simon(_ctx, msg) {
    let _ = msg.channel_id.say("this command has been discontinued, thanks for using it or not i don't care.");
});

command!(ferris(_ctx, msg) {
    let _ = msg.channel_id.say("<a:ferris_bongo:494140332812926981>");
});

command!(pingme(_ctx, msg) {
    let _ = msg.reply("you wanted a ping? here it is!");
});

command!(everyfake(ctx, msg){
    let _ =msg.channel_id.say("why? i won't do that !");
});

command!(help(_ctx, msg) {
let _ = msg.channel_id.send_message(|m| m
        .embed(|e| e
            .colour((20,10,65))
            .description("here are the commands you can use")
            .title("Help")
            .fields(vec![
                ("ping/🏓","pings you!", false),
                ("simon","**discontinued**", false),
                ("react_party/react_thonkspin","adds a reaction!", false),
                ("pong_me","you want a ping?", false),
                ("about","who made this bot!", false),
                ("lenny","( ͡° ͜ʖ ͡°)", false),
                ("say", "the bot says what you want!", false),
                ("help","shows this message!", false)      
            ])
            .footer(|f| f
                .text(format!("use them if you want. Ferris will do 😉 ")))
            .author(|a| a
                .icon_url("https://media.discordapp.net/attachments/495627629865598978/678663503707635713/d4b8daa527311da3fd3e406a09e07ea3.png")
                .name("midbot"))
        )
    );
});
command!(react_party(_ctx, msg){
    let  reaction = ReactionType::Custom {
    id: EmojiId(497016840250196018),
    animated: true,
    name: Some("gopher".to_string()),
};

if let Err(why) = msg.react(reaction) {
    println!("{:?}", why);
}});

command!(react_thonk(_ctx, msg){
    let  reaction = ReactionType::Custom {
    id: EmojiId(506521365633957898),
    animated: true,
    name: Some("ThonkSpin".to_string()),
};

if let Err(why) = msg.react(reaction) {
    println!("{:?}", why);
}});

command!(about(_ctx, msg){
    let _ = msg.channel_id.send_message(|m| m
        .embed(|e| e
            .colour((20,10,65))
            .description("A bot made by <@471785433232179210> !")
            .title("About")
            .fields(vec![
                ("Bot Invite", "[Invite](https://discordapp.com/oauth2/authorize?client_id=503895324490858500&scope=bot&permissions=1275452553)", true),
                ("Server Invite", "[Join](https://discord.gg/5wfbBk)", true),
                ("Helpers <3", "<@393041441301200896>, [serenity members](https://discord.gg/WBdGJCc)", false)
            ])
            .footer(|f| f
                .text(format!("Prefix: >> ")))
            .author(|a| a
                .icon_url("https://media.discordapp.net/attachments/495627629865598978/678663503707635713/d4b8daa527311da3fd3e406a09e07ea3.png")
                .name("midbot"))
            .thumbnail("https://media.discordapp.net/attachments/495627629865598978/678663503707635713/d4b8daa527311da3fd3e406a09e07ea3.png")
        )
    );
});

command!(latency(ctx, msg, _args) {
    // The shard manager is an interface for mutating, stopping, restarting, and
    // retrieving information about shards.
    let data = ctx.data.lock();

    let shard_manager = match data.get::<ShardManagerContainer>() {
        Some(v) => v,
        None => {
            let _ = msg.reply("There was a problem getting the shard manager");

            return Ok(());
        },
    };

    let manager = shard_manager.lock();
    let runners = manager.runners.lock();

    // Shards are backed by a "shard runner" responsible for processing events
    // over the shard, so we'll get the information about the shard runner for
    // the shard this command was sent over.
    let runner = match runners.get(&ShardId(ctx.shard_id)) {
        Some(runner) => runner,
        None => {
            let _ = msg.reply("No shard found");

            return Ok(());
        },
    };

    let _ = msg.channel_id.say(&format!("Pong!: `{:?}`", runner.latency.unwrap()));
});

command!(save(_ctx, _msg) {
    let mut file = File::create("test.txt")?;
    file.write_all(b"YOU SAVED A FUCKING MESSAGE")?;
     let _  = _msg.react(ReactionType::Unicode("✅".to_string()));
     let _  = _msg.channel_id.say("Saved!");

});

command!(channel_name(_ctx, msg) {
    let _ = match msg.channel() {
        Some(Channel::Category(c)) => msg.reply(&c.read().name),
        Some(Channel::Group(c)) => msg.reply(&c.read().name()),
        Some(Channel::Guild(c)) => msg.reply(&c.read().name),
        Some(Channel::Private(c)) => {
            let channel = c.read();
            let user = channel.recipient.read();

            msg.reply(&format!("DM with {}", user.name.clone()))
        },
        None => msg.reply("Unknown"),
    
    };
});

command!(say(_ctx, msg, args) {
    let mut settings = if let Some(guild_id) = msg.guild_id {
       // By default roles, users, and channel mentions are cleaned.
       ContentSafeOptions::default()
            // We do not want to clean channal mentions as they
            // do not ping users.
            .clean_channel(false)
            // If it's a guild channel, we want mentioned users to be displayed
            // as their display name.
            .display_as_member_from(guild_id)
    } else {
        ContentSafeOptions::default()
            .clean_channel(false)
            .clean_role(false)
    };

    let mut content = content_safe(&args.full(), &settings);
    
    if let Err(why) = msg.channel_id.say(&content) {
        println!("Error sending message: {:?}", why);
    }
});

command!(piing(_ctx, msg){
	let _ = msg.channel_id.say("this will take some time...");
	let channel_id = msg.channel_id ;
	let paths = vec![r"k:\ping.wav"];
let res = channel_id.send_files(paths, |m| m.content(""));
println!("send_files: {:?}", res);
	let _ =channel_id.say("seems like it didn't did it?");
});

//how to send a file on specific channels

/*
command!(ping(_ctx, msg){
//	let _ = msg.channel_id.say("that will take some time...");
//	let channel_id = channel_id(7) ;//sends message on the specific channel !!!important!!!
//	let paths = vec![r"C:\Users\midou\Desktop\videos\wheel.gif"];
//let res = channel_id.send_files(paths, |m| m.content(""));
//println!("send_files: {:?}", res);
//	let _ =channel_id.say("seems like it didn't did it?");
});
*/

command!(report(_ctx, msg, args) {
    let mut settings = if let Some(guild_id) = msg.guild_id {
       // By default roles, users, and channel mentions are cleaned.
       ContentSafeOptions::default()
            // We do not want to clean channel mentions as they
            // do not ping users.
            .clean_channel(false)
            // If it's a guild channel, we want mentioned users to be displayed
            // as their display name.
            .display_as_member_from(guild_id)
    } else {
        ContentSafeOptions::default()
            .clean_channel(false)
            .clean_role(false)
    };
	let channel_id = ChannelId(507928499156156431);
    let mut content = content_safe(&args.full(), &settings);

    if let Err(why) = channel_id.say(&content) {
        println!("Error sending message: {:?}", why);
    
    }

     let _ = msg.react(ReactionType::Unicode("✅".to_string()));

});


