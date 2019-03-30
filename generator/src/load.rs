extern crate proc_macro2;
extern crate quote;
extern crate serde_yaml;
extern crate syn;

use crate::error::Error;
use proc_macro2::TokenStream;
use quote::quote;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Site {
    pub pages: HashMap<String, HashMap<String, serde_yaml::Value>>,
}

pub(crate) fn parse_site(site: &str) -> Result<Site, Error> {
    serde_yaml::from_str(site).map_err(Error::SerdeError)
}

pub(crate) fn load_file(ts: TokenStream) -> Result<String, Error> {
    let lit: syn::LitStr = syn::parse2(ts)?;
    let mut file = File::open(dbg!(lit.value()))?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

#[cfg(test)]
mod test {
    extern crate maplit;

    use super::*;
    use maplit::hashmap;

    const EXAMPLE_SITE: &str = include_str!("testdata/site.yaml");

    #[test]
    fn parse() {
        let expected = Site {
            pages: hashmap!(
                "/".into() => hashmap!(
                    "name".into() => "Twilight Sparkle".into(),
                    "occupation".into() => "Princess of Friendship".into(),
                ),
                "/friends".into() => hashmap!(
                    "ponyville".into() => serde_yaml::Value::Sequence(vec!(
                        "Applejack".into(),
                        "Fluttershy".into(),
                        "Pinkie Pie".into(),
                        "Rainbow Dash".into(),
                        "Rarity".into(),
                        )
                    )
                )
            ),
        };

        let site = parse_site(EXAMPLE_SITE).expect("failed to parse");
        assert_eq!(expected, site)
    }

    #[test]
    fn load() {
        assert_eq!(
            EXAMPLE_SITE,
            load_file(quote!("src/testdata/site.yaml")).expect("failed to load file")
        )
    }
}
