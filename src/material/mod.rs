use std::fmt::Display;

use winnow::combinator::alt;

use crate::parser::{IFCParse, IFCParser};

pub mod direction_sense_enum;
pub mod layer_set_direction_enum;
pub mod material;
pub mod material_layer;
pub mod material_layer_set;
pub mod material_layer_set_usage;

pub struct Materials;

impl Materials {
    pub fn parse<'a>() -> impl IFCParser<'a, Box<dyn Display>> {
        alt((
            layer_set_direction_enum::LayerSetDirectionEnum::parse_any(),
            material_layer_set::MaterialLayerSet::parse_any(),
            material_layer::MaterialLayer::parse_any(),
            material::Material::parse_any(),
        ))
    }
}
