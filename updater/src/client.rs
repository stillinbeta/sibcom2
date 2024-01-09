use anyhow::Result;
use crate::cohost::Chost;
use crate::github::Node;
use crate::mastodon::Status;
use crate::blog::Post;
use redis::Commands;

#[derive(Clone, Debug)]
pub struct Client {
    redis: redis::Client,
    namespace: String,
}

impl Client {
    pub fn new(redis_url: &str, namespace: String) -> Self {
        Self {
            redis: redis::Client::open(redis_url).expect("failed to connect"),
            namespace,
        }
    }

    fn get<T>(&self, name: &'static str) -> Result<T>
    where
        T: serde::de::DeserializeOwned + Clone,
    {
        let key: String = format!("{}::{}", self.namespace, name);
        let response: String = self.redis.get_connection()?.get(key)?;
        let status: T = serde_json::from_str(&response)?;
        Ok(status.clone())
    }

    pub fn get_mastodon(&self) -> Result<Status> {
        self.get("mastodon")
    }

    pub fn get_commit(&self) -> Result<Node> {
        self.get("github")
    }

    pub fn get_chost(&self) -> Result<Chost> {
        self.get("cohost")
    }

    pub fn get_blog(&self) -> Result<Post> {
        self.get("blog")
    }
}
