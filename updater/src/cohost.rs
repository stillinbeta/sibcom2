use anyhow::Result;
use rss::{Channel, Item};
use serde::{Deserialize, Serialize};
use slog::debug;

pub struct Cohost<'a> {
    log: &'a slog::Logger,
}

impl<'a> Cohost<'a> {
    const USER_RSS_FEED: &'static str = "https://cohost.org/stillinbeta/rss/public.rss";
    const AUTHOR_NAME: &'static str = "@stillinbeta";

    pub fn new(log: &'a slog::Logger) -> Self {
        Self { log }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Chost {
    pub title: String,
    pub url: String,
}

impl<'a> crate::Updater for Cohost<'a> {
    fn name(&self) -> &'static str {
        "cohost"
    }

    fn new_value(&mut self) -> Result<String> {
        let client = reqwest::blocking::Client::new();
        let feed = client
            .get(Self::USER_RSS_FEED)
            .send()?
            .error_for_status()?
            .bytes()?;

        let channel = Channel::read_from(&feed[..])?;
        let Item { title, link, .. } = channel
            .items
            .into_iter()
            // find one we wrote
            .find(|v| match &v.author {
                Some(v) => v.contains(Self::AUTHOR_NAME),
                None => false,
            })
            .unwrap();

        let title = title.clone().unwrap_or("this one".into());
        let url = link.clone().unwrap();

        debug!(self.log, "retrieved chost"; "title" => &title, "url" => &url);

        Ok(serde_json::to_string(&Chost { title, url })?)
    }
}
