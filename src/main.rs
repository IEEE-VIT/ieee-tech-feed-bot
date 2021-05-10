use dotenv::dotenv;
use std::env;

use regex::Regex;
// use url::Url;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, id::ChannelId},
    prelude::*,
    utils::{CustomMessage, MessageBuilder},
};

struct Handler {
    internal_channel_id: String,
    techloop_channel_id: String,
}

impl Handler {
    fn new(internal_channel_id: String,  techloop_channel_id: String) -> Handler {
        return Handler {
            internal_channel_id,
            techloop_channel_id
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.channel_id.to_string() == self.internal_channel_id {
            let re = Regex::new(
                r"^(http[s]?://[www\.]?|ftp://[www\.]?|www\.){1}([0-9A-Za-z-\.@:%_\+~#=]+)+((\.[a-zA-Z]{2,3})+)(/(.)*)?(\\?(.)*)?",
            );
            match re {
                Ok(re) => {
                    for mat in re.find_iter(&msg.content) {
                        let url = msg.content[mat.start()..mat.end()].to_string();
                        println!("{:?}", url);
                        let id = match self.techloop_channel_id.parse::<u64>() {
                            Ok(i) =>i,
                            Err(_e) => return,
                        };

                        let tech_feed_channel_id = id;
                        let channel_id = ChannelId(tech_feed_channel_id);

                        let message = CustomMessage::new()
                            .content("something")
                            .channel_id(channel_id)
                            .to_owned()
                            .build();

                        let resp = MessageBuilder::new().push(url.clone()).build();
                        if let Err(err) = message.channel_id.say(&ctx.http, &resp).await {
                            println!("Error sending message: {:?}", err);
                        }
                    }
                }
                Err(err) => {
                    println!("{:?}", err);
                }
            }
        }
    }

    async fn ready(&self, _ctx: Context, data_about_bot: Ready) {
        println!("{} is connected!", data_about_bot.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut token = String::new();
    let mut internal_channel_id = String::new();
    let mut techloop_channel_id = String::new();
    for (key, value) in env::vars() {
        if key == "DISCORD_IEEE_INTERNAL_TOKEN" {
            token = value.clone();
        }
        if key == "IEEE_INTERNAL_CHANNEL_ID" {
            internal_channel_id = value.clone();
        }
        if key == "IEEE_TECHLOOP_CHANNEL_ID" {
            techloop_channel_id = value.clone();
        }
    }

    let handler = Handler::new(internal_channel_id, techloop_channel_id);

    let mut client = Client::builder(&token)
        .event_handler(handler)
        .await
        .expect("Error creating discord client");

    if let Err(err) = client.start().await {
        println!("Error starting client: {:?}", err);
    }
}
