extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate slog;

use serde::{Deserialize, Serialize};

pub struct Github<'a> {
    log: &'a slog::Logger,
    github_api_token: &'a str,
}

const GRAPH_API_URL: &str = "https://api.github.com/graphql";

#[derive(Serialize)]
struct Query {
    query: &'static str,
}

const GRAPHQL_QUERY: Query = Query {
    query: include_str!("commits.graphql"),
};

impl<'a> Github<'a> {
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
        let client = reqwest::Client::new();
        let response: GraphResponse = client
            .post(GRAPH_API_URL)
            .json(&GRAPHQL_QUERY)
            .header("accept", "application/json")
            .header("authorization", format!("bearer {}", self.github_api_token))
            .send()?
            .error_for_status()?
            .json()?;

        let nodes = response.data.viewer.comments.nodes;

        debug!(self.log, "Got result"; "node" => ?nodes);

        let node = nodes.first().ok_or("No commits found")?;

        Ok(serde_json::to_string(&node)?)
    }
}

#[derive(Debug, Deserialize)]
struct GraphResponse {
    data: Data,
}

#[derive(Debug, Deserialize)]
struct Data {
    viewer: Viewer,
}

#[derive(Debug, Deserialize)]
struct Viewer {
    #[serde(rename = "commitComments")]
    comments: Comments,
}

#[derive(Debug, Deserialize)]
struct Comments {
    nodes: Vec<Node>,
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
    #[serde(rename = "nameWithOwner")]
    pub name: String,
}
