//mod diesel_string_enum;

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::Meta::{List, Word};
use syn::NestedMeta::Meta;
use syn::*;
use synstructure::Structure;

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn from_error_derive(s: Structure<'_>) -> TokenStream {
    //let input = input.to_string();
    let ast: &DeriveInput = s.ast();

    let enum_name = ast.ident.to_string();

    let impls = if let Data::Enum(ref data_enum) = ast.data {
        data_enum
            .variants
            .iter()
            .map(|v| process_variant(&ast.ident, v))
            .fold(Vec::new(), |mut sum, mut u| {
                sum.append(&mut u);
                sum
            })
    } else {
        panic!("#derive(FromError) can only be applied to enums")
    };

    let modname = Ident::new(
        &format!("__from_error_impl_for_{}", enum_name),
        Span::call_site(),
    );

    (quote! {
        mod #modname{
            use super::#enum_name;
            use ara_error::ApiError;

            #( #impls )*
        }
    })
}

fn process_variant(enum_name: &Ident, variant: &syn::Variant) -> Vec<TokenStream> {
    let mut impls: Vec<TokenStream> = Vec::new();
    let enum_variant = &variant.ident;
    for meta_items in variant.attrs.iter().filter_map(get_custom_error_meta_items) {
        for meta_item in meta_items {
            match meta_item {
                Meta(Word(ref word)) => {
                    let q = quote! {
                        impl From<#word> for ApiError<#enum_name> {
                            fn from(err: #word) -> ApiError<#enum_name> {
                                ApiError::map_with_context(#enum_name::#enum_variant, err)
                            }
                        }
                    };
                    impls.push(q);
                }
                _ => panic!("Only error type expected"),
            }
        }
    }
    impls
}

fn get_custom_error_meta_items(attr: &syn::Attribute) -> Option<Vec<syn::NestedMeta>> {
    if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "from_error" {
        match attr.interpret_meta() {
            Some(List(ref meta)) => Some(meta.nested.iter().cloned().collect()),
            _ => {
                // TODO: produce an error
                None
            }
        }
    } else {
        None
    }
}
