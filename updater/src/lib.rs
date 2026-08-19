pub mod blog;
mod client;
pub mod github;
pub mod sourcehut;

pub use blog::Blog;
pub use client::Client;
pub use github::Github;
pub use sourcehut::Sourcehut;

use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use reqwest::blocking::Client as ReqwestClient;

pub trait Updater {
    fn name(&self) -> &'static str;

    fn new_value(&mut self) -> Result<String>;
}

pub const USER_AGENT: &str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub fn reqwest_client() -> Result<ReqwestClient> {
    ReqwestClient::builder()
        .user_agent(USER_AGENT)
        .build()
        .map_err(Error::from)
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
