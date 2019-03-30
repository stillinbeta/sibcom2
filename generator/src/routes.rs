extern crate proc_macro2;
extern crate quote;

use crate::error::Error;
use proc_macro2::TokenStream;

pub(crate) fn routes(input: TokenStream) -> Result<TokenStream, Error> {
    let site_file = crate::load::load_file(input)?;
    let site = crate::load::parse_site(&site_file)?;
    Ok(crate::serialize::value_to_bmon(
        site.pages.values().next().unwrap(),
    ))
}
