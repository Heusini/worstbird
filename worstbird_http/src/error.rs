use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Response};
use rocket::Request;
use std::io::Cursor;

#[derive(Debug)]
pub enum CustomError {
    DatabaseErr(diesel::result::Error),
    DateError(&'static str),
}

impl From<diesel::result::Error> for CustomError {
    fn from(e: diesel::result::Error) -> Self {
        CustomError::DatabaseErr(e)
    }
}

impl From<&'static str> for CustomError {
    fn from(e: &'static str) -> Self {
        CustomError::DateError(e)
    }
}

impl<'r> Responder<'r, 'static> for CustomError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let body = match self {
            CustomError::DatabaseErr(e) => format!("Diesel error: {:?}", e),
            _ => format!("Another error"),
        };

        let res = Response::build()
            .status(Status::InternalServerError)
            .header(ContentType::HTML)
            .sized_body(body.len(), Cursor::new(body))
            .finalize();
        Ok(res)
    }
}
