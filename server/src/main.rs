#![feature(proc_macro_hygiene, decl_macro)]

extern crate bmon;
extern crate generator;

#[macro_use]
extern crate rocket;

#[get("/")]
fn root() -> rocket::response::Redirect {
    rocket::response::Redirect::to("/hello")
}

fn main() {
    rocket::ignite()
        .mount("/", generator::yaml_routes!("site.yaml"))
        .mount("/", routes!(root))
        .launch();
}
