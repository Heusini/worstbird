use crate::error::CustomError;
use chrono::prelude::*;
use chrono::Month;
use dashmap::DashMap;
use diesel::sql_types::Integer;
use num_traits::FromPrimitive;
use rocket::http::{Cookie, CookieJar};
use rocket::State;
use std::net::IpAddr;
use std::net::SocketAddr;

#[derive(QueryableByName)]
pub struct DistinctYear {
    #[sql_type = "Integer"]
    pub year: i32,
}

#[derive(QueryableByName)]
pub struct DistinctMonth {
    #[sql_type = "Integer"]
    pub month: i32,
}

#[derive(Debug)]
pub struct UserVoteCount {
    pub count: u32,
    pub expiration: chrono::DateTime<Local>,
}

pub static MAX_IP_VOTE: u32 = 20;
pub fn set_cookie(key: &str, cookies: &CookieJar, bird_id: u32) -> Result<(), CustomError> {
    let expire_date = get_expire_date().to_rfc2822();
    let cookie_str = format!("{}={}; Expires={}", key, bird_id, expire_date);
    let mut cookie = Cookie::parse(cookie_str).unwrap();
    cookie.set_secure(false);
    cookie.set_path("/downvote");
    cookies.add(cookie);
    Ok(())
}

pub fn month_to_shortmonth(month: u32) -> String {
    format!(
        "{}",
        Month::from_u32(month)
            .unwrap()
            .name()
            .chars()
            .take(3)
            .collect::<String>()
    )
}

pub fn get_expire_date() -> DateTime<Local> {
    let now = Local::now();
    let new_date;

    if now.month() == 12 {
        new_date = Local.ymd(now.year() + 1, 1, 1).and_hms(0, 5, 0);
    } else {
        new_date = Local.ymd(now.year(), now.month() + 1, 1).and_hms(0, 5, 0);
    }

    new_date
}

pub fn get_ip_vote_count(
    state: &State<DashMap<IpAddr, UserVoteCount>>,
    remote_addr: SocketAddr,
) -> u32 {
    if let Some(mut entry) = state.get_mut(&remote_addr.ip()) {
        if entry.expiration < Local::now() {
            entry.count = 1;
            entry.expiration = get_expire_date();
        }
        if entry.count > MAX_IP_VOTE {
            return entry.count;
        } else {
            entry.count += 1;
        }

        return entry.count;
    } else {
        state.insert(
            remote_addr.ip(),
            UserVoteCount {
                count: 1,
                expiration: get_expire_date(),
            },
        );
        1
    }
}
pub fn check_year(year: i32) -> Result<(), CustomError> {
    if year < 0 || year > 3333 {
        Err("This year does not excist in the worstbird timeline".into())
    } else {
        Ok(())
    }
}

pub fn check_month(month: u32) -> Result<(), CustomError> {
    if month < 1 || month > 12 {
        return Err("This month does not exist in this universe".into());
    } else {
        Ok(())
    }
}
