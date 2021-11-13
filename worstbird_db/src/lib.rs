pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn establish_connection_env() -> Result<PgConnection> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("No DATABASE_URL");
    Ok(PgConnection::establish(&database_url)?)
}

pub fn get_birds_year(sel_year: i32, con: &PgConnection) -> Result<Vec<(models::Bird, i32)>> {
    use crate::schema::bird::dsl::*;
    use crate::schema::worstbird_year::dsl::*;
    Ok(worstbird_year
        .filter(year.eq(sel_year))
        .inner_join(bird)
        .select(((id, name, description, assetid, url, width, height), votes))
        .load(con)?)
}

pub fn get_birds_month(
    sel_month: i32,
    sel_year: i32,
    con: &PgConnection,
) -> Result<Vec<(models::Bird, i32)>> {
    use crate::schema::bird::dsl::*;
    use crate::schema::worstbird_month::dsl::*;
    Ok(worstbird_month
        .filter(month.eq(sel_month))
        .filter(year.eq(sel_year))
        .inner_join(bird)
        .select(((id, name, description, assetid, url, width, height), votes))
        .load(con)?)
}

pub fn get_worstbird_year(sel_year: i32, con: &PgConnection) -> Result<Vec<(models::Bird, i32)>> {
    let birds_year = get_birds_year(sel_year, con)?;
    let max_votes = birds_year.iter().map(|e| e.1).max().unwrap_or(0);

    Ok(birds_year
        .into_iter()
        .filter(|e| e.1 == max_votes)
        .collect::<Vec<(models::Bird, i32)>>())
}

pub fn get_worstbird_month(
    sel_month: i32,
    sel_year: i32,
    con: &PgConnection,
) -> Result<Vec<models::Bird>> {
    let results = get_birds_month(sel_month, sel_year, con)?;
    let max_votes = results.iter().map(|e| e.1).max().unwrap_or(0);
    Ok(results
        .iter()
        .filter(|e| e.1 == max_votes)
        .map(|e| e.0.clone())
        .collect::<Vec<models::Bird>>())
}
