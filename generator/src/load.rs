extern crate serde_yaml;

use crate::error::Error;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Site {
    pub pages: HashMap<String, HashMap<String, serde_yaml::Value>>,
}

pub(crate) fn parse_site(site: &str) -> Result<Site, Error> {
    serde_yaml::from_str(site).map_err(Error::SerdeError)
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
}
