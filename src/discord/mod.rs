use std::collections::HashSet;
use serenity::{
    async_trait,
    model::{
        channel::Message,
        gateway::Ready,
        id::UserId
    },
    framework::standard::{
        Args,
        CommandGroup,
        StandardFramework,
        HelpOptions,
        help_commands,
        macros::{hook, help},
        CommandResult,
        DispatchError
    },
    client::Context
};
use log::*;
use serenity::prelude::*;

mod coom;
mod general;

pub struct Handler;

pub fn setup_framework() -> StandardFramework {
    general::setup(
    coom::setup(
        StandardFramework::new()
            .configure(|c| c
                .prefix("!")
                .allow_dm(false))
            .before(before)
            .after(after)
            .unrecognised_command(unknown_command)
            .on_dispatch_error(dispatch_error)
            .help(&HELP)
    ))
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[hook]
async fn before(_ctx: &Context, _msg: &Message, command_name: &str) -> bool {
    info!("Executing command {}", command_name);
    true
}
 
#[hook]
async fn after(_ctx: &Context, _msg: &Message, command_name: &str, command_result: CommandResult) {
    match command_result {
        Ok(()) => info!("Command finished successfully {}", command_name),
        Err(why) => warn!("Command '{}' returned error {:?}", command_name, why)
    };
}

#[hook]
async fn unknown_command(ctx: &Context, msg: &Message, command_name: &str) {
    warn!("{} is an unknown command", command_name);

    let _ = msg
        .channel_id
        .say(&ctx.http, "I dont understand nyou! Ask for \"!help\" need help, uwu")
        .await;
}

#[hook]
async fn dispatch_error(_ctx: &Context, _msg: &Message, error: DispatchError) {
    error!("Failed to dispatch command {:?}", error); 
}

#[help]
#[individual_command_tip = "Hello nyow. Look at what I can do, nyow!\n
If you need help with a command, pwease ask!\n\n
\"!help <command>\""]
#[indention_prefix = "-"]
async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, options, groups, owners).await;
    Ok(())
}
