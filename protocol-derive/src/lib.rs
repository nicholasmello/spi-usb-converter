mod derive_deserialize;
mod derive_serialize;
mod derive_test;

use proc_macro::TokenStream;

#[proc_macro_derive(Serialize)]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    derive_serialize::derive_serialize(input)
}

#[proc_macro_derive(Deserialize)]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    derive_deserialize::derive_deserialize(input)
}

#[proc_macro_derive(SerializeTest, attributes(skip_test))]
pub fn derive_test(input: TokenStream) -> TokenStream {
    derive_test::derive_test(input)
}
