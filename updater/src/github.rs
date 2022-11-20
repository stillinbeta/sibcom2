extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate slog;

use serde::{Deserialize, Serialize};

pub struct Github<'a> {
    log: &'a slog::Logger,
    github_api_token: &'a str,
}

impl<'a> Github<'a> {
    const PUBLIC_EVENTS_URL: &'static str =
        "https://api.github.com/users/stillinbeta/events/public";
    const EVENT_NAME: &'static str = "PushEvent";

    pub fn new(log: &'a slog::Logger, github_api_token: &'a str) -> Self {
        Self {
            log,
            github_api_token,
        }
    }
}

impl<'a> crate::Updater for Github<'a> {
    fn name(&self) -> &'static str {
        "github"
    }

    fn new_value(&mut self) -> Result<String, crate::Error> {
        let client = reqwest::blocking::Client::new();
        let mut responses: Vec<Event> = client
            .get(Self::PUBLIC_EVENTS_URL)
            .header("accept", "application/json")
            .header("authorization", format!("bearer {}", self.github_api_token))
            .send()?
            .error_for_status()?
            .json()?;

        responses.reverse();

        let mut responses: Vec<Event> = responses
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
