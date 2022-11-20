extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate slog;

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::Error;

pub struct Github<'a> {
    log: &'a slog::Logger,
}

impl<'a> Github<'a> {
    const PUBLIC_EVENTS_URL: &'static str =
        "https://api.github.com/users/stillinbeta/events/public";
    const EVENT_NAME: &'static str = "PushEvent";

    pub fn new(log: &'a slog::Logger) -> Self {
        Self { log }
    }
}

impl<'a> crate::Updater for Github<'a> {
    fn name(&self) -> &'static str {
        "github"
    }

    fn new_value(&mut self) -> Result<String, crate::Error> {
        let client = reqwest::blocking::Client::builder()
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .build()?;

        let response = client
            .get(Self::PUBLIC_EVENTS_URL)
            .header("accept", "application/json")
            .send()?;

        if response.status() != StatusCode::OK {
            let code = response.status();
            error!(self.log, "bad status"; "code" => ?code, "response" => ?response.bytes());
            return Err(Error::OtherError("failed to github".to_string()));
        }

        let mut json: Vec<Event> = response.error_for_status()?.json()?;

        json.reverse();

        let mut responses: Vec<Event> = json
            .into_iter()
            .filter(|e| e.event_type == Self::EVENT_NAME)
            .collect();

        let mut event = responses.pop().ok_or("No events found")?;
        let commit = event.payload.commits.pop().ok_or("No commits found")?;

        debug!(self.log, "got event"; "event" => ?event, "commit" => ?commit);
        Ok(serde_json::to_string(&Node {
            commit,
            repository: event.repo,
        })?)
    }
}

#[derive(Debug, Deserialize)]
struct Event {
    #[serde(rename = "type")]
    event_type: String,
    repo: Repository,
    payload: Payload,
}

#[derive(Debug, Deserialize)]
struct Payload {
    #[serde(default)]
    commits: Vec<Commit>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Node {
    pub commit: Commit,
    pub repository: Repository,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Commit {
    pub message: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Repository {
    pub url: String,
    pub name: String,
}
