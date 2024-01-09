use anyhow::Result;
use rss::{Channel, Item};
use scraper::Node;
use serde::{Deserialize, Serialize};
use slog::debug;
use std::ops::Deref;

pub struct Mastodon<'a> {
    log: &'a slog::Logger,
}

impl<'a> Mastodon<'a> {
    const USER_RSS_FEED: &'static str = "https://gayhorse.club/@beta.rss";

    pub fn new(log: &'a slog::Logger) -> Self {
        Self { log }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Status {
    pub message: String,
    pub url: String,
}

impl<'a> crate::Updater for Mastodon<'a> {
    fn name(&self) -> &'static str {
        "mastodon"
    }

    fn new_value(&mut self) -> Result<String> {
        let client = reqwest::blocking::Client::new();
        let feed = client
            .get(Self::USER_RSS_FEED)
            .send()?
            .error_for_status()?
            .bytes()?;

        let channel = Channel::read_from(&feed[..])?;
        let Item {
            description, link, ..
        } = channel.items.first().unwrap();

        let message = description.clone().unwrap_or("this one".into());

        let message = scraper::Html::parse_fragment(&message)
            .tree
            .into_iter()
            .filter_map(|v| match v {
                Node::Text(t) => Some(t),
                _ => None,
            })
            .fold(String::new(), |m, v| m + v.deref());

        let url = link.clone().unwrap();

        debug!(self.log, "retrieved toot"; "title" => &message, "url" => &url);

        Ok(serde_json::to_string(&Status { message, url })?)
    }
}
