use serenity::builder::CreateInteractionResponseData;
use serenity::utils::Colour;

pub fn create_error<'a>(
  msg: &'a mut CreateInteractionResponseData,
  title: &'a str,
  description: &'a str,
) -> &'a mut CreateInteractionResponseData {
  msg.create_embed(|embed| {
    embed
      .title(title)
      .description(description)
      .colour(Colour::RED)
  })
}
