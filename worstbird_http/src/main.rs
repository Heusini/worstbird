#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_contrib;

pub mod session;
pub mod tera_models;
pub mod util;
extern crate dotenv;
use dotenv::dotenv;

use std::net::IpAddr;
use std::net::SocketAddr;

use diesel::prelude::*;

use chrono::prelude::*;
use chrono::Month;
use num_traits::FromPrimitive;

use crate::tera_models::{TeraDownVote, TeraTemplate};
use rocket::http::Cookies;
use rocket::response::Redirect;
use rocket::State;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

use chashmap::CHashMap;

use crate::session::{MonthCookie, YearCookie};
use crate::util::*;
use worstbird_db::models;

#[get("/downvote/<birdid>/<sel_year>")]
fn downvote_year_user(
    year_cookie: YearCookie,
    conn: PgDatabase,
    sel_year: i32,
    birdid: u32,
) -> Result<Template, Box<dyn std::error::Error>> {
    use worstbird_db::schema::bird::dsl::*;
    use worstbird_db::schema::worstbird_year::dsl::*;
    let downvoted_bird: (models::Bird, i32) = worstbird_year
        .filter(bird_id.eq(birdid as i32))
        .inner_join(bird)
        .select(((id, name, description, assetid, url, width, height), votes))
        .get_result(&*conn)?;

    let previously_downvoted: String = bird
        .filter(id.eq(year_cookie.0 as i32))
        .select(name)
        .get_result(&*conn)?;

    let error_message = format!(
        "You cannot downvote again as you already voted for {} in {}",
        previously_downvoted, sel_year
    );

    let context = TeraDownVote {
        bird: downvoted_bird.0,
        votes: downvoted_bird.1,
        referer: format!("/{}", sel_year),
        error_message: Some(error_message),
        month: None,
        year: Some(sel_year),
    };

    Ok(Template::render("already_downvoted", &context))
}

#[get("/downvote/<birdid>/<sel_year>", rank = 2)]
fn downvote_year(
    state: State<CHashMap<IpAddr, UserVoteCount>>,
    remote_addr: SocketAddr,
    conn: PgDatabase,
    mut cookies: Cookies,
    sel_year: i32,
    birdid: u32,
) -> Result<Template, Box<dyn std::error::Error>> {
    let now = Local::now();
    check_year(sel_year)?;

    if now.year() - 1 != sel_year && now.month() != 1 {
        return Err("Not allowed to vote this month".into());
    }
    use worstbird_db::schema::worstbird_year::dsl::*;

    let wb_year: models::WBYear = diesel::update(worstbird_year.find((birdid as i32, sel_year)))
        .set(votes.eq(votes + 1))
        .get_result(&*conn)?;

    use worstbird_db::schema::bird::dsl::*;
    let my_bird = bird.filter(id.eq(wb_year.bird_id)).get_result(&*conn)?;

    let mut context = TeraDownVote {
        bird: my_bird,
        votes: wb_year.votes,
        referer: format!("/{}", sel_year),
        error_message: None,
        month: None,
        year: Some(sel_year),
    };

    set_cookie("year", &mut cookies, birdid)?;

    if get_ip_vote_count(state, remote_addr) >= MAX_IP_VOTE {
        let error_message = format!(
            "You cannot downvote again as your ip exceeded the month's maximum of {}",
            MAX_IP_VOTE
        );
        context.error_message = Some(error_message);
        Ok(Template::render("already_downvoted", &context))
    } else {
        Ok(Template::render("downvoted", &context))
    }
}

