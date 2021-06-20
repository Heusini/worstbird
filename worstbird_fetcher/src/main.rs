pub mod bird;

extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::bird::web::*;
use worstbird_db::models::BirdEntry;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use chrono::prelude::*;
use std::time::Duration;
use worstbird_db::models;

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

fn get_new_bird() -> Result<BirdEntry> {
    let (url, data) = suprise_me("https://ebird.org/species/surprise-me")?;
    let name = get_bird_name(&data);
    let description = get_description(&data);
    let embed_id = get_embbed(&data);

    if embed_id.is_ok() && name.is_ok() && description.is_ok() {
        if let Ok((width, height)) = get_image_size(&embed_id.as_ref().unwrap()) {
            let bird = BirdEntry {
                url,
                name: name.unwrap(),
                description: description.unwrap(),
                assetid: embed_id.unwrap(),
                width: width as i32,
                height: height as i32,
            };

            Ok(bird)
        } else {
            Err("Could not get image size".into())
        }
    } else {
        Err("Missing entries".into())
    }
}

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("No DATABASE_URL");
    PgConnection::establish(&database_url).expect(&format!("Error connectiong to {}", database_url))
}

fn new_month_create(con: &PgConnection) {
    use worstbird_db::schema::bird::dsl::*;
    use worstbird_db::schema::worstbird_month::dsl::*;
    let mut added_birds = 0;
    while added_birds < 5 {
        println!("added birds {}", added_birds);
        if let Ok(ibird) = get_new_bird() {
            println!("{:?}", ibird.name);
            match diesel::insert_into(bird)
                .values(ibird)
                .get_result::<models::Bird>(con)
            {
                Ok(birdy) => {
                    // could be wrong if we cannot add to worstbird_month table
                    added_birds += 1;
                    let now = Utc::now();

                    diesel::insert_into(worstbird_month)
                        .values(models::WBMonth {
                            month: now.month() as i32,
                            year: now.year() as i32,
                            bird_id: birdy.id,
                            votes: 0,
                        })
                        .execute(con)
                        .unwrap();
                }
                Err(_) => (),
            };
        }
    }
}

fn new_year_create(con: &PgConnection) {
    use worstbird_db::schema::worstbird_month::dsl::*;
    let mut now = Utc::now();
    now = now.with_year(2022).unwrap();
    println!("{:?}", now);
    let worst_birds: Vec<models::WBMonth> = worstbird_month
        .filter(year.eq(now.year() - 1 as i32))
        .load::<models::WBMonth>(con)
        .unwrap();

    for i in 1..=12 {
        if let Some(worstbird) = worst_birds
            .iter()
            .filter(|e| e.month == i)
            .max_by_key(|e| e.votes)
        {
            println!("{:?}", worstbird);
            use worstbird_db::schema::worstbird_year::dsl::*;
            diesel::insert_into(worstbird_year)
                .values(models::WBYear {
                    bird_id: worstbird.bird_id,
                    year: now.year() - 1,
                    votes: 0,
                })
                .execute(con)
                .unwrap();
        }
    }
}

fn main() -> ! {
    // println!("{:?}", calc_time_end_of_month());
    // println!("{:?}", create_new_birds(5));

    use worstbird_db::schema::worstbird_month::dsl::*;
    println!("{:?}", get_image_size("39765651"));
    loop {
        let now = Utc::now();
        println!("Executing {:?}", now);
        let connection = establish_connection();
        let results = worstbird_month
            .filter(month.eq(now.month() as i32))
            .filter(year.eq(now.year() as i32))
            .limit(1)
            .load::<models::WBMonth>(&connection)
            .unwrap();
        if results.len() == 0 {
            new_month_create(&connection);
        }
        if now.month() == 6 {
            new_year_create(&connection);
        }
        let wait_time = calc_time_to_end_of_month();
        println!("Waiting for next call in {:?}", wait_time);
        std::thread::sleep(wait_time);

        // check if create new month
        // if true
        //  - add new birds to new month
        //  - add worst bird to year
        // else
        //  - calculate sleep time
        //  - sleep
        // check if create new year
        // if true
        //  - create new year
        // calculate sleep
        // sleep
    }
}
