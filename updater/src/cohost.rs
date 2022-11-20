use serde::{Deserialize, Serialize};

use rss::{Channel, Item};

pub struct Cohost<'a> {
    log: &'a slog::Logger,
}

impl<'a> Cohost<'a> {
    const USER_RSS_FEED: &'static str = "https://stillinbeta.cohost.org/rss/public";

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

    fn new_value(&mut self) -> Result<String, crate::Error> {
        let client = reqwest::blocking::Client::new();
        let feed = client
            .get(Self::USER_RSS_FEED)
            .send()?
            .error_for_status()?
            .bytes()?;

        let channel = Channel::read_from(&feed[..])?;
        let Item { title, link, .. } = channel.items.first().unwrap();

        let title = title.clone().unwrap_or("this one".into());
        let url = link.clone().unwrap();

        debug!(self.log, "retrieved chost"; "title" => &title, "url" => &url);

        Ok(serde_json::to_string(&Chost { title, url })?)
    }
}