#[get("/downvote/<birdid>/<sel_year>/<sel_month>")]
fn downvote_month_user(
    month_cookie: MonthCookie,
    conn: PgDatabase,
    sel_year: i32,
    sel_month: u32,
    birdid: u32,
) -> Result<Template, Box<dyn std::error::Error>> {
    check_year(sel_year)?;
    check_month(sel_month)?;
    use worstbird_db::schema::bird::dsl::*;
    use worstbird_db::schema::worstbird_month::dsl::*;
    let downvoted_bird: (models::Bird, i32) = worstbird_month
        .filter(bird_id.eq(birdid as i32))
        .inner_join(bird)
        .select(((id, name, description, assetid, url, width, height), votes))
        .get_result(&*conn)?;

    let previously_downvoted: String = bird
        .filter(id.eq(month_cookie.0 as i32))
        .select(name)
        .get_result(&*conn)?;

    let month_name = Month::from_u32(sel_month)
        .ok_or("Error could not convert month num to enum chrono::Month")?
        .name()
        .to_string();
    let error_message = format!(
        "You cannot vote again as you already voted for {} in {} {}",
        previously_downvoted, &month_name, sel_year
    );
    let context = TeraDownVote {
        bird: downvoted_bird.0,
        votes: downvoted_bird.1,
        referer: format!("/{}/{}", sel_year, sel_month),
        error_message: Some(error_message),
        month: Some(month_name),
        year: Some(sel_year),
    };

    Ok(Template::render("already_downvoted", &context))
}

#[get("/downvote/<birdid>/<sel_year>/<sel_month>", rank = 2)]
fn downvote_month(
    state: State<CHashMap<IpAddr, UserVoteCount>>,
    remote_addr: SocketAddr,
    conn: PgDatabase,
    mut cookies: Cookies,
    sel_year: i32,
    sel_month: u32,
    birdid: u32,
) -> Result<Template, Box<dyn std::error::Error>> {
    check_year(sel_year)?;
    check_month(sel_month)?;

    let now = Local::now();
    if now.year() != sel_year && now.month() != sel_month {
        return Err("Voting for this month is not allowed".into());
    }

    use worstbird_db::schema::worstbird_month::dsl::*;

    let wb_month: models::WBMonth =
        diesel::update(worstbird_month.find((birdid as i32, sel_month as i32, sel_year)))
            .set(votes.eq(votes + 1))
            .get_result(&*conn)?;

    use worstbird_db::schema::bird::dsl::*;
    let my_bird = bird.filter(id.eq(wb_month.bird_id)).get_result(&*conn)?;

    let mut context = TeraDownVote {
        bird: my_bird,
        votes: wb_month.votes,
        referer: format!("/{}/{}", sel_year, sel_month),
        error_message: None,
        month: Some(
            Month::from_u32(sel_month)
                .ok_or("Error could not convert month num to enum chrono::Month")?
                .name()
                .to_string(),
        ),
        year: Some(sel_year),
    };

    set_cookie("month", &mut cookies, birdid)?;

    if get_ip_vote_count(state, remote_addr) >= MAX_IP_VOTE {
        let error_message = format!(
            "You cannot downvote again as your ip exceeded the month's maximum of {}",
            MAX_IP_VOTE
        );
        context.error_message = Some(error_message);
        Ok(Template::render("already_downvoted", &context))
    } else {
        Ok(Template::render("downvoted", &context))
    }
}

// implement
// fn get_months(conn: PgDatabase) -> Result<Vec<Month>, Box<dyn std::error::Error>> {}

#[get("/<sel_year>")]
fn get_worstbird_year(
    conn: PgDatabase,
    sel_year: i32,
) -> Result<Template, Box<dyn std::error::Error>> {
    check_year(sel_year)?;
    let now = Local::now();
    let mut distinct_years = get_distinct_years(&*conn)?;
    distinct_years.push(now.year());

    let distinct_months: Vec<u32> = get_distinct_months(&conn, sel_year)?;

    use worstbird_db::schema::bird::dsl::*;
    use worstbird_db::schema::worstbird_year::dsl::*;
    let birds: Vec<(models::Bird, i32)> = worstbird_year
        .filter(year.eq(sel_year as i32))
        .inner_join(bird)
        .select(((id, name, description, assetid, url, width, height), votes))
        .load(&*conn)
        .expect("error");

    let context = TeraTemplate {
        sel_year,
        sel_month: None,
        sel_month_path: None,
        years: distinct_years,
        months: distinct_months
            .iter()
            .map(|e| Month::from_u32(*e).unwrap())
            .map(|e| format!("{}", e.name().chars().take(3).collect::<String>()))
            .collect(),
        months_num: distinct_months,
        max_vote: birds.iter().map(|e| e.1).max().unwrap_or(0) as u32,
        birds,
    };

    if sel_year == now.year() - 1 && now.month() == 1 {
        Ok(Template::render("vote", &context))
    } else {
        Ok(Template::render("display", &context))
    }
}

