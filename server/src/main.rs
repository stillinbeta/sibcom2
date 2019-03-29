#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate generator;

fn main() {
    routes!("/var/lib/pokemon2");
}
