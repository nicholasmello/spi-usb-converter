use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_name = &input.ident;

    let data = match input.data {
        Data::Enum(data) => data,
        _ => {
            return syn::Error::new_spanned(enum_name, "Deserialize can only be derived for enums")
                .to_compile_error()
                .into();
        }
    };

    let variant_count = data.variants.len();

    if variant_count == 0 {
        return syn::Error::new_spanned(enum_name, "Cannot derive Deserialize for an empty enum")
            .to_compile_error()
            .into();
    }

    // Compute required byte width.
    let mut width = 1usize;
    let mut capacity = 256usize;

    while variant_count > capacity {
        width += 1;

        capacity = capacity.saturating_mul(256);

        if capacity == usize::MAX {
            break;
        }
    }

    let arms = data.variants.iter().enumerate().map(|(index, variant)| {
        if !matches!(variant.fields, Fields::Unit) {
            return syn::Error::new_spanned(variant, "Deserialize only supports unit variants")
                .to_compile_error();
        }

        let ident = &variant.ident;

        quote! {
            #index => Some(Self::#ident),
        }
    });

    TokenStream::from(quote! {
        impl Deserialize for #enum_name {
            fn deserialize(data: crate::SerializedData) -> Option<Self> {
                if data.len() != #width {
                    return None;
                }

                let mut value: usize = 0;

                for (i, byte) in data.iter().enumerate() {
                    value |= (*byte as usize) << (i * 8);
                }

                match value {
                    #(#arms)*
                    _ => None,
                }
            }
        }
    })
}
