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

impl ToTokens for IfcTypesTokenType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let f = &self.types[0];

        let mut first = quote! {
            t.type_id() == std::any::TypeId::of::<#f>()
        };

        first.extend(self.types.iter().skip(1).map(|type_check| {
            quote! {
                || t.type_id() == std::any::TypeId::of::<#type_check>()
            }
        }));

        first.to_tokens(tokens);
    }
}

pub struct Field {
    pub variable_name: Ident,
    pub ifc_types: IfcTypesTokenType,
    pub data_type: DataType,
}

impl Field {
    pub fn check_function(&self) -> Ident {
        format_ident!("check_{}", self.variable_name)
    }
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let var_name = &self.variable_name;
        let ifc_types = &self.ifc_types;

        let check = match &self.data_type {
            DataType::Id(id_or_list) => match id_or_list {
                IdOrList::Id => quote! {

                    let t = ifc.data.get_untyped(self.#var_name);
                    #ifc_types

                },
                IdOrList::List => quote! {

                    self.#var_name.0.iter().all(|id| {
                        let t = ifc.data.get_untyped(*id);
                        #ifc_types
                    })

                },
                IdOrList::TypedId(typed) => quote! {

                    let t = ifc.data.get_untyped(self.#var_name.id());
                    t.type_id() == std::any::TypeId::of::<#typed>()

                },
                IdOrList::TypedIdList(typed) => quote! {

                    self.#var_name.0.iter().all(|typed_id| {
                        let t = ifc.data.get_untyped(typed_id.id());
                        t.type_id() == std::any::TypeId::of::<#typed>()
                    })

                },
                IdOrList::IdOr(typed) => quote! {

                    let t = ifc.data.get_untyped(self.#var_name.id().id());
                    t.type_id() == std::any::TypeId::of::<#typed>()

                },
                IdOrList::IdOrList(typed) => quote! {

                    self.#var_name.0.iter().all(|id_or| {
                        let t = ifc.data.get_untyped(id_or.id().id());
                        t.type_id() == std::any::TypeId::of::<#typed>()
                    })

                },
            },
            DataType::OptionalParameter(optional) => match optional {
                IdOrList::Id => quote! {

                    match self.#var_name.custom() {
                        Some(id) => {
                            let t = ifc.data.get_untyped(*id);
                            #ifc_types

                        }
                        None => true,
                    }

                },
                IdOrList::List => quote! {

                    match self.#var_name.custom() {
                        Some(#var_name) => {
                            #var_name.0.iter().all(|id| {
                                let t = ifc.data.get_untyped(*id);
                                #ifc_types
                            })
                        }
                        None => true,
                    }

                },
                IdOrList::TypedId(typed) => quote! {

                    match self.#var_name.custom() {
                        Some(typed_id) => {
                            let t = ifc.data.get_untyped(typed_id.id());
                            t.type_id() == std::any::TypeId::of::<#typed>()
                        }
                        None => true,
                    }

                },
                IdOrList::TypedIdList(typed) => quote! {

                    match self.#var_name.custom() {
                        Some(#var_name) => {
                            #var_name.0.iter().all(|typed_id| {
                                let t = ifc.data.get_untyped(typed_id.id());
                                t.type_id() == std::any::TypeId::of::<#typed>()
                            })
                        }
                        None => true,
                    }

                },
                IdOrList::IdOr(typed) => quote! {

                    match self.#var_name.custom() {
                        Some(id_or) => {
                            let t = ifc.data.get_untyped(id_or.id().id());
                            t.type_id() == std::any::TypeId::of::<#typed>()
                        }
                        None => true,
                    }

                },
                IdOrList::IdOrList(typed) => quote! {

                    match self.#var_name.custom() {
                        Some(#var_name) => {
                            #var_name.0.iter().all(|id_or| {
                                let t = ifc.data.get_untyped(id_or.id().id());
                                t.type_id() == std::any::TypeId::of::<#typed>()
                            })
                        }
                        None => true,
                    }

                },
            },
        };

        let check_function = self.check_function();

        quote! {
            fn #check_function(&self, ifc: &IFC) -> bool {
                #check
            }
        }
        .to_tokens(tokens);
    }
}
