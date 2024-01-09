use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn routes(input: TokenStream) -> Result<TokenStream> {
    let site_file = crate::load::load_file(input)?;
    let site = crate::load::parse_site(&site_file)?;
    let nav = site.nav();
    let routes = site.pages.iter().map(|(k, v)| {
        let title = k.clone().split_off(1);
        quote!(
                rocket::Route::new(rocket::http::Method::Get, #k, bmon::BMONHandler::new(#v, #nav, #title)
        ))
    });
    Ok(quote!(vec![
        #(#routes,)*
    ]))
}
