extern crate bmon;
extern crate generator;

use rocket::{get, launch, routes, Config};

#[get("/")]
fn root() -> rocket::response::Redirect {
    rocket::response::Redirect::to("/hello")
}

#[launch]
fn setup() -> _ {
    let cfg = Config {
        port: 8000,
        address: [0, 0, 0, 0].into(),
        log_level: rocket::log::LogLevel::Normal,
        cli_colors: false,
        ..Config::release_default()
    };

    rocket::build()
        .configure(cfg)
        .mount("/", generator::yaml_routes!("site.yaml"))
        .mount("/", routes!(root))
}
