// my_macro_lib/src/lib.rs

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemFn};

#[proc_macro_derive(Printable)]
pub fn printable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let gen = quote! {
        impl Printable for #name {
            fn print(&self) {
                println!("Printing {:?}", self);
            }
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn log(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let block = &input.block;

    let gen = quote! {
        fn #name() {
            println!("Entering function {}", stringify!(#name));
            #block
            println!("Exiting function {}", stringify!(#name));
        }
    };
    gen.into()
}
