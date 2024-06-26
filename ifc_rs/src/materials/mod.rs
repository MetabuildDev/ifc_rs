use winnow::combinator::alt;

use crate::{
    parser::{IFCParse, IFCParser},
    prelude::*,
};

pub(crate) mod direction_sense_enum;
pub(crate) mod layer_set_direction_enum;
pub(crate) mod material;
pub(crate) mod material_constituent;
pub(crate) mod material_constituent_set;
pub(crate) mod material_layer;
pub(crate) mod material_layer_set;
pub(crate) mod material_layer_set_usage;
pub(crate) mod prelude;

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
