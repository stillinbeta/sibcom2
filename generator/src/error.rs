extern crate serde_yaml;
extern crate syn;

#[derive(Debug)]
pub(crate) enum Error {
    SerdeError(serde_yaml::Error),
    SynError(syn::Error),
    // Msg(String),
    IOError(std::io::Error),
}

impl From<syn::Error> for Error {
    fn from(source: syn::Error) -> Self {
        Error::SynError(source)
    }
}

impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Self {
        Error::IOError(source)
    }
}
