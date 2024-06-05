use comma::Comma;

use crate::{
    objects::{shared::element_type::ElementType, walltype::type_enum::WallTypeEnum},
    parser::*,
};

use super::WallType;

impl IFCParse for WallType {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCWALLTYPE("),

                element_type: ElementType::parse(),
                _: Comma::parse(),
                predefined_type: WallTypeEnum::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}