#[get("/<sel_year>/<sel_month>")]
fn get_worstbird_month(
    conn: PgDatabase,
    sel_year: i32,
    sel_month: u32,
) -> Result<Template, Box<dyn std::error::Error>> {
    check_year(sel_year)?;
    check_month(sel_month)?;

    let now = Local::now();
    let mut distinct_years = get_distinct_years(&conn)?;
    distinct_years.push(now.year());

    let distinct_months: Vec<u32> = get_distinct_months(&conn, sel_year).unwrap();

    use worstbird_db::schema::bird::dsl::*;
    use worstbird_db::schema::worstbird_month::dsl::*;

    let birds: Vec<(models::Bird, i32)> = worstbird_month
        .filter(year.eq(sel_year))
        .filter(month.eq(sel_month as i32))
        .inner_join(bird)
        .select(((id, name, description, assetid, url, width, height), votes))
        .load(&*conn)
        .expect("error");

    let context = TeraTemplate {
        sel_year,
        sel_month: Some(format!(
            "{}",
            Month::from_u32(sel_month)
                .ok_or("could not parse month")?
                .name()
                .chars()
                .take(3)
                .collect::<String>()
        )),
        sel_month_path: Some(format!("/{}", sel_month)),
        years: distinct_years,
        months: distinct_months
            .iter()
            .map(|e| month_to_shortmonth(*e))
            .collect(),
        months_num: distinct_months,
        max_vote: birds.iter().map(|e| e.1).max().unwrap_or(0) as u32,
        birds,
    };

    if now.month() == sel_month && now.year() == sel_year {
        Ok(Template::render("vote", &context))
    } else {
        Ok(Template::render("display", &context))
    }
}

#[get("/")]
fn get_index() -> Redirect {
    let now = Local::now();
    if now.month() == 1 {
        Redirect::to(format!("/{}", now.year()))
    } else {
        Redirect::to(format!("/{}/{}", now.year(), now.month()))
    }
}

fn get_distinct_years(conn: &PgConnection) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let distinct_years: Vec<DistinctYear> =
        diesel::sql_query("select distinct year from worstbird_year").load(&*conn)?;
    let years: Vec<i32> = distinct_years.iter().map(|e| e.year).collect();
    Ok(years)
}

fn get_distinct_months(
    conn: &PgConnection,
    year: i32,
) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let distinct_months: Vec<DistinctMonth> = diesel::sql_query(format!(
        "select distinct month from worstbird_month where year = {}",
        year
    ))
    .load(&*conn)?;
    let months: Vec<u32> = distinct_months.iter().map(|e| e.month as u32).collect();
    Ok(months)
}

#[database("pg_worstbird")]
struct PgDatabase(diesel::PgConnection);
fn main() {
    dotenv().ok();

    let ip_map: CHashMap<IpAddr, UserVoteCount> = CHashMap::new();

    rocket::ignite()
        .attach(Template::fairing())
        .attach(PgDatabase::fairing())
        .mount("/www", StaticFiles::from("www/"))
        .mount(
            "/",
            routes![
                get_index,
                downvote_year,
                downvote_month,
                downvote_month_user,
                downvote_year_user,
                get_worstbird_year,
                get_worstbird_month
            ],
        )
        .manage(ip_map)
        .launch();
}
