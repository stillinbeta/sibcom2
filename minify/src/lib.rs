extern crate minifier;
extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;

use quote::quote;
use std::fs::File;
use std::io::Read;

fn mini_css_impl(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let css_file = load_file(input);
    let minified = minifier::css::minify(&css_file).expect("failed to minify");
    quote! { #minified }
}

fn mini_js_impl(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let js_file = load_file(input);
    let minified = minifier::js::minify(&js_file);
    quote! { #minified }
}

#[proc_macro]
pub fn minify_css(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    mini_css_impl(input.into()).into()
}

#[proc_macro]
pub fn minify_js(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    mini_js_impl(input.into()).into()
}

fn load_file(ts: proc_macro2::TokenStream) -> String {
    let lit: syn::LitStr = syn::parse2(ts).expect("expected string literal");
    let mut file = File::open(lit.value()).unwrap_or_else(|_| panic!(
        "failed to open file {:?} in {:?}",
        lit.value(),
        std::env::current_dir(),
    ));
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("failed to read file");
    buf
}
