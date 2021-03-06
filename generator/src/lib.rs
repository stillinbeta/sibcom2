extern crate proc_macro;

pub(crate) mod error;
pub(crate) mod load;
mod routes;
pub(crate) mod serialize;

use proc_macro::TokenStream;

#[proc_macro]
pub fn yaml_routes(input: TokenStream) -> TokenStream {
    routes::routes(input.into())
        .expect("couldn't generate routes")
        .into()
}
