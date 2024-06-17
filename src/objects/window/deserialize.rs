use comma::Comma;
use optional::OptionalParameter;

use crate::{objects::shared::element::Element, parser::*};

use super::Window;

impl IFCParse for Window {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCWINDOW("),

                element: Element::parse(),
                _: Comma::parse(),
                overall_height: OptionalParameter::parse(),
                _: Comma::parse(),
                overall_width: OptionalParameter::parse(),
                _: Comma::parse(),
                predefined_type: OptionalParameter::parse(),
                _: Comma::parse(),
                partitioning_type: OptionalParameter::parse(),
                _: Comma::parse(),
                user_defining_partitioning_type: OptionalParameter::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

#[test]
fn parse_window_works() {
    use winnow::Parser;
    let data = "IFCWINDOW('0tA4DSHd50le6Ov9Yu0I9X',#2,'Window for Test Example','Description of Window',$,#88,#33,$,1000.,1000.,.WINDOW.,.SINGLE_PANEL.,$);";
    let parsed = Window::parse().parse(data).unwrap();
    assert_eq!(data, parsed.to_string());
}
