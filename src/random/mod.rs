use crate::error::create_error;
use indoc::{formatdoc, indoc};
use rand::Rng;
use serenity::builder::CreateApplicationCommand;
use serenity::http::Http;
use serenity::model::interactions::application_command::{
  ApplicationCommandInteraction, ApplicationCommandOptionType,
};
use serenity::model::interactions::InteractionResponseType;
use serenity::model::prelude::application_command::ApplicationCommandInteractionDataOptionValue;
use std::thread;
use std::time::Duration;

pub fn random_register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
  command
    .name("골라")
    .description("여러 선택지 중에서 한 선택지를 골라드려요")
    .create_option(|option| {
      option
        .name("args")
        .description("선택지를 입력해주세요( , 로 구분해주세요)")
        .kind(ApplicationCommandOptionType::String)
        .required(true)
    })
}

pub async fn random_controller(
  http: impl AsRef<Http>,
  command: &ApplicationCommandInteraction,
  options: Vec<ApplicationCommandInteractionDataOptionValue>,
) -> () {
  if command.data.name.as_str() == "골라" {
    command
      .create_interaction_response(&http, |response| {
        response
          .kind(InteractionResponseType::ChannelMessageWithSource)
          .interaction_response_data(|msg| {
            // println!("{:#?}", options);
            if let ApplicationCommandInteractionDataOptionValue::String(str_args) = &options[0] {
              thread::sleep(Duration::from_millis(1));
              let mut rng = rand::thread_rng();
              let splited_str = str_args.split(",");
              let splited_str_vec: Vec<&str> = splited_str.collect();
              let picked_num: usize = rng.gen_range(0..splited_str_vec.len());
              msg.content(formatdoc!(
                r#"
                /골라 {}
                저는 {}! 이거 골랐어요."#,
                str_args,
                splited_str_vec[picked_num]
              ))
            } else {
              create_error(
                msg,
                "Syntax Error",
                indoc!(
                  r#"
              , 로 구분을 잘 하였는지 확인하여 주십시오
              ex) /골라 후보1, 후보2, 후보3, ... , 후보n"#
                ),
              )
            }
          })
      })
      .await
      .expect("Error on random response")
  } else {
    ()
  }
}
