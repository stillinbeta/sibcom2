extern crate serde_yaml;

#[derive(Debug)]
pub(crate) enum Error {
    SerdeError(serde_yaml::Error),
    Msg(String),
}
