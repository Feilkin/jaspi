extern crate discord;

#[macro_use]
extern crate serenity;

use serenity::client::Client;
use serenity::prelude::EventHandler;
use serenity::framework::standard::StandardFramework;
use std::env;

use discord::Discord;
use discord::model::Event;
use toml::Value;

#[macro_use]
use serde_derive;

#[derive(Deserialize)]
struct Settings {
    auth: Auth,
}

#[derive(Deserialize)]
struct Auth {
    token: String,
}

struct Handler;

impl EventHandler for Handler {}

fn main() {
    // Read config file
    let mut settings: Settings;
    {
        let mut settings_file = File::open(filename).expect("file not found");

        let mut settings_contents = String::new();
        settings_file.read_to_string(&mut contents).expect(
            "something went wrong reading the bot_settings.toml file",
        );

        settings = toml::from_str(settings_contents).unwrap();
    }

    // Login with a bot token from the environment
    let mut client = Client::new(settings.auth.token, Handler);
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "~"
        .on("ping", ping));

    // start listening for events by starting a single shard
    let _ = client.start();
}

command!(ping(_context, message) {
    let _ = message.reply("Pong!");
});
