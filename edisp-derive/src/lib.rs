//! A `derive` procedural macro for edisp.

extern crate proc_macro;

use edisp_core::prelude::*;

use syn::{DeriveInput, Enum, Result, Ident};

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

#[proc_macro_derive(Dispatch)]
pub fn dispatch_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Failed to build AST");

    impl_dispatch_macro(&ast)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn impl_dispatch_macro(ast: &DeriveInput) -> Result<TokenStream2> {
    todo!();
}

struct Enum {
    name: Ident,
    generics: Generics,
    variants: Vec<Variant>,
}

struct Variant {
    name: Ident,
    inner_type: Option<FieldsUnnamed>,
}

