pub mod blog;
mod client;
pub mod github;
pub mod sourcehut;

pub use blog::Blog;
pub use client::Client;
pub use github::Github;
pub use sourcehut::Sourcehut;

use anyhow::{Error, Result};
use reqwest::blocking::Client as ReqwestClient;

pub trait Updater {
    fn name(&self) -> &'static str;

    fn new_value(&mut self) -> Result<String>;
}

pub const USER_AGENT: &'static str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub fn reqwest_client() -> Result<ReqwestClient> {
    ReqwestClient::builder()
        .user_agent(USER_AGENT)
        .build()
        .map_err(Error::from)
}
