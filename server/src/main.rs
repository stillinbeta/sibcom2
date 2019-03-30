#![feature(proc_macro_hygiene, decl_macro)]

extern crate bmon;
extern crate generator;
extern crate rocket;

fn main() {
    rocket::ignite()
        .mount("/", generator::yaml_routes!("site.yaml"))
        .launch();
}
