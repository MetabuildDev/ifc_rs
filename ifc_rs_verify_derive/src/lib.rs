mod data_type;
mod field;
mod inherited;

use data_type::DataType;
use field::{Field, IfcTypesTokenType};
use inherited::InheritedField;
use proc_macro::TokenStream;

use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

enum FieldType {
    Normal(Field),
    Inherited(InheritedField),
}

impl FieldType {
    fn split(v: Vec<Self>) -> (Vec<Field>, Vec<InheritedField>) {
        let mut fields = Vec::new();
        let mut inheriteds = Vec::new();

        for f_type in v {
            match f_type {
                FieldType::Normal(field) => fields.push(field),
                FieldType::Inherited(inherited) => inheriteds.push(inherited),
            }
        }

        (fields, inheriteds)
    }
}

impl ToTokens for FieldType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            FieldType::Normal(field) => quote! {#field},
            FieldType::Inherited(inherited) => quote! {#inherited},
        }
        .to_tokens(tokens);
    }
}

#[proc_macro_derive(IfcVerify, attributes(ifc_types, inherited))]
pub fn ifc_type_builder(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);

    let struct_name = ast.ident;

    let ifc_types: Vec<FieldType> = match ast.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(named_fields) => named_fields
                .named
                .iter()
                .filter_map(|field| {
                    let attribute_infos: Vec<IfcTypesTokenType> = field
                        .attrs
                        .iter()
                        .filter_map(|attribute| {
                            if attribute.path().is_ident("ifc_types") {
                                attribute.parse_args().ok()
                            } else if attribute.path().is_ident("inherited") {
                                Some(IfcTypesTokenType::Inherited)
                            } else {
                                None
                            }
                        })
                        .collect();

                    let collected = IfcTypesTokenType::merge(attribute_infos);
                    let variable_name = field
                        .ident
                        .as_ref()
                        .expect("named field should have a name")
                        .clone();

                    match collected.clone() {
                        IfcTypesTokenType::Types(types) => {
                            DataType::new(&field.ty).and_then(|data_type| {
                                if data_type.needs_arguments() && types.is_empty() {
                                    None
                                } else {
                                    Some(FieldType::Normal(Field {
                                        struct_name: struct_name.clone(),
                                        variable_name,
                                        data_type,
                                        ifc_types: collected,
                                    }))
                                }
                            })
                        }
                        IfcTypesTokenType::Inherited => {
                            Some(FieldType::Inherited(InheritedField { variable_name }))
                        }
                    }
                })
                .collect(),
            Fields::Unnamed(_) => Vec::new(),
            Fields::Unit => {
                todo!("IfcVerify is not implemented for unit fields.")
            }
        },
        Data::Enum(_) => {
            todo!("IfcVerify is not implemented for enums.")
        }
        Data::Union(_) => {
            todo!("IfcVerify is not implemented for unions.")
        }
    };

    let (impls, types, where_clause) = ast.generics.split_for_impl();

    let (fields, inheriteds) = FieldType::split(ifc_types);

    let check_var_functions: Vec<_> = fields.iter().map(|field| field.check_function()).collect();

    TokenStream::from(quote! {
        impl #impls #struct_name #types #where_clause {
            #(
                #fields
            )*
        }

        impl #impls IfcVerify for #struct_name #types #where_clause {
            fn verify_id_types(&self, ifc: &IFC) -> anyhow::Result<()> {
                #(
                    self.#inheriteds.verify_id_types(ifc)?;
                )*

                #(
                    self.#check_var_functions(ifc)?;
                )*

                Ok(())
            }
        }
    })
}
