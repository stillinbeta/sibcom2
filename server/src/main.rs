#![feature(proc_macro_hygiene, decl_macro)]

extern crate bmon;
extern crate generator;

fn main() {
    println!("{:#?}", generator::yaml_routes!("site.yaml"));
}
