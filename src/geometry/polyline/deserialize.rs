use super::PolyLine;
use crate::parser::{list::IfcList, *};

impl IFCParse for PolyLine {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCPOLYLINE("),
                points: IfcList::parse(),
                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}
