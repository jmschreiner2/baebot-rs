/*use baebot_models::discord::{AudioPayload, CommandHandler, MessagePayload};
use std::collections::HashMap;

pub fn get_commands() -> HashMap<string, CommandHandler>
{
    let mut commandMap = HashMap::new();

    commandMap.insert("play", play);

    commandMap
}

fn play(params: Vec<string>, isNsfw: bool) -> Vec<MessagePayload> 
{
    match params.get(0)
    {
        Ok(link) => MessagePayload::Audio(AudioPayload::Youtube(link)),
        Err(_) => MessagePayload::Text(String::from("Please supply a youtube link, senpai!"))
    }
}*/