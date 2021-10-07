#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

extern crate dotenv;
use dotenv::dotenv;
use std::env;
use twitter_api::{SigningKey, TwitterApi};

use rocket::routes;
use rocket::State;

#[post("/tweet")]
async fn tweet(twitter: &State<TwitterApi>) -> String {
    let res = twitter.tweet("lol");
    res.await.unwrap()
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
