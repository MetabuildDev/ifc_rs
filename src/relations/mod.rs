use winnow::combinator::alt;

use crate::{
    ifc_type::IfcType,
    parser::{IFCParse, IFCParser},
};

pub mod prelude;

pub mod mapped_item;
pub mod representation_map;

pub struct Relation;

impl Relation {
    pub fn parse<'a>() -> impl IFCParser<'a, Box<dyn IfcType>> {
        alt((
            mapped_item::MappedItem::parse_any(),
            representation_map::RepresentationMap::parse_any(),
        ))
    }
}
