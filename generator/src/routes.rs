extern crate proc_macro2;
extern crate quote;

use crate::error::Error;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn routes(input: TokenStream) -> Result<TokenStream, Error> {
    let site_file = crate::load::load_file(input)?;
    let site = crate::load::parse_site(&site_file)?;
    let pages = site.with_nav();
    let routes = pages.iter().map(|(k, v)| {
        let bmon = crate::serialize::value_to_bmon(v);
        let title = k.clone().split_off(1);
        quote!(
                rocket::Route::new(rocket::http::Method::Get, #k, bmon::BMONHandler(#bmon, #title)
        ))
    });
    Ok(quote!(vec![
        #(#routes,)*
    ]))
}
