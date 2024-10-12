use poise::serenity_prelude::{self as serenity, CacheHttp, CreateMessage};

use crate::{Data, Error};

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::GuildMemberAddition { new_member } => {
            let guild_id = new_member.guild_id;
            let channels = guild_id.channels(&ctx.http()).await.unwrap();
            let welcome_channel = channels
                .values()
                .find(|channel| channel.id == data.welcome_channel_id);

            match welcome_channel {
                Some(channel) => {
                    channel
                        .send_message(
                            &ctx.http(),
                            CreateMessage::new().content(format!(
                                "Welcome <@{}> to the Noodles party!",
                                new_member.user.id
                            )),
                        )
                        .await?;
                }
                None => {
                    println!("there is no welcome channel");
                }
            }
        }
        serenity::FullEvent::ReactionAdd { add_reaction } => {
            if add_reaction.message_id == data.verifed_message_id
                && add_reaction.emoji.unicode_eq("✅")
            {
                let guild_id = add_reaction.guild_id.unwrap();
                let roles = guild_id.roles(&ctx.http()).await.unwrap();
                let verified_role = roles.values().find(|role| role.name == "verified");

                match verified_role {
                    Some(role) => {
                        add_reaction
                            .member
                            .clone()
                            .unwrap()
                            .add_role(&ctx.http(), role.id)
                            .await?;

                        println!("Verified member via ReactionAdd");
                    }
                    None => println!("there is no verified role"),
                }
            }
        }
        _ => {}
    }

    Ok(())
}
