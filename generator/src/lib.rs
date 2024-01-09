extern crate proc_macro;

pub(crate) mod load;
mod routes;

use proc_macro::TokenStream;

#[proc_macro]
pub fn yaml_routes(input: TokenStream) -> TokenStream {
    routes::routes(input.into())
        .expect("couldn't generate routes")
        .into()
}
