extern crate bingmaps;
extern crate itertools;
extern crate locationsharing;
extern crate serde;
extern crate serde_json;
extern crate slog;

use serde::{Deserialize, Serialize};

pub struct Location<'a> {
    log: &'a slog::Logger,
    cookie: &'a str,

    bing_maps_token: &'a str,
}

impl<'a> Location<'a> {
    pub fn new(log: &'a slog::Logger, cookie: &'a str, bing_maps_token: &'a str) -> Self {
        Self {
            log,
            cookie,
            bing_maps_token,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Position {
    pub position: String,
}

impl<'a> crate::Updater for Location<'a> {
    fn name(&self) -> &'static str {
        "location"
    }

    fn new_value(&mut self) -> Result<String, crate::Error> {
        debug!(self.log, "Starting location update");

        let locs = locationsharing::get_locations(self.cookie)?;
        debug!(self.log, "retrieved"; "results" => ?locs);

        let loc = locs.first().ok_or("No locations returned")?;

        let client = bingmaps::Client::new(self.bing_maps_token);
        let params = bingmaps::locations::FindPoint::from_latlng(loc.latitude, loc.longitude);

        let addresses = bingmaps::locations::Location::find_by_point(&client, params, None)?;

        match addresses.first() {
            Some(address) => {
                debug!(self.log, "retrieved"; "result" => ?address);

                let parts: Vec<&String> = vec![
                    &address.address.admin_district2,
                    &address.address.admin_district1,
                    &address.address.country,
                ]
                .into_iter()
                .flat_map(|x| x)
                .collect();
                let location: String = itertools::join(parts, ", ");
                debug!(self.log, "success"; "location" => &location);

                let position = serde_json::to_string(&Position { position: location })?;
                Ok(position)
            }
            None => {
                warn!(self.log, "no results"; "coordinates" => ?(loc.latitude, loc.longitude));
                Err("no results".into())
            }
        }
    }
}
