use comma::Comma;
use optional::OptionalParameter;

use crate::parser::*;
use crate::prelude::SpatialStructureElement;

use super::Space;

impl IFCParse for Space {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCSPACE("),

                spatial_element_structure: SpatialStructureElement::parse(),
                _: Comma::parse(),
                predefined_type: OptionalParameter::parse(),
                _: Comma::parse(),
                elevation_with_flooring: OptionalParameter::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}
