use optional::OptionalParameter;

use crate::{objects::shared::element::Element, parser::*};

use super::Wall;

impl IFCParse for Wall {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCWALL("),

                element: Element::parse(),
                _: p_space_or_comment_surrounded(","),
                predefined_type: OptionalParameter::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}
