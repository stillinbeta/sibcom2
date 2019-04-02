extern crate bmon;
extern crate proc_macro2;
extern crate quote;
extern crate serde_yaml;
extern crate syn;

use crate::error::Error;
use proc_macro2::TokenStream;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize, PartialEq)]
struct SiteIntermediate {
    #[serde(with = "tuple_vec_map")]
    pages: Vec<(String, serde_yaml::Value)>,
}

#[derive(Debug, PartialEq)]
pub struct Site {
    pub pages: Vec<(String, bmon::Value)>,
}

impl From<SiteIntermediate> for Site {
    fn from(source: SiteIntermediate) -> Self {
        Site {
            pages: source
                .pages
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

impl Site {
    pub fn with_nav(&self) -> HashMap<String, bmon::Value> {
        let nav = bmon::Value::Sequence(
            self.pages
                .iter()
                .map(|(k, _v)| bmon::Value::String(k.clone()))
                .collect(),
        );
        self.pages
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    bmon::Value::Object(vec![
                        (bmon::Value::String("nav".into()), nav.clone()),
                        (bmon::Value::String(k.clone().split_off(1)), v.clone()),
                    ]),
                )
            })
            .collect()
    }
}

pub(crate) fn parse_site(site: &str) -> Result<Site, Error> {
    let v: SiteIntermediate = serde_yaml::from_str(site)?;
    Ok(v.into())
}

pub(crate) fn load_file(ts: TokenStream) -> Result<String, Error> {
    let lit: syn::LitStr = syn::parse2(ts)?;
    let mut file = File::open(lit.value())?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

#[cfg(test)]
mod test {
    use super::*;
    use quote::quote;

    const EXAMPLE_SITE: &str = include_str!("testdata/site.yaml");

    #[test]
    fn parse() {
        let expected = Site {
            pages: vec![
                (
                    "/".into(),
                    bmon::Value::Object(vec![
                        ("name".into(), "Twilight Sparkle".into()),
                        ("occupation".into(), "Princess of Friendship".into()),
                    ]),
                ),
                (
                    "/friends".into(),
                    bmon::Value::Object(vec![(
                        "ponyville".into(),
                        bmon::Value::Sequence(vec![
                            "Applejack".into(),
                            "Fluttershy".into(),
                            "Pinkie Pie".into(),
                            "Rainbow Dash".into(),
                            "Rarity".into(),
                        ]),
                    )]),
                ),
            ],
        };

        let site = parse_site(EXAMPLE_SITE).expect("failed to parse");
        assert_eq!(expected, site)
    }

    #[test]
    fn load() {
        let loaded = load_file(quote!("src/testdata/site.yaml")).expect("failed to load file");

        assert_eq!(EXAMPLE_SITE, loaded)
    }
}
