use chrono::prelude::*;
use indoc::formatdoc;
use serde_json::Value;
use serenity::builder::CreateApplicationCommand;
use serenity::http::Http;
use serenity::model::prelude::application_command::{
  ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
  ApplicationCommandOptionType,
};
use serenity::model::prelude::InteractionResponseType;
use serenity::utils::Colour;
use std::collections::HashMap;

/// Convert DateTime to how long ago
///
/// # Arguments
///
/// * `start_play: &str ` - the time when start playing game
fn time_format(start_play: &str) -> String {
  let now: DateTime<Utc> = Utc::now();
  let play = start_play
    .parse::<DateTime<Utc>>()
    .expect("Unable to parse timestamp");
  let sub = now.timestamp() - play.timestamp();
  if sub > 86400 {
    format!("{}일 전", sub / 86400)
  } else if sub > 3600 {
    format!("{}시간 전", sub / 3600)
  } else if sub > 60 {
    format!("{}분 전", sub / 3600)
  } else {
    format!("{}초 전", sub)
  }
}

pub fn lol_history_register(
  command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
  command
    .name("롤전적")
    .description("롤 전적을 확인하세요!")
    .create_option(|option| {
      option
        .name("소환사명")
        .description("소환사명을 입력하세요")
        .kind(ApplicationCommandOptionType::String)
        .required(true)
    })
}

pub async fn lol_history_controller(
  http: impl AsRef<Http>,
  command: &ApplicationCommandInteraction,
  options: Vec<ApplicationCommandInteractionDataOptionValue>,
) -> () {
  if command.data.name.as_str() == "롤전적" {
    if let ApplicationCommandInteractionDataOptionValue::String(summoner_name) = &options[0] {
      let his = reqwest::get(format!(
        "https://www.lolog.me/kr/shortcut/{}",
        summoner_name
      ))
      .await
      .expect("Unable to get data from lolog")
      .json::<HashMap<String, Value>>()
      .await
      .expect("Unable to parse json data from lolog");
      let mut matches_text = String::new();
      for e in his["matches"]
        .clone()
        .as_array()
        .expect("Unable to get matches data")
        .iter()
      {
        let participant = &e["participant"]
          .as_object()
          .expect("Cannot parse participant");
        let win_code = participant["win_my"]
          .as_u64()
          .expect("Cannot parse win data");
        if win_code == 21 || win_code == 11 {
          matches_text.push_str("```🏆 승 ");
        } else {
          matches_text.push_str("```❌ 패 ");
        }
        let kills = participant["kills"]
          .as_u64()
          .expect("Cannot parse kills data");
        let deaths = participant["deaths"]
          .as_u64()
          .expect("Cannot parse deaths data");
        let assists = participant["assists"]
          .as_u64()
          .expect("Cannot parse assists data");
        matches_text.push_str(
          format!(
            "KDA:{:2}/{:2}/{:2} 평점:{:.2}  ",
            kills,
            deaths,
            assists,
            ((kills as f64 + assists as f64) / deaths as f64) as f64
          )
          .as_str(),
        );
        matches_text.push_str(
          time_format(&e["start_time"].as_str().expect("Unable to parse to str")).as_str(),
        );
        matches_text.push_str("```\n");
      }
      println!("{:#?}", his);
      command
        .create_interaction_response(&http, |response| {
          response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|msg| {
              msg.create_embed(|embed| {
                embed
                  .title(
                    &his["summoner_name"]
                      .as_str()
                      .expect("Unable to convert to str"),
                  )
                  .url(format!(
                    "https://lolog.me/kr/user/{}",
                    &his["summoner_name"]
                      .as_str()
                      .expect("Unable to convert to str")
                  ))
                  .colour(Colour::ORANGE)
                  .field(
                    "솔로 랭크",
                    format!(
                      "{} {}",
                      &his["solo_tier"].as_str().expect("Unable to convert to str"),
                      &his["solo_rank"].as_str().expect("Unable to convert to str"),
                    ),
                    true,
                  )
                  .field(
                    "자유 랭크",
                    format!(
                      "{} {}",
                      &his["flex_tier"].as_str().expect("Unable to convert to str"),
                      &his["flex_rank"].as_str().expect("Unable to convert to str"),
                    ),
                    true,
                  )
                  .field("전적", matches_text, false)
                  .field(
                    "링크",
                    formatdoc!(
                      r#"
                      [{user} LoLog.me](https://lolog.me/kr/user/{user})
                      [{user} op.gg](https://www.op.gg/summoner/userName={user})
                      "#,
                      user = &his["summoner_name"]
                        .as_str()
                        .expect("Unable to convert to str")
                    ),
                    false,
                  )
                  .footer(|footer| {
                    footer
                      .text("Data from LoLog.me")
                      .icon_url("https://www.lolog.me/favicon/favicon-16x16.png")
                  })
              })
            })
        })
        .await
        .expect("Error on mention response")
    }
  } else {
    ()
  }
}
