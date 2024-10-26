mod commands;
mod event_handler;

use dotenv::dotenv;
use poise::serenity_prelude::{self as serenity, ApplicationId, MessageId};
use std::env::var;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    verifed_message_id: MessageId,
    welcome_channel_id: u64,
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

async fn clear_old_commands(bot_token: &str, application_id: ApplicationId) -> Result<(), Error> {
    let http = serenity::HttpBuilder::new(bot_token)
        .application_id(application_id)
        .build();
    let global_commands = http.get_global_commands().await?;

    for command in global_commands {
        if let Some(guild_id) = command.guild_id {
            http.delete_guild_command(guild_id, command.id).await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv().ok();

    let options = poise::FrameworkOptions {
        commands: vec![
            commands::create_embed_message(),
            commands::create_verify_message(),
            commands::verify(),
        ],
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
            Box::pin(event_handler::event_handler(ctx, event, framework, data))
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
                    welcome_channel_id: 1057511145243676762_u64,
                })
            })
        })
        .options(options)
        .build();

    let token = var("DISCORD_TOKEN")
        .expect("Missing `DISCORD_TOKEN` env var, see README for more information.");
    let application_id = var("APPLICATION_ID")
        .expect("Missing `application_id` env var, see README for more information.")
        .parse::<u64>()
        .expect("Cannot parse APPLICATION_ID from String to u64");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    clear_old_commands(&token, ApplicationId::new(application_id))
        .await
        .unwrap();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap()
}
