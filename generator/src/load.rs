use anyhow::Result;
use proc_macro2::TokenStream;
use serde::Deserialize;
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
    pub fn nav(&self) -> bmon::Value {
        bmon::Value::Sequence(
            self.pages
                .iter()
                .map(|(k, _v)| bmon::Value::Link(k.clone(), k.clone()))
                .collect(),
        )
    }
}

pub(crate) fn parse_site(site: &str) -> Result<Site> {
    let v: SiteIntermediate = serde_yaml::from_str(site)?;
    Ok(v.into())
}

pub(crate) fn load_file(ts: TokenStream) -> Result<String> {
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
