use rocket::handler::{Handler, Outcome};
use rocket::http::{Accept, ContentType, Status};
use rocket::{Data, Request, Response};
use rocket_contrib::json::Json;
use std::io::Cursor;

use crate::html;
use crate::Value;

#[derive(Clone, Debug)]
pub struct BMONHandler(pub Value, pub &'static str);

impl Handler for BMONHandler {
    fn handle<'r>(&self, req: &'r Request, _data: Data) -> Outcome<'r> {
        let mut response = Response::new();
        response.set_status(Status::Ok);

        match req.accept() {
            Some(v) if *v == Accept::JSON => Outcome::from(req, Json(&self.0)),
            _ => {
                response.set_header(ContentType::HTML);
                response.set_sized_body(Cursor::new(html::render_page(self.1, &self.0)));
                Outcome::from(req, response)
            }
        }
    }
}
