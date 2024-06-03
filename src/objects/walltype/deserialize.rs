use crate::{
    objects::{shared::element_type::ElementType, walltype::type_enum::WallTypeEnum},
    parser::{optional::IFCParse, *},
};

use super::WallType;

impl IFCParse for WallType {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCWALLTYPE("),

                element_type: ElementType::parse(),
                _: p_space_or_comment_surrounded(","),
                predefined_type: WallTypeEnum::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}
