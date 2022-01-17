mod delete;
mod error;
mod mention;
mod random;

extern crate dotenv;

use dotenv::dotenv;

use crate::delete::{delete_controller, delete_register};
use crate::mention::{mention_controller, mention_register};
use crate::random::{random_controller, random_register};
use serenity::async_trait;
use serenity::model::gateway::{Activity, Ready};
use serenity::model::id::GuildId;
use serenity::model::interactions::application_command::ApplicationCommand;
use serenity::model::interactions::Interaction;
use serenity::model::prelude::application_command::ApplicationCommandInteractionDataOptionValue;
use serenity::prelude::*;
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
    if let Interaction::ApplicationCommand(command) = interaction {
      let mut options: Vec<ApplicationCommandInteractionDataOptionValue> = Vec::new();
      for i in 0..command.data.options.len() {
        options.push(
          command
            .data
            .options
            .get(i)
            .expect("Expected option")
            .resolved
            .as_ref()
            .expect("Error")
            .clone(),
        );
      }

      mention_controller(&ctx.http, &ctx.http, &command, options.clone()).await;
      random_controller(&ctx.http, &command, options.clone()).await;
      delete_controller(&ctx.http, &command, options.clone()).await;
    }
  }

  async fn ready(&self, ctx: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
    ctx
      .set_activity(Activity::playing(" '/' 입력 | 슬래시 커맨드 테스트 중"))
      .await;

    let guild_id = GuildId(
      env::var("GUILD_ID")
        .expect("Expected GUILD_ID in environment")
        .parse()
        .expect("GUILD_ID must be an integer"),
    );

    let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
      commands
        .create_application_command(|command| random_register(command))
        .create_application_command(|command| mention_register(command))
        .create_application_command(|command| delete_register(command))
    })
    .await;

    println!(
      "I now have the following guild slash commands: {:#?}",
      commands
    );

    let guild_command =
      ApplicationCommand::create_global_application_command(&ctx.http, |command| {
        command.name("테스트").description("테스트용")
      })
      .await;

    println!(
      "I created the following global slash command: {:#?}",
      guild_command
    );
  }
}

#[tokio::main]
async fn main() {
  dotenv().ok();
  // Configure the client with your Discord bot token in the environment.
  let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

  // The Application Id is usually the Bot User Id.
  let application_id: u64 = env::var("APPLICATION_ID")
    .expect("Expected an application id in the environment")
    .parse()
    .expect("application id is not a valid id");

  // Build our client.
  let mut client = Client::builder(token)
    .event_handler(Handler)
    .application_id(application_id)
    .await
    .expect("Error creating client");

  // Finally, start a single shard, and start listening to events.
  //
  // Shards will automatically attempt to reconnect, and will perform
  // exponential backoff until it reconnects.
  if let Err(why) = client.start().await {
    println!("Client error: {:?}", why);
  }
}
