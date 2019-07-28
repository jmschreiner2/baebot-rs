mod settings;
mod commands;
use serenity::{
    model::{event::ResumedEvent, gateway::Ready, channel::Message},
    prelude::*
};

struct Handler;

impl EventHandler for Handler
{
    fn ready(&self, _: Context, ready: Ready)
    {
        println!("Connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent)
    {
        println!("Resumed");
    }

    fn message(&self, ctx: Context, msg: Message)
    {
        commands::delegate_command(ctx, msg);
    }
}

fn main() 
{
    let discord_token = settings::build_auth().discord.token;

    let mut client = Client::new(&discord_token, Handler).expect("Error creating client.");

    if let Err(why) = client.start()
    {
        println!("Client error: {:?}", why)
    }
}
