use winnow::combinator::alt;

use crate::{
    parser::{IFCParse, IFCParser},
    prelude::*,
};

pub mod direction_sense_enum;
pub mod layer_set_direction_enum;
pub mod material;
pub mod material_constituent;
pub mod material_constituent_set;
pub mod material_layer;
pub mod material_layer_set;
pub mod material_layer_set_usage;
pub mod prelude;

pub struct Materials;

impl Materials {
    pub fn parse<'a>() -> impl IFCParser<'a, Box<dyn IfcType>> {
        alt((
            material_layer_set_usage::MaterialLayerSetUsage::parse_any(),
            material_layer_set::MaterialLayerSet::parse_any(),
            material_layer::MaterialLayer::parse_any(),
            material::Material::parse_any(),
            material_constituent::MaterialConstituent::parse_any(),
            material_constituent_set::MaterialConstituentSet::parse_any(),
        ))
    }
}
