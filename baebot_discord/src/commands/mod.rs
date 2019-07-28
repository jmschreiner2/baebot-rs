mod music;
use std::collections::HashMap;
use baebot_models::discord::{AudioPayload, MessagePayload, CommandHandler};
use serenity::{
    model::channel::{Message, Channel},
    prelude::*
};

pub fn delegate_command(ctx: Context, msg: Message)
{
    let payloads = match get_command_params(&ctx, &msg)
    {
        Ok((command, params, is_nsfw)) => {
            let command_map = get_command_map();

            let command_method = match command_map.get(&command)
            {
                Some(method) => method,
                None => return
            };

            command_method(params, is_nsfw).unwrap_or_else(|_| { Vec::new() })
        },
        Err(err) => {
            println!("{:?}", err);
            vec![MessagePayload::Text(String::from("I dont understand nyaa."))]
        }
    };

    for payload in payloads
    {
        handle_payload(&ctx, &msg, payload);
    }
}

fn get_command_map() -> HashMap<String, CommandHandler>
{
    let mut command_map = HashMap::new();

    command_map.insert(String::from("ping"), ping as CommandHandler);
    //commandMap.extend(music::get_commands());

    command_map
}

fn get_command_params(ctx: &Context, msg: &Message) -> Result<(String, Vec<String>, bool), &'static str>
{
    // Remove initial "!" from command.
    let mut content_parts = msg.content[1..].split_whitespace();

    let command = match content_parts.next()
    {
        Some(cmd) => String::from(cmd),
        None => return Err("Could not parse command.")
    };

    let mut params = Vec::new();

    loop
    {
        match content_parts.next()
        {
            Some(param) => params.push(String::from(param)),
            None => { break }
        }
    }

    let is_nsfw = match msg.channel(&ctx.cache){
        Some(channel) => channel,
        None => return Err("Could not parse channel.")
    }.is_nsfw();

    Ok((command, params, is_nsfw))
}

fn handle_payload(ctx: &Context, msg: &Message, payload: MessagePayload)
{
    let channel = msg.channel_id;

    match payload
    {
        MessagePayload::Text(text) => 
        {
            if let Err(why) = channel.say(&ctx.http, text)
            {
                println!("{:?}", why);
            }
        },
        MessagePayload::Attachment(_) => println!("An attachment!"),
        MessagePayload::Audio(_) => println!("An audio!"),
        MessagePayload::AttachmentLink(_) => println!("An attachment link!")
    }
}

fn ping(_: Vec<String>, _: bool) -> Result<Vec<MessagePayload>, &'static str>
{
    let mut res = Vec::new();

    res.push(MessagePayload::Text(String::from("pong!")));

    Ok(res)
}