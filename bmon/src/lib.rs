extern crate rocket;

use rocket::handler::{self, Handler, Outcome};
use rocket::{Data, Request};

#[derive(Debug, PartialEq, Clone)]
pub enum Value<'a> {
    String(&'a str),
    Link(&'a str), // Will be hotlinked if rendered as HTML
    Sequence(&'a [Value<'a>]),
    Object(&'a [(Value<'a>, Value<'a>)]), // Easier to make literals of
    Number(i64),
    Boolean(bool),
    Null,
}

#[derive(Clone, Debug)]
pub struct BMONHandler(pub Value<'static>);

impl Handler for BMONHandler {
    fn handle<'r>(&self, req: &'r Request, _data: Data) -> Outcome<'r> {
        Outcome::from(req, format!("{:#?}", self.0))
    }
}
