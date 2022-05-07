mod mongo_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, DeriveInput, Item};

#[proc_macro_attribute]
pub fn mongo_orm(attr: TokenStream, item: TokenStream) -> TokenStream {
    eprintln!("{:#?}", parse_macro_input!(attr as AttributeArgs));
    let body_ast = parse_macro_input!(item as Item);
    eprintln!("{:#?}", body_ast);
    quote!(#body_ast).into()
}

#[proc_macro_derive(DeriveCollection)]
pub fn derive_collection(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    match mongo_macro::expand_derive_collection(ident, data) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
