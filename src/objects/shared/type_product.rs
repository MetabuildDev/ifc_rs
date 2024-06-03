use std::{fmt::Display, ops::Deref};

use crate::{
    id::Id,
    parser::{
        label::Label,
        optional::{IFCParse, OptionalParameter},
        p_space_or_comment_surrounded, IFCParser,
    },
};

use super::type_object::TypeObject;

pub struct TypeProduct {
    type_object: TypeObject,

    pub representation_maps: OptionalParameter<Id>,
    pub tag: OptionalParameter<Label>,
}

impl Deref for TypeProduct {
    type Target = TypeObject;

    fn deref(&self) -> &Self::Target {
        &self.type_object
    }
}

impl IFCParse for TypeProduct {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                type_object: TypeObject::parse(),
                _: p_space_or_comment_surrounded(","),
                representation_maps: OptionalParameter::parse(),
                _: p_space_or_comment_surrounded(","),
                tag: OptionalParameter::parse()
            }
        }
    }
}

impl Display for TypeProduct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{}",
            self.type_object, self.representation_maps, self.tag
        )
    }
}
