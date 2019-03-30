extern crate proc_macro2;
extern crate quote;

use proc_macro2::TokenStream;
use quote::quote;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub(crate) fn routes(input: TokenStream) -> TokenStream {
    quote!({ println!("{:?}", #input) })
}
