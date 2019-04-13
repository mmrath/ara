

use quote::{quote,quote_each_token, quote_spanned, pounded_var_names, multi_zip_expr, nested_tuples_pat};
use syn::*;
use syn::Meta::{List, NameValue, Word};
use syn::NestedMeta::{Literal, Meta};
use proc_macro2::{TokenStream, Ident, Span};
use synstructure::{Structure};


crate fn diesel_string_enum_derive(s: Structure<'_>) -> TokenStream {

    //let input = input.to_string();
    let ast: &DeriveInput = s.ast();

    let enum_name = ast.ident.to_string();


    let (to_varchar, from_varchar) = if let Data::Enum(ref data_enum) = ast.data {
        let to_varchar = data_enum.variants.iter().map(
            |variant|{
                let enum_variant = &variant.ident;
                quote!(
                    #enum_name::#enum_variant => out.write_all(stringify!(#enum_variant))?,
                )
            }
        ).fold(Vec::new(), |mut sum,mut u|{sum.push(&mut u); sum});

        let from_varchar = data_enum.variants.iter().map(
            |variant|{
                let enum_variant = &variant.ident;
                quote!(
                    stringify!(#enum_variant) => Ok(#enum_name::#enum_variant),
                )
            }
        ).fold(Vec::new(), |mut sum,mut u|{sum.push(&mut u); sum});
        (to_varchar, from_varchar)
    } else {
        panic!("#derive(CustomError) can only be applied to enums")
    };



    let modname = Ident::new(&format!("error_impl_for_{}", enum_name), Span::call_site());


    (quote! {
        impl ToSql<Varchar, Pg> for #enum_name {
            fn to_sql<W: Write>(&self, out: &mut Output<'_, W, Pg>) -> serialize::Result {
                match *self {
                    #( #to_varchar)*
                }
                Ok(IsNull::No)
            }
        }

        impl FromSql<Varchar, Pg> for #enum_name {
            fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
                match not_none!(bytes) {
                    #( #from_varchar)*
                _ => Err("Unrecognized enum variant".into()),
                }
            }
        }
    }).into()

}



