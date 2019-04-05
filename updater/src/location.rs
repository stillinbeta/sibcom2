extern crate itertools;
extern crate locationsharing;
extern crate mapquest;
extern crate serde;
extern crate serde_json;
extern crate slog;

use serde::{Deserialize, Serialize};

pub struct Location<'a> {
    log: &'a slog::Logger,
    cookie: &'a str,

    mapquest_token: &'a str,
}

impl<'a> Location<'a> {
    pub fn new(log: &'a slog::Logger, cookie: &'a str, mapquest_token: &'a str) -> Self {
        Self {
            log,
            cookie,
            mapquest_token,
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

        let client = mapquest::geocode::Client::new(self.mapquest_token);
        let addresses = client.reverse_geocode(loc.latitude as f32, loc.longitude as f32)?;
        match addresses.results.first().and_then(|r| r.locations.first()) {
            Some(address) => {
                debug!(self.log, "retrieved"; "result" => ?address);

                let parts: Vec<&String> = vec![
                    &address.admin_area_5,
                    &address.admin_area_3,
                    &address.admin_area_1,
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
                Err("no location found".into())
            }
        }
    }
}
