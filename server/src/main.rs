extern crate bmon;
extern crate generator;

#[macro_use]
extern crate rocket;

#[get("/")]
fn root() -> rocket::response::Redirect {
    rocket::response::Redirect::to("/hello")
}

#[launch]
fn setup() -> _ {
    rocket::build()
        .mount("/", generator::yaml_routes!("site.yaml"))
        .mount("/", routes!(root))
}
