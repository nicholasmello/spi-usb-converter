use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_name = &input.ident;

    let data = match input.data {
        Data::Enum(data) => data,
        _ => {
            return syn::Error::new_spanned(enum_name, "Serialize can only be derived for enums")
                .to_compile_error()
                .into();
        }
    };

    let variant_count = data.variants.len();

    if variant_count == 0 {
        return syn::Error::new_spanned(enum_name, "Cannot derive Serialize for an empty enum")
            .to_compile_error()
            .into();
    }

    // Number of bytes required to encode every variant.
    //
    // 1..=256 variants      -> 1 byte
    // 257..=65_536          -> 2 bytes
    // 65_537..=16_777_216   -> 3 bytes
    // etc.
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
            return syn::Error::new_spanned(variant, "Serialize only supports unit variants")
                .to_compile_error();
        }

        let ident = &variant.ident;
        let value = index as u64;

        quote! {
            Self::#ident => {
                let mut bytes = alloc::vec::Vec::with_capacity(#width);
                let mut value = #value;

                for _ in 0..#width {
                    bytes.push((value & 0xff) as u8);
                    value >>= 8;
                }

                bytes.into()
            }
        }
    });

    TokenStream::from(quote! {
        impl Serialize for #enum_name {
            fn serialize(&self) -> crate::SerializedData {
                match self {
                    #(#arms),*
                }
            }
        }
    })
}
