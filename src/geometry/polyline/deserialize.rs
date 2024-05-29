use super::PolyLine;
use crate::parser::{list::IfcList, optional::IFCParse, *};

impl PolyLine {
    pub(crate) fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCPOLYLINE("),
                points: IfcList::parse(),
                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}
