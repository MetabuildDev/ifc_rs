use std::fmt::Display;

use crate::{
    geometry::point::Point3D,
    id::{Id, IdOr},
    parser::{
        optional::{IFCParse, OptionalParameter},
        p_space_or_comment_surrounded, IFCParser,
    },
};

use super::object::Object;

pub struct Product {
    object: Object,

    pub object_placement: OptionalParameter<IdOr<Point3D>>,
    pub representation: OptionalParameter<Id>,
}

impl IFCParse for Product {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                object: Object::parse(),
                _: p_space_or_comment_surrounded(","),
                object_placement: OptionalParameter::parse(),
                _: p_space_or_comment_surrounded(","),
                representation: OptionalParameter::parse(),
            }
        }
    }
}

impl Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{}",
            self.object, self.object_placement, self.representation
        )
    }
}
