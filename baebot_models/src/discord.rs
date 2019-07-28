pub enum AudioPayload
{
    Youtube(String),
    Asset(u128),  //TODO Figure out stream?
    Stop
}

pub enum MessagePayload
{
    Attachment(u128), //TODO: Figure out stream?
    AttachmentLink(String),
    Text(String),
    Audio(AudioPayload)
}

pub type CommandHandler = fn(Vec<String>, bool) -> Result<Vec<MessagePayload>, &'static str>;