extern crate dotenv;
use std::env;
use std::error::Error;
use serenity::async_trait;
use serenity::framework::standard::Args;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};
use serde::{Serialize, Deserialize};
use urlencoding::encode;

#[group]
#[commands(ping, pong, ranked)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to read .env file :(");
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("missing token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn pong(ctx: &Context, msg: &Message) -> CommandResult {
    print!("HAHAHAHHAH");
    msg.reply(ctx, "Ping!").await?;

    Ok(())
}

#[command]
async fn ranked(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    println!("YOOOOOO!!!");
    let summ_name = args.single_quoted::<String>().unwrap();
    println!("{}", summ_name);
    let encoded = encode(&summ_name);
    let ranks = get_ranks(&encoded).await?;
    for rank in ranks {
        msg.reply(ctx, format!("{}", rank)).await?;
    }

    Ok(())
}

async fn get_summoner(summ_name: &str, api_key: &str) -> Result<Summoner, Box<dyn Error>> {
    let res: Summoner = reqwest::get(format!("https://euw1.api.riotgames.com/lol/summoner/v4/summoners/by-name/\
        {}?api_key={}", summ_name, api_key))
        .await?
        .json()
        .await?;
    println!("{:?}", res);
    Ok(res)

}

async fn get_ranks(summ_name: &str) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
    let api_key = env::var("RIOT_API_KEY").expect("RIOT_API_KEY must be set");
    let summoner = get_summoner(summ_name, &api_key).await.unwrap();
    let res = reqwest::get(format!("https://euw1.api.riotgames.com/lol/league/v4/entries/\
        by-summoner/{}?api_key={}", summoner.id, api_key))
            .await?
            .text()
            .await?;
    println!("{}", res);
    let res_json: Vec<Rank> = serde_json::from_str(&res).unwrap();
    println!("{:?}", res_json);
    let msgs = res_json.iter().map(|r| {
        format!("{}, {} {}, points: {}, wins: {}, losses: {}", r.queueType, r.tier, r.rank, r.leaguePoints, r.wins, r.losses)
    }).collect::<Vec<String>>();
    Ok(msgs)
}

#[derive(Debug, Serialize, Deserialize)]
struct Rank {
    queueType: String,
    tier: String,
    rank: String,
    summonerName: String,
    leaguePoints: i32,
    wins: i32,
    losses: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Summoner {
    id: String,
    name: String,
    summonerLevel: i32,
    profileIconId: i32
}
