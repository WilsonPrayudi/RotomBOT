use slack_hook2::{Slack, PayloadBuilder};

#[tokio::main]
async fn main() {
  let slack = Slack::new("https://hooks.slack.com/services/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").unwrap();
  let p = PayloadBuilder::new()
    .text("Hi, my name is rotom_bot")
    .channel("#slack-connector-for-telkoms-atlassian-cloud-platform")
    .username("rotom_bot")
    .icon_emoji(":chart_with_upwards_trend:")
    .build()
    .unwrap();

    let res = slack.send(&p).await;
    match res {
        Ok(()) => println!("ok"),
        Err(x) => println!("ERR: {:?}",x)
      }
}