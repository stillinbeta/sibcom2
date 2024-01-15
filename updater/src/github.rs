use anyhow::{anyhow, Result};
use graphql_client::GraphQLQuery;
use serde::{Deserialize, Serialize};

pub struct Github<'a> {
    log: &'a slog::Logger,
}

impl<'a> Github<'a> {
    // const PUBLIC_EVENTS_URL: &'static str =
    //     "https://api.github.com/users/stillinbeta/events/public";
    // const EVENT_NAME: &'static str = "PushEvent";

    pub fn new(log: &'a slog::Logger) -> Self {
        Self { log }
    }
}

#[allow(clippy::upper_case_acronyms)]
type URI = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./src/github-schema.graphql",
    query_path = "./src/github-query.graphql"
)]
pub struct GithubQuery;

#[derive(Serialize, Deserialize, Clone)]
pub struct Repository {
    repository: String,
    url: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Commit {
    message: String,
    url: String,
    repository: Repository,
}

impl<'a> crate::Updater for Github<'a> {
    fn name(&self) -> &'static str {
        "github"
    }

    fn new_value(&mut self) -> Result<String> {
        let client = reqwest::blocking::Client::builder()
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .build()?;

        let body = GithubQuery::build_query(github_query::Variables);
        let resp: github_query::ResponseData = dbg!(client
            .post("https://api.github.com/graphql")
            .json(&body)
            .send()?)
            .json()?;

        let node = resp.viewer.commit_comments.nodes.and_then(|mut v| v.pop()).flatten().ok_or(anyhow!("no commits on github"))?;

        let commit = node.commit.ok_or(anyhow!("no commits on github"))?;

        let commit = Commit {
            message: commit.message,
            url: commit.url,
            repository: Repository { repository: node.repository.name_with_owner, url: node.repository.url }
        };

        Ok(serde_json::to_string(&commit)?)
    }
}
