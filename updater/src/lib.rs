mod location;
#[macro_use]
extern crate slog;
extern crate bingmaps;
extern crate locationsharing;

pub use location::Location;

#[derive(Debug)]
pub enum Error {
    OtherError(String),
    LocationSharingError(locationsharing::error::Error),
    BingMapsError(bingmaps::Error),
}

impl From<locationsharing::error::Error> for Error {
    fn from(source: locationsharing::error::Error) -> Self {
        Error::LocationSharingError(source)
    }
}

impl From<&str> for Error {
    fn from(source: &str) -> Self {
        Error::OtherError(source.into())
    }
}

impl From<String> for Error {
    fn from(source: String) -> Self {
        Error::OtherError(source)
    }
}

impl From<bingmaps::Error> for Error {
    fn from(source: bingmaps::Error) -> Self {
        Error::BingMapsError(source)
    }
}

pub trait Updater {
    fn name(&self) -> &'static str;

    fn new_value(&mut self) -> Result<String, Error>;
}
