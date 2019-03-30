extern crate htmlescape;
extern crate rocket;

use rocket::handler::{Handler, Outcome};
use rocket::http::{Accept, ContentType, Status};
use rocket::{Data, Request, Response};
use std::io::Cursor;

macro_rules! div {
    ($class : expr, $body: expr $(, $args: expr)*) => {
        format!("{}\n{}\n{}",
                format!(r#"<div class="{}">"#, $class),
                format!($body, $($args),*),
                "</div>"
        )
    }
}

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

impl<'a> Value<'a> {
    fn to_json(&self) -> String {
        match self {
            Value::String(s) => format!("{:?}", s),
            Value::Link(s) => format!("{:?}", s),
            Value::Number(n) => format!("{}", n),
            Value::Boolean(b) => format!("{}", b),
            Value::Null => "null".into(),
            Value::Sequence(s) => format!(
                "[{}]",
                s.iter()
                    .map(|v| v.to_json())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            Value::Object(m) => format!(
                "{{{}}}",
                m.iter()
                    .map(|(k, v)| format!("{}:{}", k.to_json(), v.to_json()))
                    .collect::<Vec<String>>()
                    .join(",")
            ),
        }
    }

    fn to_html(&self) -> String {
        match self {
            Value::String(s) => div!("string", r#""{}""#, htmlescape::encode_minimal(s)),
            Value::Link(s) => div!(
                "link",
                r#"<a href="https://{0}">{0}</a>"#,
                htmlescape::encode_minimal(s)
            ),
            Value::Boolean(b) => div!("boolean", "{}", b),
            Value::Number(n) => div!("number", "{}", n),
            Value::Null => div!("null", "null"),
            Value::Sequence(s) => format!(
                "{}\n{}\n{}\n",
                div!("bracket-open", "["),
                div!(
                    "bracket-inner",
                    "{}",
                    s.iter()
                        .map(|s| s.to_html())
                        .collect::<Vec<String>>()
                        .join(",\n")
                ),
                div!("bracket-close", "]")
            ),
            Value::Object(s) => format!(
                "{}\n{}\n{}\n",
                div!("brace-open", "{{"),
                div!(
                    "brace-inner",
                    "{}",
                    s.iter()
                        .map(|(k, v)| format!(
                            "{}:\n{}",
                            div!("key", "{}", k.to_html()),
                            div!("value", "{}", v.to_html())
                        ))
                        .collect::<Vec<String>>()
                        .join(",\n")
                ),
                div!("brace-close", "}}")
            ),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BMONHandler(pub Value<'static>);

impl Handler for BMONHandler {
    fn handle<'r>(&self, req: &'r Request, _data: Data) -> Outcome<'r> {
        let mut response = Response::new();
        response.set_status(Status::Ok);

        match req.accept() {
            Some(v) if *v == Accept::JSON => {
                response.set_header(ContentType::JSON);
                response.set_sized_body(Cursor::new(self.0.to_json()));
            }
            _ => {
                response.set_header(ContentType::HTML);
                response.set_sized_body(Cursor::new(self.0.to_html()));
            }
        };
        Outcome::from(req, response)
    }
}
