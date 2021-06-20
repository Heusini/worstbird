#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_contrib;

extern crate dotenv;
use dotenv::dotenv;

use diesel::prelude::*;
use diesel::sql_types::Integer;

use chrono::prelude::*;
use chrono::Month;
use num_traits::FromPrimitive;

use rocket::response::Redirect;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use worstbird_db::models;

use serde::Serialize;

#[get("/<year>/<birdid>")]
fn downvote_year(year: u32, birdid: String) {}

#[get("/<year>/<month>/<birdid>")]
fn downvote_month(year: u32, month: u32, birdid: String) {}

#[derive(QueryableByName)]
struct DistinctYear {
    #[sql_type = "Integer"]
    year: i32,
}

#[derive(QueryableByName)]
struct DistinctMonth {
    #[sql_type = "Integer"]
    month: i32,
}

#[derive(Serialize)]
struct TemplateInfo {
    sel_year: u32,
    years: Vec<i32>,
    months: Vec<String>,
    months_num: Vec<u8>,
    birds: Vec<models::Bird>,
}

#[derive(Serialize)]
struct TemplateInfoEmpty {
    sel_year: u32,
    years: Vec<i32>,
    months: Vec<String>,
}

#[get("/<sel_year>")]
fn get_worstbird_year(conn: PgDatabase, sel_year: u32) -> Template {
    let distinct_years: Vec<DistinctYear> =
        diesel::sql_query("select distinct year from worstbird_year")
            .load(&*conn)
            .expect("Query failed");

    let distinct_months: Vec<DistinctMonth> = diesel::sql_query(format!(
        "select distinct month from worstbird_month where year = {}",
        sel_year
    ))
    .load(&*conn)
    .expect("month query failed");

    use worstbird_db::schema::bird::dsl::*;
    use worstbird_db::schema::worstbird_year::dsl::*;
    let birds: Vec<models::Bird> = worstbird_year
        .filter(year.eq(sel_year as i32))
        .inner_join(bird)
        .select((id, name, description, assetid, url, width, height))
        .load(&*conn)
        .expect("error");

    // if birds.len() == 0 {
    //     let context = TemplateInfoEmpty {
    //         sel_year,
    //         years: distinct_years.iter().map(|e| e.year).collect(),
    //         months: distinct_months
    //             .iter()
    //             .map(|e| Month::from_u32(e.month as u32).unwrap())
    //             .map(|e| format!("{}", e.name().chars().take(3).collect::<String>()))
    //             .collect(),
    //     };
    //     Template::render("year", &context)
    // } else {
    let context = TemplateInfo {
        sel_year,
        years: distinct_years.iter().map(|e| e.year).collect(),
        months: distinct_months
            .iter()
            .map(|e| Month::from_u32(e.month as u32).unwrap())
            .map(|e| format!("{}", e.name().chars().take(3).collect::<String>()))
            .collect(),
        months_num: distinct_months.iter().map(|e| e.month as u8).collect(),
        birds,
    };
    Template::render("year", &context)
    // }
}

#[get("/")]
fn get_index() -> Redirect {
    let year_now = Utc::now().year();
    Redirect::to(format!("/{}", year_now))
}

#[get("/<year>/<month>")]
fn get_worstbird_month(year: u32, month: u32) {}

#[database("pg_worstbird")]
struct PgDatabase(diesel::PgConnection);
fn main() {
    dotenv().ok();
    rocket::ignite()
        .attach(Template::fairing())
        .attach(PgDatabase::fairing())
        .mount("/www", StaticFiles::from("www/"))
        .mount(
            "/",
            routes![
                // downvote_year,
                get_index,
                downvote_month,
                get_worstbird_year,
                get_worstbird_month
            ],
        )
        .launch();
}
