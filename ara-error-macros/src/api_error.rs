use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::Meta::{List, Word};
use syn::NestedMeta::{Literal, Meta};
use syn::*;
use synstructure::Structure;

#[allow(clippy::needless_pass_by_value)]
pub fn custom_error_derive(s: Structure<'_>) -> TokenStream {
    let ast: &DeriveInput = s.ast();

    let enum_name = ast.ident.to_string();
    let (error_name, _) = enum_name.split_at(enum_name.len() - 4); //Remove Kind
    let error_type = Ident::new(error_name, Span::call_site());

    let main_impl = if let Data::Enum(ref _ev) = ast.data {
        generate_error_type(&ast.ident, &error_type)
    } else {
        panic!("#derive(CustomError) can only be applied to enums")
    };

    let mut from_impls: Vec<TokenStream> = Vec::new();
    let mut http_match_wings: Vec<TokenStream> = Vec::new();

    if let Data::Enum(ref data_enum) = ast.data {
        data_enum.variants.iter().for_each(|v| {
            let (mut f, m) = process_variant(&ast.ident, &error_type, v);
            from_impls.append(&mut f);
            http_match_wings.push(m);
        });
    } else {
        panic!("#derive(CustomError) can only be applied to enums")
    };

    let modname = Ident::new(&format!("error_impl_for_{}", enum_name), Span::call_site());

    (quote! {
        pub use self::#modname::#error_type;

        #[allow(non_snake_case)]
        mod #modname {
            use serde_json::{json, Value};
            use ara_error::HttpResponse;
            use log::{info, error};

            #main_impl
            #( #from_impls )*

            impl HttpResponse for #error_type {
                fn status(&self) -> u16 {
                    match self.kind() {
                       #( #http_match_wings )*
                    }
                }
                fn body(&self) -> Value {
                    let val = serde_json::to_value(self).unwrap_or_else(|e|{
                        error!("Error converting to json Value {:?}. Value will be null", e);
                        Value::Null
                    });
                    info!("Returned value {}", val.to_string());
                    val
                }
            }

        }
    })
}

fn process_variant(
    enum_name: &Ident,
    error_type: &Ident,
    variant: &syn::Variant,
) -> (Vec<TokenStream>, TokenStream) {
    let mut from_impls: Vec<TokenStream> = Vec::new();
    let mut http_impls = None;

    let enum_variant = &variant.ident;
    for meta_items in variant.attrs.iter().filter_map(get_custom_error_meta_items) {
        for meta_item in meta_items {
            match meta_item {
                // Parse `#[serde(bound(serialize = "D: Serialize", deserialize = "D: Deserialize"))]`
                Meta(List(ref m)) if m.ident == "map_from" => {
                    for meta in &m.nested {
                        match *meta {
                            Meta(Word(ref word)) => {
                                let q = quote! {
                                    impl From<#word> for #error_type {
                                        fn from(err: #word) -> #error_type {
                                            use failure::Fail;
                                            #error_type::map_to(#enum_name::#enum_variant)(err)
                                        }
                                    }
                                };
                                from_impls.push(q);
                            }
                            _ => panic!("Only error type expected"),
                        }
                    }
                }
                Meta(List(ref m)) if m.ident == "http" => {
                    for meta in &m.nested {
                        match meta {
                            Literal(Lit::Int(http_lit)) => {
                                if http_impls.is_some() {
                                    panic!("Only one http attribute can be specified")
                                } else {
                                    let code = http_lit.value();
                                    let http_match_wing = quote! {
                                        #enum_name::#enum_variant => {
                                            return #code as u16
                                        },
                                    };
                                    http_impls = Some(http_match_wing);
                                }
                            }
                            _ => panic!("Only error type expected"),
                        }
                    }
                }
                _ => panic!("Only map_from currently supported"),
            }
        }
    }

    if http_impls.is_none() {
        let http_match_wing = quote! {
           #enum_name::#enum_variant => 500 as u16,
        };
        http_impls = Some(http_match_wing);
    }
    (from_impls, http_impls.unwrap())
}

fn get_custom_error_meta_items(attr: &syn::Attribute) -> Option<Vec<syn::NestedMeta>> {
    if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "api_error" {
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

fn generate_error_type(enum_name: &Ident, error_type: &Ident) -> TokenStream {
    let main_impl = quote! {

            use super::*;
            use failure::{Fail,Backtrace,Context,Error};
            use std::error::Error as StdError;
            use std::fmt;

            #[derive(Debug)]
            pub struct #error_type {
                pub(crate) inner: Context<#enum_name>,
            }


            impl StdError for #error_type {
                fn description(&self) -> &str {
                    self.inner.get_context().description()
                }
                fn cause(&self) -> Option<&dyn StdError> {
                    Some(self.inner.get_context())
                }
            }

            impl fmt::Display for #error_type {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    fmt::Display::fmt(&self.inner, f)
                }
            }

            impl #error_type {
                pub fn map_to<T: Into<Error>>(error_kind: #enum_name) -> impl Fn(T) -> #error_type {
                    move |err| #error_type { inner: err.into().context(error_kind) }
                }

                fn kind(&self) -> &#enum_name {
                    self.inner.get_context()
                }
            }

            impl ::serde::Serialize for #error_type {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                    where
                        S: ::serde::Serializer,
                {
                    use ::serde::ser::SerializeStruct;

                    let mut state = serializer.serialize_struct(stringify!(#error_type), 2)?;
                    state.serialize_field("error", stringify!(#error_type))?;
                    state.serialize_field("kind", &self.inner.get_context())?;
                    state.end()
                }
            }

            impl From<#enum_name> for #error_type {
                fn from(kind: #enum_name) -> #error_type {
                    #error_type { inner: Context::new(kind) }
                }
            }

            impl From<Context<#enum_name>> for #error_type {
                fn from(inner: Context<#enum_name>) -> #error_type {
                    #error_type { inner: inner }
                }
            }

    };
    main_impl
}
