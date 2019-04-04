extern crate bingmaps;
extern crate itertools;
extern crate locationsharing;
extern crate slog;

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
                    &address.address.locality,
                    &address.address.admin_district1,
                    &address.address.country,
                ]
                .into_iter()
                .flat_map(|x| x)
                .collect();
                let location: String = itertools::join(parts, ", ");
                debug!(self.log, "success"; "location" => &location);
                Ok(location)
            }
            None => {
                warn!(self.log, "no results"; "coordinates" => ?(loc.latitude, loc.longitude));
                Err("no results".into())
            }
        }
    }
}
