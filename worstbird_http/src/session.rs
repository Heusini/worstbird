use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Request};

#[derive(Debug)]
pub struct MonthCookie(pub usize);

impl<'a, 'r> FromRequest<'a, 'r> for YearCookie {
    type Error = std::convert::Infallible;
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        request
            .cookies()
            .get("year")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(YearCookie)
            .or_forward(())
    }
}

#[derive(Debug)]
pub struct YearCookie(pub usize);

impl<'a, 'r> FromRequest<'a, 'r> for MonthCookie {
    type Error = std::convert::Infallible;
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        request
            .cookies()
            .get("month")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(MonthCookie)
            .or_forward(())
    }
}
