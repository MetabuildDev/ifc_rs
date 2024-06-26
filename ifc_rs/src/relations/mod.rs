use winnow::combinator::alt;

use crate::{
    parser::{IFCParse, IFCParser},
    prelude::*,
};

pub(crate) mod mapped_item;
pub(crate) mod prelude;
pub(crate) mod rel_aggregates;
pub(crate) mod rel_associates_material;
pub(crate) mod rel_contained_in_spatial_structure;
pub(crate) mod rel_declares;
pub(crate) mod rel_defines_by_type;
pub(crate) mod rel_fills_element;
pub(crate) mod rel_voids_element;
pub(crate) mod representation_map;

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
