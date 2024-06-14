use comma::Comma;
use optional::OptionalParameter;

use crate::{objects::shared::element::Element, parser::*};

use super::OpeningElement;

impl IFCParse for OpeningElement {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCOPENINGELEMENT("),

                element: Element::parse(),
                _: Comma::parse(),
                predefined_type: OptionalParameter::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

#[test]
fn parse_opening_element_works() {
    use winnow::Parser;
    let data = "IFCOPENINGELEMENT('2bJiss68D6hvLKV8O1xmqJ',#2,'Opening Element for Test Example','Description of Opening',$,#84,#31,$,.OPENING.);";
    let parsed = OpeningElement::parse().parse(data).unwrap();
    assert_eq!(data, parsed.to_string());
}
