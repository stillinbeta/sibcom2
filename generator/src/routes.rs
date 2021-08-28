extern crate proc_macro2;
extern crate quote;

use crate::error::Error;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn routes(input: TokenStream) -> Result<TokenStream, Error> {
    let site_file = crate::load::load_file(input)?;
    let site = crate::load::parse_site(&site_file)?;
    let nav = site.nav();
    let routes = site.pages.iter().map(|(k, v)| {
        let title = k.clone().split_off(1);
        quote!(
            bmon::Page{
                path: #k,
                root: #v,
                nav: #nav,
                title: #title,
            }
        )
    });
    Ok(quote!(vec![
        #(#routes,)*
    ]))
}
