mod settings;
mod discord;
mod connectors;

use serenity::client::Client;
use settings::StartupSettings;
use discord::{
    Handler,
    setup_framework
};
use log::*;

#[macro_use]
extern crate serde_derive;

#[tokio::main]
async fn main() {
    simple_logger::SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .with_module_level("tracing::span", LevelFilter::Warn)
        .with_module_level("serenity", LevelFilter::Warn)
        .init().unwrap();

    let config = StartupSettings::new().unwrap();

    let framework = setup_framework();

    let mut client = Client::builder(config.token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client.");

    if let Err(why) = client.start().await {
        error!("Client Error: {:?}", why);
    }
}


