use poise::{serenity_prelude::CreateEmbed, CreateReply};

use crate::{Context, Error};

#[poise::command(slash_command, guild_only, ephemeral)]
pub async fn verify(ctx: Context<'_>) -> Result<(), Error> {
    let member = ctx.author_member().await.unwrap();
    let guild_id = ctx.guild_id().unwrap();
    let roles = guild_id.roles(&ctx.http()).await.unwrap();
    let verified_role = roles.values().find(|role| role.name == "verified");

    match verified_role {
        Some(role) => {
            match member.add_role(&ctx.http(), role.id).await {
                Ok(_) => ctx.say("Verified successfully").await?,
                Err(why) => ctx.say(format!("Cannot verify: {}", why)).await?,
            };
        }
        None => println!("there is no verified role"),
    }

    Ok(())
}

#[poise::command(prefix_command, slash_command, guild_only)]
pub async fn create_verify_message(ctx: Context<'_>) -> Result<(), Error> {
    let embed = CreateEmbed::new()
        .title("Welcome to noodles ≈ 🍜")
        .description("Click the ✅ to verify and gain access to the rest of the server.");

    ctx.send(CreateReply::default().embed(embed).reply(false))
        .await?;

    Ok(())
}
