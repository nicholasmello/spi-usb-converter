use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

pub fn derive_test(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_name = &input.ident;

    let test_name = syn::Ident::new(
        &format!(
            "test_serialize_deserialize_{}",
            type_name.to_string().to_lowercase()
        ),
        type_name.span(),
    );

    let data = match input.data {
        Data::Enum(data) => data,
        _ => {
            return syn::Error::new_spanned(
                type_name,
                "SerializeTest can only be derived for enums",
            )
            .to_compile_error()
            .into();
        }
    };

    let variants = data.variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        quote! {
            {
                let value = #type_name::#variant_name;

                let bytes = value.serialize();

                let result = #type_name::deserialize(bytes)
                    .expect("deserialize failed");

                assert_eq!(value, result);
            }
        }
    });

    TokenStream::from(quote! {
        #[cfg(test)]
        mod #test_name {
            use super::*;

            #[test]
            fn #test_name() {
                #(#variants)*
            }
        }
    })
}
