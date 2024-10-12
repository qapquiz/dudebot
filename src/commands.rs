use crate::{Context, Error};

#[poise::command(slash_command, guild_only)]
pub async fn verify(ctx: Context<'_>) -> Result<(), Error> {
    let member = ctx.author_member().await.unwrap();
    let guild_id = ctx.guild_id().unwrap();
    let roles = guild_id.roles(&ctx.http()).await.unwrap();
    let verified_role = roles.values().find(|role| role.name == "verified").unwrap();

    match member.add_role(&ctx.http(), verified_role.id).await {
        Ok(_) => ctx.say("Verified successfully").await?,
        Err(why) => ctx.say(format!("Cannot verify: {}", why)).await?
    };

    Ok(())
}
