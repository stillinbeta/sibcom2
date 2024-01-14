use anyhow::{Result, anyhow};
use rss::{Channel, Item};
use serde::{Deserialize, Serialize};
use slog::debug;

pub struct Blog<'a> {
    log: &'a slog::Logger,
}

impl<'a> Blog<'a> {
    const BLOG_RSS: &'static str = "https://blog.stillinbeta.com/rss.xml";

    pub fn new(log: &'a slog::Logger) -> Self {
        Self { log }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Post {
    pub title: String,
    pub url: String,
}

impl<'a> crate::Updater for Blog<'a> {
    fn name(&self) -> &'static str {
        "blog"
    }

    fn new_value(&mut self) -> Result<String> {
        let client = reqwest::blocking::Client::new();
        let feed = client
            .get(Self::BLOG_RSS)
            .send()?
            .error_for_status()?
            .bytes()?;

        let channel = Channel::read_from(&feed[..])?;
        let Item { title, link, .. } = channel.items.first().ok_or(anyhow!("no blog posts found"))?;

        let title = title.clone().ok_or(anyhow!("no title"))?;
        let url = link.clone().ok_or(anyhow!("no url"))?;

        debug!(self.log, "retrieved blog"; "title" => &title, "url" => &url);

        Ok(serde_json::to_string(&Post { title, url })?)
    }
}
