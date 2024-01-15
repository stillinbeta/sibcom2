use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use slog::debug;

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

impl Github<'_> {
    fn github_latest(&self) -> Result<Node> {
        let client = reqwest::blocking::Client::builder()
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .build()?;

        let events: Vec<Event> = client
            .get(Self::PUBLIC_EVENTS_URL)
            .header("accept", "application/json")
            .send()?
            .error_for_status()?
            .json()?;

        let Event { repo, payload, .. } = events
            .into_iter()
            .rev()
            .find(|e| e.event_type == Self::EVENT_NAME)
            .ok_or(anyhow!("somehow no events found on github"))?;

        let commit = payload
            .commits
            .into_iter()
            .next()
            .ok_or(anyhow!("No commits found"))?;

        debug!(self.log, "got push event"; "repo" => ?repo, "commit" => ?commit);

        Ok(Node {
            commit,
            repository: repo,
        })
    }
}

impl<'a> crate::Updater for Github<'a> {
    fn name(&self) -> &'static str {
        "github"
    }

    fn new_value(&mut self) -> Result<String> {
        let node = self.github_latest()?;

        Ok(serde_json::to_string(&node)?)
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
