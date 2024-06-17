use syn::{GenericArgument, PathArguments, PathSegment, Type};

#[derive(Debug)]
pub enum IdOrList {
    Id,
    List,
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
                        if let GenericArgument::Type(t) = generic_type {
                            if let Type::Path(path) = t {
                                (path.path.segments[0].ident.to_string().as_str() == "Id")
                                    .then(|| Self::List)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .flatten(),

                _ => unreachable!(),
            },

            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum DataType {
    Type(IdOrList),
    OptionalParameter(IdOrList),
}

impl DataType {
    pub fn new(field_type: &Type) -> Self {
        match field_type {
            Type::Path(path) => {
                let segment = &path.path.segments[0];

                match IdOrList::check_segment(segment) {
                    Some(t) => Self::Type(t),
                    None => match segment.ident.to_string().as_str() {
                        "OptionalParameter" => match &segment.arguments {
                            PathArguments::AngleBracketed(args) => args
                                .args
                                .first()
                                .map(|generic_type| {
                                    if let GenericArgument::Type(t) = generic_type {
                                        if let Type::Path(path) = t {
                                            let segment = &path.path.segments[0];

                                            IdOrList::check_segment(segment)
                                                .map(|t| Self::OptionalParameter(t))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                })
                                .flatten()
                                .expect(&format!(
                                    "unsupported data type: {}",
                                    segment.ident.to_string().as_str(),
                                )),

                            _ => unreachable!(),
                        },

                        _ => panic!(
                            "unsupported data type: {}",
                            segment.ident.to_string().as_str()
                        ),
                    },
                }
            }

            Type::BareFn(_) => {
                todo!("Field Type Bare Fn")
            }
            Type::Group(_) => {
                todo!("Field Type Group")
            }
            Type::ImplTrait(_) => {
                todo!("Field Type Impl Trait")
            }
            Type::Infer(_) => {
                todo!("Field Type Infer")
            }
            Type::Macro(_) => {
                todo!("Field Type Macro")
            }
            Type::Never(_) => {
                todo!("Field Type Never")
            }
            Type::Paren(_) => {
                todo!("Field Type Paren")
            }
            Type::Ptr(_) => {
                todo!("Field Type Pointer")
            }
            Type::Reference(_) => {
                todo!("Field Type Reference")
            }
            Type::Slice(_) => {
                todo!("Field Type Slice")
            }
            Type::TraitObject(_) => {
                todo!("Field Type Trait Object")
            }
            Type::Tuple(_) => {
                todo!("Field Type Tuple")
            }
            Type::Verbatim(_) => {
                todo!("Field Type Verbatim")
            }
            &_ => {
                todo!("exhaustive")
            }
        }
    }
}
