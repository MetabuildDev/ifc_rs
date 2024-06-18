mod data_type;
mod field;

use data_type::DataType;
use field::{Field, IfcTypesTokenType};
use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(IfcVerify, attributes(ifc_types))]
pub fn ifc_type_builder(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);

    let struct_name = ast.ident;

    let ifc_types: Vec<Field> = match ast.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(named_fields) => named_fields
                .named
                .iter()
                .filter_map(|field| {
                    let attribute_infos: Vec<IfcTypesTokenType> = field
                        .attrs
                        .iter()
                        .filter_map(|attribute| {
                            attribute
                                .path()
                                .is_ident("ifc_types")
                                .then(|| attribute.parse_args().ok())
                                .flatten()
                        })
                        .collect();

                    let collected = IfcTypesTokenType {
                        types: attribute_infos
                            .into_iter()
                            .map(|tt| tt.types.into_iter())
                            .flatten()
                            .collect(),
                    };

                    DataType::new(&field.ty)
                        .map(|data_type| {
                            if data_type.needs_arguments() && collected.types.is_empty() {
                                None
                            } else {
                                Some(Field {
                                    struct_name: struct_name.clone(),
                                    variable_name: field
                                        .ident
                                        .as_ref()
                                        .expect("named field should have a name")
                                        .clone(),
                                    data_type,
                                    ifc_types: collected,
                                })
                            }
                        })
                        .flatten()
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

    let check_var_functions: Vec<_> = ifc_types
        .iter()
        .map(|ifc_type| ifc_type.check_function())
        .collect();

    TokenStream::from(quote! {
        impl #impls #struct_name #types #where_clause {
            #(
                #ifc_types
            )*
        }

        impl #impls IfcVerify for #struct_name #types #where_clause {
            fn verify_id_types(&self, ifc: &IFC) -> anyhow::Result<()> {
                #(
                    self.#check_var_functions(ifc)?;
                )*

                Ok(())
            }
        }
    })
}
