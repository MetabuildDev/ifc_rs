use comma::Comma;
use optional::OptionalParameter;

use crate::{objects::shared::element::Element, parser::*};

use super::Slab;

impl IFCParse for Slab {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCSLAB("),

                element: Element::parse(),
                _: Comma::parse(),
                predefined_type: OptionalParameter::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}
