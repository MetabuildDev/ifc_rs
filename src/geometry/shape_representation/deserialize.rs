use crate::{
    id::Id,
    parser::{
        optional::{IFCParse, OptionalParameter},
        p_list_of, p_space_or_comment_surrounded,
    },
};

use super::ShapeRepresentation;

impl IFCParse for ShapeRepresentation {
    fn parse<'a>() -> impl crate::parser::IFCParser<'a, Self>
    where
        Self: Sized,
    {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCSHAPEREPRESENTATION("),
                context_of_items: Id::parse(),
                _: p_space_or_comment_surrounded(","),
                representation_identifier: OptionalParameter::parse(),
                _: p_space_or_comment_surrounded(","),
                representation_type: OptionalParameter::parse(),
                _: p_space_or_comment_surrounded(","),
                items: p_list_of::<Id>(),
                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

#[test]
fn parse_shape_representation_works() {
    use winnow::prelude::*;

    let data = "IFCSHAPEREPRESENTATION(#107,'Body','MappedRepresentation',(#2921786));";
    let parsed = ShapeRepresentation::parse().parse(data).unwrap();
    assert_eq!(format!("{data}"), format!("{parsed}"));
}
