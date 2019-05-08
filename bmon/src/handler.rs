extern crate updater;

use rocket::handler::{Handler, Outcome};
use rocket::http::{Accept, ContentType, Status};
use rocket::response::Redirect;
use rocket::{Data, Request, Response};
use rocket_contrib::json::Json;
use std::io::Cursor;
use std::str::FromStr;

use crate::html;
use crate::Value;

#[derive(Clone, Debug)]
pub enum PageValue {
    Body(Value),
    Redirect(String),
    Homepage(Value, Value, updater::Client),
}

#[derive(Clone, Debug)]
pub struct BMONHandler {
    body: PageValue,
    title: &'static str, // Title needs to live for lifetime of application
}
impl BMONHandler {
    pub fn new(body: Value, nav: Value, title: &'static str) -> Self {
        match body {
            Value::Link(_, s) => Self {
                body: PageValue::Redirect(s),
                title: title,
            },
            _ if title == "hello" => {
                let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL unset");
                let redis_namespace = std::env::var("REDIS_NAMESPACE").unwrap_or("sibcom2".into());
                Self {
                    body: PageValue::Homepage(
                        body,
                        nav,
                        updater::Client::new(&redis_url, redis_namespace),
                    ),
                    title: title,
                }
            }
            _ => Self {
                body: PageValue::Body(Self::make_page_with_nav(body, nav, title.into())),
                title: title,
            },
        }
    }

    fn make_page_with_nav(body: Value, nav: Value, title: String) -> Value {
        Value::Object(vec![
            (Value::String("nav".into()), nav),
            (Value::String(title.into()), body),
        ])
    }

    fn send_value<'r>(&self, req: &'r Request, value: &Value) -> Outcome<'r> {
        match req.accept() {
            Some(v) if *v == Accept::JSON => Outcome::from(req, Json(value)),
            _ => {
                let mut response = Response::new();
                let theme = req
                    .cookies()
                    .get("theme")
                    .and_then(|t| FromStr::from_str(t.value()).ok())
                    .unwrap_or(html::Theme::SolarizedDark);
                response.set_status(Status::Ok);
                response.set_header(ContentType::HTML);
                response.set_sized_body(Cursor::new(html::render_page(self.title, theme, value)));
                Outcome::from(req, response)
            }
        }
    }

    fn get_latest(&self, client: &updater::Client) -> Value {
        let mastodon = match client.get_mastodon() {
            Ok(status) => Value::Link(status.url, status.message),
            // TODO: slog
            Err(err) => {
                eprintln!("Mastodon error: {:?}", err);
                Value::String("unknown".into())
            }
        };
        let github = match client.get_commit() {
            Ok(commit) => Value::Object(vec![
                (
                    Value::String("commit".into()),
                    Value::Link(commit.commit.url, commit.commit.message),
                ),
                (
                    Value::String("repository".into()),
                    Value::Link(commit.repository.url, commit.repository.name),
                ),
            ]),
            Err(err) => {
                eprintln!("Github error: {:?}", err);
                Value::String("unknown".into())
            }
        };

        let location = match client.get_location() {
            Ok(location) => Value::String(location.position),
            Err(err) => {
                eprintln!("Github error: {:?}", err);
                Value::String("unknown".into())
            }
        };

        Value::Object(vec![
            (Value::String("toot".into()), mastodon),
            (Value::String("location".into()), location),
            (Value::String("push".into()), github),
        ])
    }
}

impl Handler for BMONHandler {
    fn handle<'r>(&self, req: &'r Request, _data: Data) -> Outcome<'r> {
        match &self.body {
            PageValue::Redirect(r) => Outcome::from(req, Redirect::to(r.clone())), // TODO can we eliminate this clone?
            PageValue::Body(body) => self.send_value(req, body),
            PageValue::Homepage(body, nav, client) => match body {
                Value::Object(map) => {
                    let mut body = map.clone();
                    body.push((Value::String("latest".into()), self.get_latest(client)));

                    self.send_value(
                        req,
                        &Self::make_page_with_nav(
                            Value::Object(body),
                            nav.clone(),
                            self.title.into(),
                        ),
                    )
                }
                _ => {
                    eprintln!("Homepage not a string!");
                    self.send_value(req, &body)
                }
            },
        }
    }
}
