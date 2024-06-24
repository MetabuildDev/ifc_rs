use comma::Comma;

use crate::objects::rooftype::type_enum::RoofTypeEnum;
use crate::{objects::shared::element_type::ElementType, parser::*};

use super::RoofType;

impl IFCParse for RoofType {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCROOFTYPE("),

                element_type: ElementType::parse(),
                _: Comma::parse(),
                predefined_type: RoofTypeEnum::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}
