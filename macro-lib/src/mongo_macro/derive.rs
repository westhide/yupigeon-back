use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Data;

pub(crate) fn expand_derive_collection(_ident: Ident, _data: Data) -> syn::Result<TokenStream> {
    let struct_name_literal = _ident.to_string();

    let ts = quote! {
        impl CollectionTrait for #_ident{
            fn collection_name<'a>() -> &'a str {
                #struct_name_literal
            }

            fn primary_key(&self) -> ObjectId {
                self._id
            }
        }
    };

    Ok(ts)
}
