use serde::Serialize;
use worstbird_db::models;
#[derive(Serialize)]
pub struct TemplateInfo {
    pub sel_year: u32,
    pub years: Vec<i32>,
    pub months: Vec<String>,
    pub months_num: Vec<u8>,
    pub birds: Vec<(models::Bird, i32)>,
    pub max_vote: u32,
}

#[derive(Serialize)]
pub struct TeraTemplate {
    pub sel_year: i32,
    pub sel_month: Option<String>,
    pub sel_month_path: Option<String>,
    pub years: Vec<i32>,
    pub months: Vec<String>,
    pub months_num: Vec<u8>,
    pub birds: Vec<(models::Bird, i32)>,
    pub max_vote: u32,
}

#[derive(Serialize)]
pub struct TeraDownVote {
    pub bird: models::Bird,
    pub votes: i32,
    pub referer: String,
    pub month: Option<String>,
    pub year: Option<i32>,
    pub error_message: Option<String>,
}
