use indoc::formatdoc;
use serenity::builder::CreateApplicationCommand;
use serenity::http::{CacheHttp, Http};
use serenity::model::prelude::application_command::{
  ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
  ApplicationCommandOptionType,
};
use serenity::model::prelude::InteractionResponseType;
use serenity::model::user::User;

pub fn mention_register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
  command
    .name("m")
    .description("유저를 멘션하세요!")
    .create_option(|option| {
      option
        .name("user")
        .description("멘션할 유저를 입력하세요")
        .kind(ApplicationCommandOptionType::User)
        .required(true)
    })
}

pub async fn mention_controller(
  http: impl AsRef<Http>,
  cache_http: impl CacheHttp,
  command: &ApplicationCommandInteraction,
  options: Vec<ApplicationCommandInteractionDataOptionValue>,
) -> () {
  if command.data.name.as_str() == "m" {
    if let ApplicationCommandInteractionDataOptionValue::User(user, _member) = &options[0] {
      mention_to_dm(&user, &cache_http).await;
      command
        .create_interaction_response(&http, |response| {
          response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|msg| {
              msg.content(formatdoc!(
                r#"
            /m <@!{}>
            멘션을 DM으로도 보냈어요! 📨"#,
                user.id
              ))
            })
        })
        .await
        .expect("Error on mention response")
    }
  } else {
    ()
  }
}

pub async fn mention_to_dm(user: &User, http: impl CacheHttp) {
  user
    .direct_message(&http, |dm| {
      println!("here1.1");
      dm.embed(|embed| {
        println!("here1.2");
        embed.title("🛎 멘션되었어요!").description(format!(
          "📩  {} 에게 멘션되었어요! 메시지를 확인하세요!!",
          user.name
        ))
      })
    })
    .await
    .expect("error in dm");
}
