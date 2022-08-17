use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use tokio::time;

use std::time::Duration;

use tokio::task;

use syllable;

const SLEEP_COMMAND: &str = "!sleep";
const SUFFIXES: [&str; 15] = [
    "y", "ee", "ie", " me", " be", " he", " c", " b", " d", " e", " g", " p", " t", " v", " z",
];

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let mut syllable_counter = syllable::Counter::new();
        let mut num_syllables = 0;
        for word in msg
            .content
            .replace(&['(', ')', ',', '\"', '.', ';', ':', '\''][..], "")
            .split_whitespace()
        {
            num_syllables += syllable_counter.count(&word);
        }
        if num_syllables == 5 && string_ends_with_any(&msg.content, &SUFFIXES) {
            if let Err(why) = msg
                .channel_id
                .say(
                    &ctx.http,
                    "https://tenor.com/view/everybody-wants-to-be-my-gif-25137901",
                )
                .await
            {
                eprintln!("Error sending message: {:?}", why);
            }
        }

        if let Some((command, time_to_wait)) = msg.content.split_once(" ") {
            if command == SLEEP_COMMAND {
                /*
                match time.trim().parse() {
                    Ok(time: u32) => (),
                    Err(why) => (),

                }*/

                if let Ok(time_to_wait) = time_to_wait.trim().parse::<u64>() {
                    if let Err(why) = msg
                        .channel_id
                        .say(
                            &ctx.http,
                            format!("You will be disconnected in {} minutes!", time_to_wait),
                        )
                        .await
                    {
                        eprintln!("Error sending message: {:?}", why);
                    }

                    match msg.member(&ctx.http).await {
                        Ok(member) => {
                            task::spawn(async move {
                                time::sleep(Duration::from_secs(time_to_wait * 60)).await;
                                match member.disconnect_from_voice(&ctx.http).await {
                                    Ok(_) => (),
                                    Err(why) => eprintln!("Error disconnecting user: {:?}", why),
                                }
                            });
                        }
                        Err(why) => eprintln!("Error retrieving member: {:?}", why),
                    };
                } else {
                    if let Err(why) = msg
                        .channel_id
                        .say(
                            &ctx.http,
                            "Please specify a valid time in minutes e.g. !sleep 15",
                        )
                        .await
                    {
                        eprintln!("Error sending message: {:?}", why);
                    }
                }
            }
        } else if msg.content == "!sleep" {
            if let Err(why) = msg
                .channel_id
                .say(
                    &ctx.http,
                    "Please specify a valid time in minutes e.g. !sleep 15",
                )
                .await
            {
                eprintln!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn string_ends_with_any(s: &str, suffixes: &[&str]) -> bool {
    let s = s.to_lowercase();
    return suffixes.iter().any(|&suffix| s.ends_with(suffix));
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
