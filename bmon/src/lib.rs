extern crate htmlescape;
extern crate rocket;

use rocket::handler::{Handler, Outcome};
use rocket::http::{Accept, ContentType, Status};
use rocket::{Data, Request, Response};
use std::io::Cursor;

macro_rules! container {
    ($tag: expr, $class : expr, $body: expr $(, $args: expr)*) => {
        format!("{}{}{}",
                format!(r#"<{} class="{}">"#, $tag, $class),
                format!($body, $($args),*),
                format!("</{}>", $tag)
        )
    }

}

macro_rules! div {
    ($class : expr, $body: expr $(, $args: expr)*) => {
        container!("div", $class, $body $(, $args)*)
    };
}

macro_rules! span {
    ($class : expr, $body: expr $(, $args: expr)*) => {
        container!("span", $class, $body $(, $args)*)
    };
}

macro_rules! html_page {
    ($body: expr) => {
        format!(
            r#"<!DOCTYPE html>
<html>
<head>
<meta charset="UTF-8">
<title>title</title>
<link rel="stylesheet" href="//fonts.googleapis.com/css?family=Inconsolata" type="text/css">
<style type="text/css">
{}
</style>
</head>
<body>
{}
</body>
</html>
"#,
            include_str!("../../assets/style.css"),
            $body
        )
    };
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
            Value::String(s) => format!(
                r#""{}""#,
                span!("string", "{}", htmlescape::encode_minimal(s))
            ),
            Value::Link(s) => span!(
                "link",
                r#""<a href="https://{0}">{0}</a>""#,
                htmlescape::encode_minimal(s)
            ),
            Value::Boolean(b) => span!("boolean", "{}", b),
            Value::Number(n) => span!("number", "{}", n),
            Value::Null => span!("null", "null"),
            Value::Sequence(s) => format!(
                "{}{}{}\n",
                span!("bracket-open", "["),
                div!("bracket-inner", "{}", make_rows(s, |v| v.to_html())),
                span!("bracket-close", "]")
            ),
            Value::Object(s) => format!(
                "{}{}{}",
                span!("brace-open", "{{"),
                div!(
                    "brace-inner",
                    "{}",
                    make_rows(s, |(k, v)| {
                        format!(
                            "{}:\n{}",
                            span!("key", "{}", k.to_html()),
                            span!("value", "{}", v.to_html())
                        )
                    })
                ),
                span!("brace-close", "}}")
            ),
        }
    }
}

/// Make rows makes a <div class="row"></div>, but with the commas inside the div
/// a .join() would put them outside the main
fn make_rows<'a, T, F>(rows: &'a [T], f: F) -> String
where
    F: Fn(&T) -> String,
{
    let mut iter = rows.iter().peekable();
    let mut buf = String::new();
    // Can't use a for loop because that'd take ownership of the iter
    while iter.peek().is_some() {
        let val = iter.next().unwrap();
        let mut row = f(val);
        if iter.peek().is_some() {
            row.push(',')
        }
        buf.push_str(&div!("row", "{}", row));
    }
    buf
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
                response.set_sized_body(Cursor::new(html_page!(self.0.to_html())));
            }
        };
        Outcome::from(req, response)
    }
}
