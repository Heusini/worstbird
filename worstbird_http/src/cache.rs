use rocket::http::{ContentType, Status};
use rocket::response::{Builder, Responder, Response};
use rocket::Request;
use std::io::Cursor;
use std::result;

pub struct Cache {
    pub bytes: Vec<u8>,
}

impl<'r> Responder<'r, 'static> for &Cache {
    fn respond_to(self, _: &Request<'_>) -> result::Result<Response<'static>, Status> {
        let mut response = Response::new();
        response.set_header(ContentType::CSS);
        let mut builder = Builder::new(response);
        builder.sized_body(self.bytes.len(), Cursor::new(self.bytes.clone()));
        Ok(builder.finalize())
    }
}
