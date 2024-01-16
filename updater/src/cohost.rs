use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use slog::{debug, warn};

pub struct Cohost<'a> {
    log: &'a slog::Logger,
}

#[derive(Deserialize)]
struct Page {
    items: Vec<Item>,
}

#[derive(Deserialize)]
struct Item {
    url: String,
    title: String,
    author: Author,
}

#[derive(Deserialize)]
struct Author {
    name: String,
}

impl<'a> Cohost<'a> {
    const USER_PAGE_JSON: &'static str = "https://cohost.org/stillinbeta/rss/public.json";
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

impl Cohost<'_> {
    fn get_page(&mut self) -> Result<Page> {
        let client = reqwest::blocking::Client::new();

        let resp = client.get(Self::USER_PAGE_JSON).send()?;

        if resp.status().is_success() {
            Ok(resp.json()?)
        } else {
            let code = resp.status();
            let headers = format!("{:?}", resp.headers());
            let body = resp.text()?;

            warn!(self.log, "failed to get result"; "code" => code.as_str(), "body" => body, "headers" => headers);
            Err(anyhow!("couldn't update Cohost"))
        }
    }
}

impl<'a> crate::Updater for Cohost<'a> {
    fn name(&self) -> &'static str {
        "cohost"
    }

    fn new_value(&mut self) -> Result<String> {
        let page = self.get_page()?;

        let Item { title, url, .. } = page
            .items
            .into_iter()
            .find(|v| v.author.name == Self::AUTHOR_NAME)
            .ok_or(anyhow!("couldn't find any chosts"))?;

        debug!(self.log, "retrieved chost"; "title" => &title, "url" => &url);

        Ok(serde_json::to_string(&Chost { title, url })?)
    }
}
