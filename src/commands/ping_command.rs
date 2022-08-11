use std::error::Error;
use serenity::client::Context;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::id::GuildId;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

pub async fn set_command(guild: &GuildId, ctx: &Context) -> Result<(), Box<dyn Error>> {
    guild.set_application_commands(&ctx.http, |commands| {
        commands.create_application_command(|command| {
            command.name("ping").description("Test the bot")
        })
    }).await?;

    Ok(())
}

pub async fn handle(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<(), Box<dyn Error>> {
    interaction.create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.content("hoe"))
    }).await?;

    Ok(())
}