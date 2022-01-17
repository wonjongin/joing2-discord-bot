use crate::error::create_error;
use indoc::{formatdoc, indoc};
use serenity::builder::CreateApplicationCommand;
use serenity::http::Http;
use serenity::model::interactions::application_command::{
  ApplicationCommandInteraction, ApplicationCommandOptionType,
};
use serenity::model::interactions::InteractionResponseType;
use serenity::model::prelude::application_command::ApplicationCommandInteractionDataOptionValue;

pub fn delete_register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
  command
    .name("삭제")
    .description("메시지를 삭제하세요")
    .create_option(|option| {
      option
        .name("count")
        .description("삭제할 메시지 개수를 입력하세요")
        .kind(ApplicationCommandOptionType::Integer)
        .required(true)
    })
}

pub async fn delete_controller(
  http: impl AsRef<Http>,
  command: &ApplicationCommandInteraction,
  options: Vec<ApplicationCommandInteractionDataOptionValue>,
) -> () {
  if command.data.name.as_str() == "삭제" {
    if let ApplicationCommandInteractionDataOptionValue::Integer(count) = &options[0] {
      let msgs = command
        .channel_id
        .messages(&http, |r| r.limit(*count as u64))
        .await
        .expect("Unable to get recent messages");
      command
        .channel_id
        .delete_messages(&http, msgs)
        .await
        .expect("Can't delete messages");
      command
        .create_interaction_response(&http, |response| {
          response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|msg| {
              msg.content(formatdoc!(
                r#"
                /삭제 {count}
                {count}개의 메시지를 삭제했어요."#,
                count = count,
              ))
            })
        })
        .await
        .expect("Error on delete response")
    } else {
      command
        .create_interaction_response(&http, |response| {
          response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|msg| {
              create_error(
                msg,
                "Syntax Error",
                indoc!(
                  r#"
                  인자로 숫자를 넣어주세요
                  ex) /삭제 10"#
                ),
              )
            })
        })
        .await
        .expect("Error on delete response");
    }
  } else {
    ()
  }
}
