pub mod blog;
mod client;
pub mod github;
pub mod mastodon;

pub use blog::Blog;
pub use client::Client;
pub use github::Github;
pub use mastodon::Mastodon;

use anyhow::Result;

pub trait Updater {
    fn name(&self) -> &'static str;

    fn new_value(&mut self) -> Result<String>;
}
