use std::{fmt::Display, ops::Deref};

use crate::parser::{
    label::Label,
    optional::{IFCParse, OptionalParameter},
    p_space_or_comment_surrounded, IFCParser,
};

use super::root::Root;

pub struct Object {
    root: Root,

    pub object_type: OptionalParameter<Label>,
}

impl Deref for Object {
    type Target = Root;

    fn deref(&self) -> &Self::Target {
        &self.root
    }
}

impl IFCParse for Object {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                root: Root::parse(),
                _: p_space_or_comment_surrounded(","),
                object_type: OptionalParameter::parse(),
            }
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.root, self.object_type)
    }
}
