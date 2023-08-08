use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Table, attributes(sql_json))]
pub fn table(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let data = input.data;

    let fields = match data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => fields,
        _ => panic!("只能用于结构体"),
    };

    let mut fields_vec_innards = quote!();
    let mut struct_init = quote!();
    let mut index = 0usize;
    for field in fields.clone() {
        let mut is_json = false;

        let name = field.ident.unwrap();

        field.attrs.iter().for_each(|v| {
            if v.path().is_ident("sql_json") {
                is_json = true;
            }
        });

        fields_vec_innards.extend(quote!(stringify!(#name),));

        if is_json {
            struct_init.extend(quote!(
                #name:serde_json::from_value(row.try_get(#index)?).unwrap(),
            ));
        } else {
            struct_init.extend(quote!(
                #name:row.try_get(#index)?,
            ));
        }

        index += 1;
    }
    let fields_vec = quote!(vec![#fields_vec_innards]);

    let expanded = quote! {
        impl #name {
            pub fn get_columns_vec() -> Vec<&'static str> {
                return #fields_vec;
            }

            pub fn get_columns() -> String {
                return #fields_vec.join(",");
            }
        }

        impl TryFrom<tokio_postgres::Row> for #name{
            type Error = tokio_postgres::Error;

            fn try_from(row:tokio_postgres::Row) ->std::result::Result<Self, Self::Error> {

                Ok(#name{#struct_init})
            }
        }

        impl<'a> TryFrom<&tokio_postgres::Row> for #name{
            type Error = tokio_postgres::Error;

            fn try_from(row:&tokio_postgres::Row) ->std::result::Result<Self, Self::Error> {

                Ok(#name{#struct_init})
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
