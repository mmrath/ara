use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Lit, Meta, NestedMeta};
use synstructure::Structure;

#[allow(clippy::needless_pass_by_value)]
pub fn http_status_derive(s: Structure<'_>) -> TokenStream {
    let ast: &DeriveInput = s.ast();

    let enum_type = &ast.ident;

    let mut http_match_wings: Vec<TokenStream> = Vec::new();

    if let Data::Enum(ref _ev) = ast.data {
        s.variants().iter().for_each(|v| {
            let match_wing = process_variant(v);
            http_match_wings.push(match_wing);
        });
    } else {
        panic!("#derive(HttpStatus) can only be applied to enums")
    };

    quote! {
            impl HttpStatus for #enum_type {
                fn status(&self) -> u16 {
                    match self {
                       #( #http_match_wings )*
                    }
                }
            }
    }
}

fn process_variant(variant: &synstructure::VariantInfo<'_>) -> TokenStream {
    let mut http_impls = None;
    for meta_item in variant
        .ast()
        .attrs
        .iter()
        .filter_map(get_http_status_meta_items)
    {
        match meta_item {
            Meta::List(ref m) => {
                if m.nested.len() == 1 {
                    match m.nested.iter().next() {
                        Some(&NestedMeta::Literal(Lit::Int(ref http_lit))) => {
                            if http_impls.is_some() {
                                panic!("Only one http attribute can be specified")
                            } else {
                                let code = http_lit.value();
                                let pat = variant.pat();
                                let http_match_wing = quote! {
                                    #pat => {
                                        return #code as u16
                                    },
                                };
                                http_impls = Some(http_match_wing);
                            }
                        }
                        _ => panic!("Only #[http_status(code)] is currently supported"),
                    }
                }
            }
            _ => panic!("Only #[http_status(code)] is currently supported"),
        }
    }

    if http_impls.is_none() {
        panic!("http_status attribute must be specified for all variants")
    }

    (http_impls.unwrap())
}

fn get_http_status_meta_items(attr: &syn::Attribute) -> Option<Meta> {
    if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "http_status" {
        match attr.parse_meta() {
            Ok(meta) => Some(meta),
            _ => {
                // TODO: produce an error
                None
            }
        }
    } else {
        None
    }
}
