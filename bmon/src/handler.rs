use rocket::handler::{Handler, Outcome};
use rocket::http::{Accept, ContentType, Status};
use rocket::response::Redirect;
use rocket::{Data, Request, Response};
use rocket_contrib::json::Json;
use std::io::Cursor;

use crate::html;
use crate::Value;

#[derive(Clone, Debug)]
pub enum PageValue {
    Body(Value),
    Redirect(String),
}

#[derive(Clone, Debug)]
pub struct BMONHandler {
    body: PageValue,
    title: &'static str, // Title needs to live for lifetime of application
}

impl BMONHandler {
    pub fn new(body: Value, nav: Value, title: &'static str) -> Self {
        match body {
            Value::Link(s) | Value::RelativeLink(s) => Self {
                body: PageValue::Redirect(s),
                title: title,
            },
            _ => Self {
                body: PageValue::Body(Value::Object(vec![
                    (Value::String("nav".into()), nav),
                    (Value::String(title.into()), body),
                ])),
                title: title,
            },
        }
    }
}

impl Handler for BMONHandler {
    fn handle<'r>(&self, req: &'r Request, _data: Data) -> Outcome<'r> {
        match &self.body {
            PageValue::Redirect(r) => Outcome::from(req, Redirect::to(r.clone())), // TODO can we eliminate this clone?
            PageValue::Body(body) => match req.accept() {
                Some(v) if *v == Accept::JSON => Outcome::from(req, Json(body)),
                _ => {
                    let mut response = Response::new();
                    response.set_status(Status::Ok);
                    response.set_header(ContentType::HTML);
                    response.set_sized_body(Cursor::new(html::render_page(self.title, body)));
                    Outcome::from(req, response)
                }
            },
        }
    }
}
