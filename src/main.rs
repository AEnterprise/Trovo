use futures::stream::StreamExt;
use std::{env, error::Error};
use twilight_gateway::{cluster::{Cluster}, Event, Intents};
use twilight_http::Client as HttpClient;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("Trovo starting...");
    let token = env::var("DISCORD_TOKEN")?;

    let cluster = Cluster::builder(&token, Intents::GUILD_MESSAGES)
        .build()
        .await?;

    let cluster_spawn = cluster.clone();
    tokio::spawn(async move {
        cluster_spawn.up().await;
        println!("Trovo connected to the gateway")
    });

    let http = HttpClient::new(&token);

    let mut events = cluster.events();
    while let Some((_, event)) = events.next().await {
        if let Event::MessageCreate(message) = event {
            if message.channel_id.0 == 743508461651886151 && message.content == ".r" {
                if let Err(e) = http.create_message(743508461651886151.into()).content("<@&743186303981453464> <@237624303581921281>").unwrap().await {
                    println!("Error: {}", e);
                } else {
                    if let Err(e) = http.delete_message(message.channel_id, message.id).await {
                        println!("Error: {}", e);
                    }
                }
            }
        }


    }

    Ok(())
}