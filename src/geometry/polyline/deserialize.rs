use super::PolyLine;
use crate::{id::Id, parser::*};

impl PolyLine {
    pub(crate) fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCPOLYLINE("),
                points: p_list_of::<Id>(),
                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}
