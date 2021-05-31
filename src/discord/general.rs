use serenity::{
    framework::standard::{
        StandardFramework,
        Args, CommandResult,
        macros::{command, group}
    },
    model::channel::Message,
    client::Context
};
use rand::{
    Rng,
    SeedableRng,
    rngs::SmallRng
};
use tokio::time::{sleep, Duration};

#[group]
#[description = "Various random commands."]
#[summary = "Various random commands."]
#[commands(dab, dabird, doot, fbi, gachi, imgay, kappa, like, mm, ohyeah, these, edifier, dawae)]
struct General;

pub fn setup(framework: StandardFramework) -> StandardFramework {
    framework.group(&GENERAL_GROUP)
}

#[command]
async fn dab(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut rng = SmallRng::from_entropy();
    let mut count = 1;

    match args.parse() {
        Ok(res) => count = res,
        Err(_) => println!("Could not parse the dab count.")
    }

    let mut is_reverse = rng.gen_bool(0.5);

    for _ in 0..std::cmp::min(count, 5) {
        if is_reverse {
            let _ = msg
                .channel_id
                .send_files(&ctx.http, vec!["assets/dab_reverse.png"], |m| m)
                .await;
        }
        else {
            let _ = msg
                .channel_id
                .send_files(&ctx.http, vec!["assets/dab.png"], |m| m)
                .await;
        }

        is_reverse = !is_reverse;
        sleep(Duration::from_millis(10)).await;
    }

    Ok(())
}

#[command]
async fn dabird(ctx: &Context, msg: &Message) -> CommandResult {
    send_image(ctx, msg, vec!["assets/dabird.gif"]).await
}

#[command]
async fn doot(ctx: &Context, msg: &Message) -> CommandResult {
    send_image(ctx, msg, vec!["assets/doot.png"]).await
}

#[command]
async fn fbi(ctx: &Context, msg: &Message) -> CommandResult {
    send_image(ctx, msg, vec!["assets/fbi.gif"]).await
}

#[command]
async fn gachi(ctx: &Context, msg: &Message) -> CommandResult {
    send_image(ctx, msg, vec!["assets/gachi.png"]).await
}

#[command]
async fn imgay(_ctx: &Context, _msg: &Message) -> CommandResult {
    Ok(())
}

#[command]
async fn kappa(ctx: &Context, msg: &Message) -> CommandResult {
    send_image(ctx, msg, vec!["assets/kappa.png"]).await
}

#[command]
async fn like(_ctx: &Context, _msg: &Message) -> CommandResult {
    Ok(())
}

#[command]
async fn mm(ctx: &Context, msg: &Message) -> CommandResult {
    send_image(ctx, msg, vec!["assets/mm.gif"]).await
}

#[command]
async fn ohyeah(_ctx: &Context, _msg: &Message) -> CommandResult {
    Ok(())
}

#[command]
async fn these(ctx: &Context, msg: &Message) -> CommandResult {
    send_image(ctx, msg, vec!["assets/psy.png"]).await
}

#[command]
async fn edifier(ctx: &Context, msg: &Message) -> CommandResult {
    let _ = msg
        .channel_id
        .say(&ctx.http, "https://www.amazon.com/Swans-Speakers-Bluetooth-Bookshelf-Enclosure/dp/B07C1TVLDX")
        .await;

    Ok(())
}

#[command]
async fn dawae(ctx: &Context, msg: &Message) -> CommandResult {
    let _ = msg
        .channel_id
        .say(&ctx.http, "
```
⢀⢀⢀⢀⢀⢀⢀⢀⢀⢀⠶⣿⣭⡧⡤⣤⣻⣛⣹⣿⣿⣿⣶⣄
⢀⢀⢀⢀⢀⢀⢀⢀⢀⣼⣊⣤⣶⣷⣶⣧⣤⣽⣿⣿⣿⣿⣿⣿⣷
⢀⢀⢀⢀⢀⢀⢀⢀⢀⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇
⢀⢀⢀⢀⢀⢀⢀⣠⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧
⢀⢀⢀⢀⢀⢀⠸⠿⣿⣿⠿⣿⣿⣿⣿⣿⣿⣿⡿⣿⣻⣿⣿⣿⣿⣿⡆
⢀⢀⢀⢀⢀⢀⢀⢸⣿⣿⡀⠘⣿⡿⢿⣿⣿⡟⣾⣿⣯⣽⣼⣿⣿⣿⣿⡀
⢀⢀⢀⢀⢀⢀⡠⠚⢛⣛⣃⢄⡁⢀⢀⢀⠈⠁⠛⠛⠛⠛⠚⠻⣿⣿⣿⣷
⢀⢀⣴⣶⣶⣶⣷⡄⠊⠉⢻⣟⠃⢀⢀⢀⢀⡠⠔⠒⢀⢀⢀⢀⢹⣿⣿⣿⣄⣀⣀⣀⣀⣀⣀
⢠⣾⣿⣿⣿⣿⣿⣿⣿⣶⣄⣙⠻⠿⠶⠒⠁⢀⢀⣀⣤⣰⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄
⢿⠟⠛⠋⣿⣿⣿⣿⣿⣿⣿⣟⡿⠷⣶⣶⣶⢶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡄
⢀⢀⢀⢀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠉⠙⠻⠿⣿⣿⡿
⢀⢀⢀⢀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢀⢀⢀⢀⠈⠁
⢀⢀⢀⢀⢸⣿⣿⣿⣿⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⢀⢀⢀⢀⢸⣿⣿⣿⣿⣄⠈⠛⠿⣿⣿⣿⣿⣿⣿⣿⡿⠟⣹⣿⣿⣿⣿⣿⣿⣿⣿⠇
⢀⢀⢀⢀⢀⢻⣿⣿⣿⣿⣧⣀⢀⢀⠉⠛⠛⠋⠉⢀⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⠏
⢀⢀⢀⢀⢀⢀⢻⣿⣿⣿⣿⣿⣷⣤⣄⣀⣀⣤⣴⣾⣿⣿⣿⣿⣿⣿⣿⣿⡿⠋
⢀⢀⢀⢀⢀⢀⢀⠙⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠛
⢀⢀⢀⢀⢀⢀⢀⢀⢀⢹⣿⡿⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡟⠁
⢀⢀⢀⢀⢀⢀⢀⢀⢀⢸⣿⡇⢀⠈⠙⠛⠛⠛⠛⠛⠛⠻⣿⣿⣿⠇
⢀⢀⢀⢀⢀⢀⢀⢀⢀⣸⣿⡇⢀⢀⢀⢀⢀⢀⢀⢀⢀⢀⢨⣿⣿
⢀⢀⢀⢀⢀⢀⢀⢀⣾⣿⡿⠃⢀⢀⢀⢀⢀⢀⢀⢀⢀⢀⢸⣿⡏
⢀⢀⢀⢀⢀⢀⢀⢀⠻⠿⢀⢀⢀⢀⢀⢀⢀⢀⢀⢀⢀⢠⣿⣿⡇
``` 
        ")
        .await;

    Ok(())
}

async fn send_image(ctx: &Context, msg: &Message, files: Vec<&str>) -> CommandResult {
    let _ = msg
        .channel_id
        .send_files(&ctx.http, files, |m| m)
        .await;

    Ok(())
}
