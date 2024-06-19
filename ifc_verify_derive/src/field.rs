use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream, Result},
    Attribute, Ident, Token,
};

use crate::data_type::{DataType, IdOrList};

pub struct IfcTypesTokenType {
    pub types: Vec<Ident>,
}

impl Parse for IfcTypesTokenType {
    fn parse(input: ParseStream) -> Result<Self> {
        let _attrs = input.call(Attribute::parse_outer)?;

        let types: Vec<_> = input
            .parse_terminated(|i| i.parse::<Ident>(), Token![,])?
            .into_iter()
            .map(|ident| ident)
            .collect();

        Ok(Self { types })
    }
}

pub struct Field {
    pub struct_name: Ident,
    pub variable_name: Ident,
    pub ifc_types: IfcTypesTokenType,
    pub data_type: DataType,
}

impl Field {
    pub fn check_function(&self) -> Ident {
        format_ident!("check_{}", self.variable_name)
    }

    fn loop_check(&self) -> TokenStream {
        let struct_name = self.struct_name.to_string();
        let var_name = self.variable_name.to_string();

        let mut def = quote! {
            let mut correct_type = false;
        };

        let checks = self.ifc_types.types.iter().map(|type_check| {
            quote! {

                if t.type_id() == std::any::TypeId::of::<#type_check>() {
                    correct_type = true;
                }

            }
        });

        let type_names = self
            .ifc_types
            .types
            .iter()
            .map(|type_check| type_check.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        let check_error = quote! {
            if !correct_type {
                anyhow::bail!("Variable {} of type {} isn't any of these types: {} instead is: {}", #var_name, #struct_name, #type_names, t.type_name());
            }
        };

        def.extend(checks);
        def.extend(check_error);

        def
    }

    fn single_check(&self, typed: &TokenStream) -> TokenStream {
        let struct_name = self.struct_name.to_string();
        let var_name = self.variable_name.to_string();
        let typed_str = typed.to_string();

        quote! {
            if t.type_id() != std::any::TypeId::of::<#typed>() {
                anyhow::bail!("Variable {} of type {}: expected type {} but found {}", #var_name, #struct_name, #typed_str, t.type_name());
            }
        }
    }
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let var_name = &self.variable_name;

        let check = match &self.data_type {
            DataType::Id(id_or_list) => match id_or_list {
                IdOrList::Id => {
                    let multiple = self.loop_check();

                    quote! {

                        let t = ifc.data.get_untyped(self.#var_name);
                        #multiple

                    }
                }
                IdOrList::List => {
                    let multiple = self.loop_check();

                    quote! {

                        self.#var_name.0.iter().try_for_each(|id| {
                            let t = ifc.data.get_untyped(*id);
                            #multiple

                            Ok(())
                        })?;

                    }
                }
                IdOrList::TypedId(typed) => {
                    let single = self.single_check(typed);

                    quote! {

                        let t = ifc.data.get_untyped(self.#var_name.id());
                        #single

                    }
                }
                IdOrList::TypedIdList(typed) => {
                    let single = self.single_check(typed);

                    quote! {

                        self.#var_name.0.iter().try_for_each(|typed_id| {
                            let t = ifc.data.get_untyped(typed_id.id());
                            #single

                            Ok(())
                        })?;

                    }
                }
                IdOrList::IdOr(typed) => {
                    let single = self.single_check(typed);

                    quote! {

                        let t = ifc.data.get_untyped(self.#var_name.id().id());
                        #single

                    }
                }
                IdOrList::IdOrList(typed) => {
                    let single = self.single_check(typed);

                    quote! {

                        self.#var_name.0.iter().try_for_each(|id_or| {
                            let t = ifc.data.get_untyped(id_or.id().id());
                            #single

                            Ok(())
                        })?;

                    }
                }
            },
            DataType::OptionalParameter(optional) => match optional {
                IdOrList::Id => {
                    let multiple = self.loop_check();

                    quote! {

                        if let Some(id) = self.#var_name.custom() {
                            let t = ifc.data.get_untyped(*id);
                            #multiple
                        }

                    }
                }
                IdOrList::List => {
                    let multiple = self.loop_check();

                    quote! {

                        if let Some(#var_name) = self.#var_name.custom() {
                            #var_name.0.iter().try_for_each(|id| {
                                let t = ifc.data.get_untyped(*id);
                                #multiple

                                Ok(())
                            })?;
                        }

                    }
                }
                IdOrList::TypedId(typed) => {
                    let single = self.single_check(typed);

                    quote! {

                        if let Some(typed_id) = self.#var_name.custom() {
                            let t = ifc.data.get_untyped(typed_id.id());
                            #single
                        }

                    }
                }
                IdOrList::TypedIdList(typed) => {
                    let single = self.single_check(typed);

                    quote! {

                        if let Some(#var_name) = self.#var_name.custom() {
                            #var_name.0.iter().try_for_each(|typed_id| {
                                let t = ifc.data.get_untyped(typed_id.id());
                                #single

                                Ok(())
                            })?;
                        }

                    }
                }
                IdOrList::IdOr(typed) => {
                    let single = self.single_check(typed);

                    quote! {

                        if let Some(id_or) = self.#var_name.custom() {
                            let t = ifc.data.get_untyped(id_or.id().id());
                            #single
                        }

                    }
                }
                IdOrList::IdOrList(typed) => {
                    let single = self.single_check(typed);

                    quote! {

                        if let Some(#var_name) = self.#var_name.custom() {
                            #var_name.0.iter().try_for_each(|id_or| {
                                let t = ifc.data.get_untyped(id_or.id().id());
                                #single

                                Ok(())
                            })?;
                        }

                    }
                }
            },
        };

        let check_function = self.check_function();

        quote! {
            fn #check_function(&self, ifc: &IFC) -> anyhow::Result<()> {
                #check

                Ok(())
            }
        }
        .to_tokens(tokens);
    }
}