use comma::Comma;

use crate::{
    objects::{
        shared::element_type::ElementType,
        spacetype::{deserialize::optional::OptionalParameter, type_enum::SpaceTypeEnum},
    },
    parser::*,
};

use super::SpaceType;

impl IFCParse for SpaceType {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            SpaceType {
                _: p_space_or_comment_surrounded("IFCSPACETYPE("),

                element_type: ElementType::parse(),
                _: Comma::parse(),
                predefined_type: SpaceTypeEnum::parse(),
                _: Comma::parse(),
                long_name: OptionalParameter::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}
