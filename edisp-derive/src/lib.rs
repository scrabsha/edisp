//! A `derive` procedural macro for edisp.

extern crate proc_macro;

use syn::{
    Data, DataEnum, DeriveInput, Error, Fields, Generics, Ident, Result, Variant as SVariant,
};

use quote::{format_ident, quote};

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

/// Allows to automatically derive the `Dispatch` trait for an enum.
///
/// This derive fails if either:
///   - the type on which `Dispatch` is derived is a struct or an union,
///   - one of the enum variants contains an anonymous structure.
///
/// Note that a variant has no associated data, then empty tuples (`()`) will
/// be added to the corresponding container each time this variant is met.
#[proc_macro_derive(Dispatch)]
pub fn dispatch_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Failed to build AST");

    impl_dispatch_macro(ast)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn impl_dispatch_macro(ast: DeriveInput) -> Result<TokenStream2> {
    let e = Enum::from_syn(ast)?;
    let name = &e.name;

    let struct_generics = e.generics.params.iter();

    let full_type = quote! {
        #name< #( #struct_generics, )*>
    };
    let full_type2 = full_type.clone();

    let struct_generics = e.generics.params.iter();
    let container_type_letters = e.variants.iter().map(|v| &v.container_type_name);
    let container_type_letters2 = container_type_letters.clone();
    let container_type_letters3 = container_type_letters.clone();
    let container_type_letters4 = container_type_letters.clone();

    let trait_generics = quote! {
        #( #struct_generics, )* #( #container_type_letters, )*
    };

    let inner_types = e.variants.iter().map(Variant::container_inner_type);

    let where_clause_content = quote! {
        #( #container_type_letters2: Default + Extend< #inner_types >, )*
    };

    let return_type = quote! {
        ( #( #container_type_letters3, )* )
    };
    let return_type2 = return_type.clone();

    let container_names = e.variants.iter().map(|v| &v.container_name);
    let container_names2 = container_names.clone();
    let containers_initialization = quote! {
        #( let mut #container_names = #container_type_letters4::default(); )*
    };

    let match_arms = e.variants.iter().map(Variant::match_arm);

    let return_expression = quote! {
        ( #( #container_names2, )* )
    };

    Ok(quote! {
        impl< #trait_generics > Dispatch< #return_type > for #full_type
        where #where_clause_content
        {
            fn dispatch<I>(iter: I) -> #return_type2
            where
                I: Iterator<Item = #full_type2 >
            {
                #containers_initialization

                use #name ::*;

                for element in iter {
                    match element {
                        #( #match_arms )*
                    }
                }

                #return_expression
            }
        }
    })
}

/// An enum.
struct Enum {
    /// The enum name.
    name: Ident,
    /// The generics it may have.
    generics: Generics,
    /// Every variant the enum has.
    variants: Vec<Variant>,
}

/// Generates a *friendly* error message when the `Dispatch` trait is derived
/// on a structure or an union.
fn wrong_type_error(node: &DeriveInput, name: &Ident, type_name: &str) -> Error {
    Error::new_spanned(
        node,
        format!("Edisp can only dispatch enums, `{}` is {}", name, type_name),
    )
}

impl Enum {
    /// Tries to convert a structure, tuple or union into `Enum`.
    ///
    /// # Errors
    ///
    /// This function will return an error if the `Dispatch` trait is being
    /// derived on a struct or on an union.
    fn from_syn(ast: DeriveInput) -> Result<Enum> {
        let name = &ast.ident;

        match ast.data {
            Data::Enum(e) => Enum::from_data_enum(e, ast.ident, ast.generics),
            Data::Struct(_) => Err(wrong_type_error(&ast, name, "a struct")),
            Data::Union(_) => Err(wrong_type_error(&ast, name, "an union")),
        }
    }

    /// Tries to convert a `DataEnum` into an `Enum`, with its name and generics.
    ///
    /// # Errors
    ///
    /// This function returns an error if the creation of a variant fails, as
    /// defined in `Variant::from_s_variant`.
    fn from_data_enum(e: DataEnum, name: Ident, generics: Generics) -> Result<Enum> {
        let variants = e
            .variants
            .into_iter()
            .enumerate()
            .map(|(idx, v)| Variant::from_s_variant(v, &name, idx))
            .collect::<Result<_>>()?;

        Ok(Enum {
            name,
            generics,
            variants,
        })
    }
}

/// An enum variant
struct Variant {
    /// The variant parsed by `syn`.
    inner: SVariant,
    /// The name of the associated type parameter.
    container_type_name: Ident,
    /// The name of the associted container.
    container_name: Ident,
}

impl Variant {
    /// Tries to convert a syn `Variant` to an `edisp` `Variant`.
    ///
    /// # Errors
    ///
    /// This function fails if the variant is an anonymous struct.
    fn from_s_variant(sv: SVariant, e_name: &Ident, idx: usize) -> Result<Variant> {
        let v_name = &sv.ident;

        if matches!(sv.fields, Fields::Named(_)) {
            return Err(wrong_variant_field(
                &sv,
                e_name,
                v_name,
                "an anonymous struct",
            ));
        }

        let container_type_name = container_type_letter(idx);
        let container_name = container_name(idx);
        let inner = sv;

        Ok(Variant {
            inner,
            container_type_name,
            container_name,
        })
    }

    /// Returns the inner type of the associated container.
    ///
    /// If the variant is an unit variant, then the returned token stream is
    /// `()`, otherwise, it is the contained type.
    fn container_inner_type(&self) -> TokenStream2 {
        match self.inner.fields {
            Fields::Unnamed(ref f) => {
                let t = &f.unnamed;
                quote! { #t }
            }
            Fields::Unit => quote! { () },
            _ => unreachable!(),
        }
    }

    /// Returns the content of the enum match arm.
    fn match_arm(&self) -> TokenStream2 {
        let variant_name = &self.inner.ident;
        let container_name = &self.container_name;
        match self.inner.fields {
            Fields::Unnamed(_) => {
                quote! {
                    #variant_name (v) => #container_name .extend(Some(v)),
                }
            }
            Fields::Unit => {
                quote! {
                    #variant_name => #container_name .extend(Some(())),
                }
            }
            _ => unreachable!(),
        }
    }
}

/// Generates a *friendly* error message when `Dispatch` is derived on an enum
/// with a anonymous struct in one of its variants.
fn wrong_variant_field(
    node: &SVariant,
    enum_name: &Ident,
    variant_name: &Ident,
    variant_desc: &str,
) -> Error {
    Error::new_spanned(
        node,
        format!(
            "Edisp can only dispatch enum whith unnamed variants. `{}` has a variant named `{}`, which is {}",
            enum_name,
            variant_name,
            variant_desc
        ),
    )
}

/// Returns a custom type letter.
///
/// This type letter allows to declare generic containers.
fn container_type_letter(n: usize) -> Ident {
    format_ident!("T{}", n)
}

/// Returns a custom container name.
///
/// This container name allows to declare variables in the `dispatch` method.
fn container_name(n: usize) -> Ident {
    format_ident!("c{}", n)
}
