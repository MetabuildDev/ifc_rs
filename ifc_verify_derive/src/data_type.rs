use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{GenericArgument, PathArguments, PathSegment, Type};

#[derive(Debug)]
pub enum IdOrList {
    Id,
    List,
    TypedId(TokenStream),
    TypedIdList(TokenStream),
    IdOr(TokenStream),
    IdOrList(TokenStream),
}

impl IdOrList {
    pub fn check_segment(segment: &PathSegment) -> Option<Self> {
        match segment.ident.to_string().as_str() {
            "Id" => Some(Self::Id),
            "IfcList" => match &segment.arguments {
                PathArguments::AngleBracketed(args) => args
                    .args
                    .first()
                    .map(|generic_type| -> Option<Self> {
                        if let GenericArgument::Type(Type::Path(path)) = generic_type {
                            match path.path.segments[0].ident.to_string().as_str() {
                                "Id" => Some(Self::List),
                                "TypedId" => {
                                    if let PathArguments::AngleBracketed(args) =
                                        &path.path.segments[0].arguments
                                    {
                                        Some(Self::TypedIdList(args.args.to_token_stream()))
                                    } else {
                                        None
                                    }
                                }
                                "IdOr" => {
                                    if let PathArguments::AngleBracketed(args) =
                                        &path.path.segments[0].arguments
                                    {
                                        Some(Self::IdOrList(args.args.to_token_stream()))
                                    } else {
                                        None
                                    }
                                }

                                _ => None,
                            }
                        } else {
                            None
                        }
                    })
                    .flatten(),

                _ => unreachable!(),
            },
            "TypedId" => match &segment.arguments {
                PathArguments::AngleBracketed(args) => {
                    Some(Self::TypedId(args.args.to_token_stream()))
                }
                _ => unreachable!(),
            },
            "IdOr" => match &segment.arguments {
                PathArguments::AngleBracketed(args) => {
                    Some(Self::IdOr(args.args.to_token_stream()))
                }
                _ => unreachable!(),
            },

            _ => None,
        }
    }

    pub fn needs_arguments(&self) -> bool {
        match self {
            Self::Id | Self::List => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum DataType {
    Id(IdOrList),
    OptionalParameter(IdOrList),
}

impl DataType {
    pub fn new(field_type: &Type) -> Option<Self> {
        match field_type {
            Type::Path(path) => {
                let segment = &path.path.segments[0];

                IdOrList::check_segment(segment)
                    .map(|t| Self::Id(t))
                    .or_else(|| {
                        (segment.ident.to_string().as_str() == "OptionalParameter")
                            .then(|| match &segment.arguments {
                                PathArguments::AngleBracketed(args) => args
                                    .args
                                    .first()
                                    .map(|generic_type| {
                                        if let GenericArgument::Type(Type::Path(path)) =
                                            generic_type
                                        {
                                            let segment = &path.path.segments[0];

                                            IdOrList::check_segment(segment)
                                                .map(|t| Self::OptionalParameter(t))
                                        } else {
                                            None
                                        }
                                    })
                                    .flatten(),

                                _ => unreachable!(),
                            })
                            .flatten()
                    })
            }

            _ => None,
        }
    }

    pub fn needs_arguments(&self) -> bool {
        match self {
            DataType::Id(id_or_list) => id_or_list.needs_arguments(),
            DataType::OptionalParameter(id_or_list) => id_or_list.needs_arguments(),
        }
    }
}
