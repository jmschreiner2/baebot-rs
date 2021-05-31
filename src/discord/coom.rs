use serenity::{
    framework::standard::{
        StandardFramework,
        Args, CommandResult,
        macros::{command, group}
    },
    model::channel::Message,
    client::Context,
    utils::MessageBuilder
};

use log::*;

use crate::connectors::{search_tag, get_random_picture, get_real_picture, search_real_tag, base::TagResult};

#[group]
#[prefixes("coom", "cum", "tribute")]
#[description = "Finds and post a waifu.\n\n
Ex. \"!coom on tohsaka\""]
#[summary = "Find a waifu."]
#[default_command(on)]
#[commands(on, search)]
struct Coom;

pub fn setup(framework: StandardFramework) -> StandardFramework {
    framework.group(&COOM_GROUP)
}

#[command]
#[sub_commands(on_real)]
async fn on(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let params: Vec<String> = args
        .iter::<String>()
        .map(|s| s.unwrap())
        .collect();

    if params.is_empty() {
        msg.channel_id
            .say(&ctx.http, "I couldn't understand what you searched for, nya!\n
            You can ask for !help or search for tags using !coom search").await?;

        return Ok(());
    }

    let sfw_only = !msg.channel_id
        .to_channel(&ctx.http)
        .await?
        .is_nsfw();

    let picture_result = get_random_picture(params, sfw_only).await;

    match picture_result {
        Some(res) => msg.channel_id
            .send_message(&ctx.http, move |m| {
                m.embed(|e| {
                    e.url(res.source_url.clone());
                    e.title("SAUCE");
                    e.image(res.picture_url.clone());
                    e
                });
                m
            }).await?,
        None => msg.channel_id
            .say(&ctx.http, "I couldn't find a picture, nya!")
            .await?
    };

    Ok(())
}

#[command("real")]
async fn on_real(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let params: Vec<String> = args
        .iter::<String>()
        .map(|s| s.unwrap())
        .collect();

    if params.is_empty() {
        msg.channel_id
            .say(&ctx.http, "I couldn't understand what you searched for, nya!\n
            You can ask for !help or search for tags using !coom search").await?;

        return Ok(());
    }

    let sfw_only = !msg.channel_id
        .to_channel(&ctx.http)
        .await?
        .is_nsfw();

    let picture_result = get_real_picture(params, sfw_only).await;

    match picture_result {
        Some(res) => msg.channel_id
            .send_message(&ctx.http, move |m| {
                m.embed(|e| {
                    e.url(res.source_url.clone());
                    e.title("SAUCE");
                    e.image(res.picture_url.clone());
                    e
                });
                m
            }).await?,
        None => msg.channel_id
            .say(&ctx.http, "I couldn't find a picture, nya!")
            .await?
    };

    Ok(())
}

#[command]
#[sub_commands(search_real)]
async fn search(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let param = match args.single() {
        Ok(tag) => tag,
        Err(why) => {
            error!("Could not pull arg: {:?}", why);
            String::new()
        }
    };

    if param.is_empty() {
        msg.channel_id
            .say(&ctx.http, "I couldn't understand what you searched for, nya!\n
            You can ask for !help.").await?;

        return Ok(());
    }

    let tags = search_tag(param).await;

    send_tag_results(ctx, msg, tags).await?;
    Ok(())
}

#[command("real")]
async fn search_real(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let param = match args.single() {
        Ok(tag) => tag,
        Err(why) => {
            error!("Could not pull arg: {:?}", why);
            String::new()
        }
    };

    if param.is_empty() {
        msg.channel_id
            .say(&ctx.http, "I couldn't understand what you searched for, nya!\n
            You can ask for !help.").await?;

        return Ok(());
    }

    let tags = search_real_tag(param).await;

    send_tag_results(ctx, msg, tags).await?;
    Ok(())
}

async fn send_tag_results(ctx: &Context, msg: &Message, tags: Vec<TagResult>) -> CommandResult {
    msg.channel_id
        .send_message(&ctx.http, move |m| {
            m.embed(|e| {
                e.title("Search Result");

                let mut results = Vec::new();

                let mut i = 0;
                for tag in tags.iter() {
                    i += 1;
                    let content = MessageBuilder::new()
                        .push_safe(format!("{}. {}", i, tag.tag))
                        .build();

                    results.push(content);
                }
                
                e.description(results.join("\n"));
                e.author(|a| {
                    a.name("derp");
                    a.url("https://bestgirl.us");
                    a
                });
                e
            });
            m
        })
        .await?;
    Ok(())
}
