extern crate envy;
extern crate redis;
extern crate serde;
#[macro_use]
extern crate slog;
extern crate slog_term;

extern crate updater;

use redis::Commands;
use serde::Deserialize;
use slog::Drain;
use std::convert::AsRef;

use updater::Updater;

fn default_namespace() -> String {
    "sibcom2".into()
}

#[derive(Deserialize, Debug)]
struct Config {
    redis_url: String,

    #[serde(default = "default_namespace")]
    redis_namespace: String,
}

fn main() {
    let cfg: Config = envy::from_env().expect("Missing configuration");

    let client = redis::Client::open(cfg.redis_url.as_ref()).expect("Failed to connect to Redis");
    let mut conn = client.get_connection().expect("Failed to connect to Redis");

    let plain = slog_term::PlainSyncDecorator::new(std::io::stdout());
    let root = slog::Logger::root(slog_term::FullFormat::new(plain).build().fuse(), o!());

    let updaters: Vec<Box<dyn Updater>> = vec![
        Box::new(updater::Blog::new(&root)),
        Box::new(updater::Github::new(&root)),
        Box::new(updater::Mastodon::new(&root)),
    ];

    for mut updater in updaters {
        match updater.new_value() {
            Err(err) => {
                error!(root, "updater error"; "updater" => updater.name(), "err"=> ?err);
            }
            Ok(val) => {
                match conn.set(format!("{}::{}", cfg.redis_namespace, updater.name()), &val) {
                    Err(e) => {
                        crit!(root, "redis_error"; "url" => &cfg.redis_url, "updater" => updater.name(), "err" => e.to_string());
                        panic!("{}", e)
                    }
                    Ok(()) => {
                        info!(root, "updated status"; "updater" => updater.name(), "value" => &val)
                    }
                }
            }
        }
    }
}
