use std::error::Error;
use std::ops::Add;
use mysql::PooledConn;
use serenity::client::Context;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::async_trait;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use crate::commands::CommandHandler;
use crate::data_types::User;
use crate::Database;

pub struct TestCommandHandler {
    database: Option<Database>
}

impl TestCommandHandler {
    pub fn temp() -> TestCommandHandler {
        TestCommandHandler {
            database: None
        }
    }
}

#[async_trait]
impl CommandHandler for TestCommandHandler {
    async fn handle(&self, interaction: &ApplicationCommandInteraction, ctx: &Context, db_conn: &PooledConn) -> Result<(), Box<dyn Error>> {
        

        let mut m: String = String::from("_");
        let users: Vec<User> = vec![];

        for i in 0..users.len() {
            m.push_str(format!("id: {}\nmoney: {}\ndistance traveled: {}", users[i].user_id, users[i].money, users[i].distance_traveled).as_str());
        }

        interaction.create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content(m))
        }).await?;

        Ok(())
    }

    fn get_command_description(&self) -> String {
        String::from("test the bot 2")
    }

    fn get_command_name(&self) -> String {
        String::from("test")
    }
}