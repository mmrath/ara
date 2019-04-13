use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::*;
use synstructure::Structure;

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn http_response_derive(s: Structure<'_>) -> TokenStream {
    //let input = input.to_string();
    let ast: &DeriveInput = s.ast();

    let enum_name = &ast.ident;
    let enum_name_str = ast.ident.to_string();
    let (error_name, _) = enum_name_str.split_at(enum_name_str.len() - 4); //Remove Kind
    let _error_type = Ident::new(error_name, Span::call_site());

    let s = Structure::new(&ast);

    let impls = s.each_variant(|v| {
        let code = process_variant(&v.ast().attrs);
        quote!(#code)
    });

    let modname = Ident::new(
        &format!("__HttpResponse_for_{}", enum_name_str),
        Span::call_site(),
    );

    let tokens = quote! {
        mod #modname {
            use super::#enum_name;
            use serde_json::{json, Value};

            impl ara_error::HttpResponse for #enum_name {
                fn status(&self) -> u16 {
                    match self {
                        #impls
                    }
                }
                fn body(&self) -> Value {
                    json!(self)
                }
            }
        }
    };
    tokens
}

fn process_variant(variant_attrs: &[syn::Attribute]) -> TokenStream {
    let http_status_metas = get_status_code_meta(variant_attrs);

    match http_status_metas {
        Some(lit) => {
            if let Lit::Int(litint) = lit {
                let code = litint.value();
                quote! {
                    #code as u16
                }
            } else {
                panic!("Only integer status code are supported");
            }
        }
        None => quote! {
            500 as u16
        },
    }
}

fn get_status_code_meta(attrs: &[syn::Attribute]) -> Option<(Lit)> {
    let mut http_attrs = None;
    for attr in attrs {
        if let Some(meta) = attr.interpret_meta() {
            if meta.name() == "http" {
                if http_attrs.is_some() {
                    panic!("Cannot have two http attributes")
                } else if let syn::Meta::List(list) = meta {
                    http_attrs = Some(list);
                } else {
                    panic!("http attribute must take a list in parentheses")
                }
            }
        }
    }
    if http_attrs.is_some() {
        let msg = http_attrs.unwrap();

        match msg.nested[0] {
            syn::NestedMeta::Meta(syn::Meta::NameValue(ref nv)) if nv.ident == "status" => {
                Some(nv.lit.clone())
            }
            _ => panic!("http attribute must begin `status = \"\"` ."),
        }
    } else {
        None
    }
}
