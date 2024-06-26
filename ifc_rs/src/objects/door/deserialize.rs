use comma::Comma;
use optional::OptionalParameter;
use winnow::combinator::alt;

use crate::{objects::shared::element::Element, parser::*};

use super::Door;

impl IFCParse for Door {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: alt((p_space_or_comment_surrounded("IFCDOOR("), p_space_or_comment_surrounded("IFCDOORSTANDARDCASE("))),

                element: Element::parse(),
                _: Comma::parse(),
                overall_height: OptionalParameter::parse(),
                _: Comma::parse(),
                overall_width: OptionalParameter::parse(),
                _: Comma::parse(),
                predefined_type: OptionalParameter::parse(),
                _: Comma::parse(),
                operation_type: OptionalParameter::parse(),
                _: Comma::parse(),
                user_defining_operation_type: OptionalParameter::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

#[test]
fn parse_door_works() {
    use winnow::Parser;
    let data = "IFCDOOR('2jTRqchjf7oB0yhQ6462T0',#12,'Haustuer',$,$,#24218,#27009,'3CAFB746-2204-4C6C-BF-ED-F5FE276FA162',2.,1.,$,$,$);";
    let parsed = Door::parse().parse(data).unwrap();
    assert_eq!(data, parsed.to_string());
}
