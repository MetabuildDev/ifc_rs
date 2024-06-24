use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Ident;

pub struct InheritedField {
    pub variable_name: Ident,
}

impl ToTokens for InheritedField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let var_name = &self.variable_name;

        quote! {#var_name}.to_tokens(tokens);
    }
}
