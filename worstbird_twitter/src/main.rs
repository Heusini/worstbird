#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

extern crate dotenv;
use dotenv::dotenv;
use std::env;
use twitter_api::{SigningKey, TwitterApi};

use rocket::form::FromForm;
use rocket::routes;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};

#[derive(FromForm, Serialize, Deserialize)]
struct TweetMSG {
    size: i32,
    text: String,
}

#[derive(Serialize, Deserialize)]
struct ResponseJson {
    response_code: i32,
    response_msg: String,
}

#[post("/tweet", format = "json", data = "<twitter_msg>")]
async fn tweet(
    twitter: &State<TwitterApi>,
    twitter_msg: Json<TweetMSG>,
) -> Result<Json<ResponseJson>, Json<ResponseJson>> {
    if twitter_msg.size <= 280
        && twitter_msg.size > 0
        && twitter_msg.text.len() < 280
        && twitter_msg.text.len() > 0
    {
        let response = twitter.tweet(&twitter_msg.text);
        match response.await {
            Ok(_) => Ok(ResponseJson {
                response_code: 0,
                response_msg: twitter_msg.text.clone(),
            }
            .into()),
            Err(e) => Err(ResponseJson {
                response_code: 1,
                response_msg: e.to_string(),
            }
            .into()),
        }
    } else {
        Err(ResponseJson {
            response_code: 2,
            response_msg: "Message to big or to small to Tweet".to_string(),
        }
        .into())
    }
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let twitter_api = TwitterApi::new(
        &env::var("CONSUMER_KEY").unwrap(),
        &env::var("TOKEN").unwrap(),
        SigningKey::new(
            &env::var("CONSUMER_SECRET").unwrap(),
            &env::var("TOKEN_SECRET").unwrap(),
        ),
    );

    rocket::build()
        .mount("/", routes![tweet])
        .manage(twitter_api)
}
