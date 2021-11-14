extern crate dotenv;
use dotenv::dotenv;
use std::env;
use twitter_api::{SigningKey, TwitterApi};

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use worstbird_db::*;

#[derive(Serialize, Deserialize)]
struct TweetMSG {
    size: i32,
    text: String,
}

#[derive(Serialize, Deserialize)]
struct ResponseJson {
    response_code: i32,
    response_msg: String,
}
async fn tweet(
    twitter: &TwitterApi,
    twitter_msg: TweetMSG,
) -> Result<(), Box<dyn std::error::Error>> {
    if twitter_msg.size <= 280
        && twitter_msg.size > 0
        && twitter_msg.text.len() < 280
        && twitter_msg.text.len() > 0
    {
        twitter.tweet(&twitter_msg.text).await?;
        Ok(())
    } else {
        return Err("Message to long".into());
    }
}

fn calc_time_to_end_of_month() -> Duration {
    let now = Utc::now();
    let new_date;
    if now.month() == 12 {
        new_date = Utc.ymd(now.year() + 1, 1, 1).and_hms(0, 5, 0);
    } else {
        new_date = Utc.ymd(now.year(), now.month() + 1, 1).and_hms(0, 5, 0);
    }

    let duration = new_date.signed_duration_since(now);
    duration.to_std().unwrap()
}

async fn send_post(twitter_api: &TwitterApi) -> Result<(), Box<dyn std::error::Error>> {
    let now = Utc::now();
    let month = now.month() as i32;
    // calculate last month
    let prev_month = get_previous_month(month);
    let prev_year = get_previous_year_if_necessary(month, now.year() as i32);
    let con = establish_connection_env()?;

    let birds = get_worstbird_month(prev_month, prev_year, &con)?;
    for bird in birds.into_iter() {
        let text = format!(
            "The worstbird of the month: {}\n{} {}\n{}",
            bird.name,
            "@daspodcastufo".to_string(),
            "worstbird.eu".to_string(),
            bird.url,
        );

        tweet(
            &twitter_api,
            TweetMSG {
                size: text.len() as i32,
                text,
            },
        )
        .await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let twitter_api = TwitterApi::new(
        &env::var("CONSUMER_KEY").unwrap(),
        &env::var("TOKEN").unwrap(),
        SigningKey::new(
            &env::var("CONSUMER_SECRET").unwrap(),
            &env::var("TOKEN_SECRET").unwrap(),
        ),
    );
    loop {
        let wait_time = calc_time_to_end_of_month();
        std::thread::sleep(wait_time);
        send_post(&twitter_api).await?;
    }
}

// takes the original month and returns the previous year if the month is january
fn get_previous_year_if_necessary(month: i32, year: i32) -> i32 {
    if month == 12 {
        year - 1
    } else {
        year
    }
}

fn get_previous_month(month: i32) -> i32 {
    let prev_month = ((month - 1) % 12) + ((month - 12) / 11) * -12;
    prev_month
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        for i in 1..12 {
            println!("{}|{}", i, get_previous_month(i));
        }
        assert!(true);
    }
    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn test_tweet() {
        dotenv().ok();
        let twitter_api = TwitterApi::new(
            &env::var("CONSUMER_KEY").unwrap(),
            &env::var("TOKEN").unwrap(),
            SigningKey::new(
                &env::var("CONSUMER_SECRET").unwrap(),
                &env::var("TOKEN_SECRET").unwrap(),
            ),
        );

        aw!(send_post(&twitter_api));
    }
}
