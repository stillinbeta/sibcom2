extern crate envy;
extern crate redis;
extern crate serde;
extern crate slog;
extern crate slog_term;
use redis::Commands;
use serde::Deserialize;
use slog::{crit, error, o, Drain};
use std::convert::AsRef;

fn default_namespace() -> String {
    "sibcom2".into()
}

#[derive(Deserialize, Debug)]
struct Config {
    redis_url: String,

    #[serde(default = "default_namespace")]
    namespace: String,
}

#[derive(Debug)]
enum Error {
    OtherError(String),
}

trait Updater {
    fn name(&self) -> &'static str;

    fn new_value(&mut self) -> Result<String, Error>;
}

fn main() {
    let cfg: Config = envy::from_env().expect("Missing configuration");

    let client = redis::Client::open(cfg.redis_url.as_ref()).expect("Failed to connect to Redis");
    let conn = client.get_connection().expect("Failed to connect to Redis");

    let plain = slog_term::PlainSyncDecorator::new(std::io::stdout());
    let root = slog::Logger::root(slog_term::FullFormat::new(plain).build().fuse(), o!());

    let updaters: Vec<Box<dyn Updater>> = vec![];

    for mut updater in updaters {
        match updater.new_value() {
            Err(err) => {
                error!(root, "updater error"; "updater" => updater.name(), "err"=> ?err)
            }
            Ok(val) => conn.set("{}::{}", val).unwrap_or_else(|e| {
                    crit!(root, "redis_error"; "url" => &cfg.redis_url, "updater" => updater.name(), "err" => e.to_string());
                    panic!(e)
                }),
        }
    }
}
