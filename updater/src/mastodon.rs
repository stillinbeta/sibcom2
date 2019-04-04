extern crate dissolve;
extern crate itertools;
extern crate mammut;
extern crate serde;

use serde::Serialize;

pub struct Mastodon<'a> {
    log: &'a slog::Logger,
    mastodon: mammut::Mastodon,
}

const MASTODON_URL: &str = "https://gayhorse.club";
const USER_ID: &str = "2";

impl<'a> Mastodon<'a> {
    pub fn new(log: &'a slog::Logger) -> Self {
        Self {
            log,
            mastodon: mammut::Mastodon::from_data(mammut::Data {
                base: MASTODON_URL.into(),
                // Our method doesn't require authentication
                client_id: "".into(),
                client_secret: "".into(),
                redirect: "".into(),
                token: "".into(),
            }),
        }
    }
}

#[derive(Serialize)]
pub struct Status {
    message: String,
    url: String,
}

impl<'a> crate::Updater for Mastodon<'a> {
    fn name(&self) -> &'static str {
        "mastodon"
    }

    fn new_value(&mut self) -> Result<String, crate::Error> {
        let request = mammut::StatusesRequest::new().limit(1).exclude_replies();
        let responses = self.mastodon.statuses(USER_ID, request)?;

        let status = responses
            .items_iter()
            .next()
            .ok_or::<crate::Error>("No statuses".into())?;

        let message = itertools::join(dissolve::strip_html_tags(&status.content), " ");

        debug!(self.log, "retrieved status"; "message" => message, "status" => ?status);

        Ok(serde_json::to_string(&Status {
            message: status.content,
            url: status.url.unwrap_or(status.uri),
        })?)
    }
}
