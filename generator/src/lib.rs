extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[proc_macro]
pub fn routes(input: TokenStream) -> TokenStream {
    _routes_impl(input.into()).into()
}

fn _routes_impl(input: TokenStream2) -> TokenStream2 {
    quote!({ println!("{:?}", #input) })
}
