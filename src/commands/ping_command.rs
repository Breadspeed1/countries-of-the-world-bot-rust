use serenity::client::Context;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::id::GuildId;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

pub async fn set_command(guild: &GuildId, ctx: &Context) {
    if let Err(why) = guild.set_application_commands(&ctx.http, |commands| {
        commands.create_application_command(|command| {
            command.name("ping").description("Test the bot")
        })
    }).await
    {
        println!("Error setting up ping command ({})", why);
    }
    else {
        println!("Success setting up ping command!");
    }
}

pub async fn handle(ctx: &Context, interaction: &ApplicationCommandInteraction) {
    if let Err(why) = interaction.create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.content("hoe"))
    }).await
    {
        println!("Error responding to ping command ({})", why);
    }
}