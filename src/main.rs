mod commands;

use dotenv::dotenv;
use poise::serenity_prelude::{self as serenity, CacheHttp, MessageId};
use std::env::var;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    verifed_message_id: MessageId,
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv().ok();

    let options = poise::FrameworkOptions {
        commands: vec![commands::create_verify_message(), commands::verify()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("!".into()),
            ..Default::default()
        },
        command_check: Some(|ctx| {
            Box::pin(async move {
                if ctx.command().name == "verify" {
                    return Ok(true);
                }

                Ok(false)
            })
        }),
        skip_checks_for_owners: true,
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            })
        },
        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}!", ctx.command().qualified_name);
            })
        },
        event_handler: |ctx, event, framework, data| {
            Box::pin(event_handler(ctx, event, framework, data))
        },
        on_error: |error| Box::pin(on_error(error)),
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                Ok(Data {
                    verifed_message_id: MessageId::new(1028609208989523969_u64),
                })
            })
        })
        .options(options)
        .build();

    let token = var("DISCORD_TOKEN")
        .expect("Missing `DISCORD_TOKEN` env var, see README for more information.");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap()
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    if let serenity::FullEvent::ReactionAdd { add_reaction } = event {
        if add_reaction.message_id == data.verifed_message_id && add_reaction.emoji.unicode_eq("✅")
        {
            let guild_id = add_reaction.guild_id.unwrap();
            let roles = guild_id.roles(&ctx.http()).await.unwrap();
            let verified_role = roles.values().find(|role| role.name == "verified").unwrap();

            add_reaction
                .member
                .clone()
                .unwrap()
                .add_role(&ctx.http(), verified_role.id)
                .await?;

            println!("Verified member via ReactionAdd");
        }
    }

    Ok(())
}
