use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use tokio::time;

use std::time::Duration;

use tokio::task;

const SLEEP_COMMAND: &str = "!sleep";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.to_lowercase() == "oh the misery" {
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

        if msg.content.to_lowercase() == "spare the sympathy" {
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
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
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
