use comma::Comma;

use crate::objects::slabtype::type_enum::SlabTypeEnum;
use crate::{objects::shared::element_type::ElementType, parser::*};

use super::SlabType;

impl IFCParse for SlabType {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            SlabType {
                _: p_space_or_comment_surrounded("IFCSLABTYPE("),

                element_type: ElementType::parse(),
                _: Comma::parse(),
                predefined_type: SlabTypeEnum::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}
