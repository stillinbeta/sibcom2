use anyhow::{anyhow, Result};
use graphql_client::{GraphQLQuery, Response};
use reqwest::{
    blocking::Client as ReqwestClient,
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
};
use serde::{Deserialize, Serialize};
use sourcehut_query::{
    SourcehutQueryMeRepositoriesResults as SourcehutRepository,
    SourcehutQueryMeRepositoriesResultsLogResults as SourcehutCommit,
};

type Time = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path="src/sourcehut_schema.graphql",
    query_path="src/sourcehut.graphql",
    response_derives=Debug,
)]
pub struct SourcehutQuery;

pub struct Sourcehut {
    client: ReqwestClient,
}

impl Sourcehut {
    const GRAPHQL_URL: &'static str = "https://git.sr.ht/query";

    pub fn new(bearer: &str) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", bearer))?,
        );

        Ok(Self {
            client: ReqwestClient::builder()
                .user_agent(super::USER_AGENT)
                .default_headers(headers)
                .build()?,
        })
    }

    pub fn get_query(&self) -> Result<sourcehut_query::ResponseData> {
        let body = SourcehutQuery::build_query(sourcehut_query::Variables);
        match dbg!(self
            .client
            .post(Self::GRAPHQL_URL)
            .json(&body)
            .send()?)
            .json()?
        {
            Response {
                errors: Some(errors),
                ..
            } => Err(anyhow!(errors
                .into_iter()
                .map(|err| err.message)
                .fold(String::new(), |c, n| c + "\n" + &n))),
            Response {
                data: Some(data), ..
            } => Ok(data),
            Response { data: None, .. } => Err(anyhow!("no data returned")),
        }
    }

    pub fn get_push(&self) -> Result<Push> {
        self.get_query()?.try_into()
    }
}

fn repo_url(name: &str, repo: &str) -> String {
    format!("https://git.sr.ht/{}/{}/", name, repo)
}

fn commit_url(name: &str, repo: &str, commit: &str) -> String {
    repo_url(name, repo) + "/commit/" + commit
}

impl TryFrom<sourcehut_query::ResponseData> for Push {
    type Error = anyhow::Error;

    fn try_from(value: sourcehut_query::ResponseData) -> Result<Self> {
        let username = value.me.canonical_name;
        let SourcehutRepository {
            name: repo_name,
            log,
            ..
        } = value
            .me
            .repositories
            .results
            .first()
            .ok_or(anyhow!("No repositories found"))?;
        let SourcehutCommit { message, id } = log.results.first().ok_or(anyhow!("No commit!"))?;

        Ok(Push {
            commit: Commit {
                message: message.to_string(),
                url: commit_url(&username, &repo_name, id),
            },
            repository: Repository {
                url: repo_url(&username, &repo_name),
                name: repo_name.to_string(),
            },
        })
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Push {
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
