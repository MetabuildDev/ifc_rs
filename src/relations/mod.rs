use winnow::combinator::{alt, fail};

use crate::{ifc_type::IfcType, parser::IFCParser};

pub mod prelude;

pub struct Relation;

impl Relation {
    pub fn parse<'a>() -> impl IFCParser<'a, Box<dyn IfcType>> {
        alt((fail,))
    }
}
