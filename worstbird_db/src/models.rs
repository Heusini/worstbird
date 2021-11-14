use crate::schema::bird;
use crate::schema::worstbird_month;
use crate::schema::worstbird_year;
use serde::Serialize;
#[derive(Debug, Queryable, Serialize, Clone)]
pub struct Bird {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub assetid: String,
    pub url: String,
    pub width: i32,
    pub height: i32,
}

#[derive(Queryable, Insertable, Debug)]
#[table_name = "worstbird_month"]
pub struct WBMonth {
    pub bird_id: i32,
    pub month: i32,
    pub year: i32,
    pub votes: i32,
}

#[derive(Queryable, Insertable)]
#[table_name = "worstbird_year"]
pub struct WBYear {
    pub bird_id: i32,
    pub year: i32,
    pub votes: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "bird"]
pub struct BirdEntry {
    pub url: String,
    pub name: String,
    pub description: String,
    pub assetid: String,
    pub width: i32,
    pub height: i32,
}
