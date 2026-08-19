use anyhow::{anyhow, Result};
use reqwest::blocking::Client;
use serde::Deserialize;
use slog::debug;

use crate::{reqwest_client, Commit, Push, Repository};

pub struct Github<'a> {
    log: &'a slog::Logger,
    client: Client,
}

impl<'a> Github<'a> {
    const PUBLIC_EVENTS_URL: &'static str =
        "https://api.github.com/users/stillinbeta/events/public";
    const EVENT_NAME: &'static str = "PushEvent";

    pub fn new(log: &'a slog::Logger) -> Result<Self> {
        Ok(Self {
            log,
            client: Client::builder().user_agent(super::USER_AGENT).build()?,
        })
    }
}

impl Github<'_> {
    fn github_latest(&self) -> Result<Event> {
        let client = reqwest_client()?;

        let events: Vec<Event> = client
            .get(Self::PUBLIC_EVENTS_URL)
            .header("accept", "application/json")
            .send()?
            .error_for_status()?
            .json()?;

        events
            .into_iter()
            .rev()
            .find(|e| e.event_type == Self::EVENT_NAME)
            .ok_or(anyhow!("somehow no events found on github"))
    }

    fn get_push(&self) -> Result<Push> {
        let event = self.github_latest()?;

        debug!(self.log, "retrieving commit message"; "commit" => &event.payload.head);

        let commit: GHCommit = self.client.get(event.url_for_commit()).send()?.json()?;
        Ok(Push {
            repository: event.repo,
            commit: commit.commit,
        })
    }
}

impl<'a> crate::Updater for Github<'a> {
    fn name(&self) -> &'static str {
        "github"
    }

    fn new_value(&mut self) -> Result<String> {
        let push = self.get_push()?;

        Ok(serde_json::to_string(&push)?)
    }
}

#[derive(Debug, Deserialize)]
struct Event {
    #[serde(rename = "type")]
    event_type: String,
    repo: Repository,
    payload: Payload,
}

impl Event {
    fn url_for_commit(&self) -> String {
        format!("{}/commits/{}", self.repo.url, self.payload.head)
    }
}

#[derive(Debug, Deserialize)]
struct Payload {
    #[serde(default)]
    head: String,
}

#[derive(Debug, Deserialize)]
struct GHCommit {
    commit: Commit,
}
