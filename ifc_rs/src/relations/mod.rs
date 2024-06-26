use winnow::combinator::alt;

use crate::{
    parser::{IFCParse, IFCParser},
    prelude::*,
};

pub mod mapped_item;
pub mod prelude;
pub mod rel_aggregates;
pub mod rel_associates_material;
pub mod rel_contained_in_spatial_structure;
pub mod rel_declares;
pub mod rel_defines_by_type;
pub mod rel_fills_element;
pub mod rel_voids_element;
pub mod representation_map;

pub struct Relation;

impl Relation {
    pub fn parse<'a>() -> impl IFCParser<'a, Box<dyn IfcType>> {
        alt((
            rel_aggregates::RelAggregates::parse_any(),
            rel_associates_material::RelAssociatesMaterial::parse_any(),
            rel_contained_in_spatial_structure::RelContainedInSpatialStructure::parse_any(),
            rel_declares::RelDeclares::parse_any(),
            rel_defines_by_type::RelDefinesByType::parse_any(),
            rel_voids_element::RelVoidsElement::parse_any(),
            rel_fills_element::RelFillsElement::parse_any(),
            mapped_item::MappedItem::parse_any(),
            representation_map::RepresentationMap::parse_any(),
        ))
    }
}
