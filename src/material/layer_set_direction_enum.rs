use std::str::FromStr;

use strum::{Display, EnumString, VariantNames};
use winnow::combinator::{alt, delimited};
use winnow::Parser;

use crate::parser::*;

/// IfcLayerSetDirectionEnum provides identification of the axis of
/// element geometry, denoting the layer set thickness direction,
/// or direction of layer offsets.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcmaterialresource/lexical/ifclayersetdirectionenum.htm
#[derive(EnumString, VariantNames, Display, Clone, Copy)]
pub enum LayerSetDirectionEnum {
    /// Usually x-axis.
    #[strum(to_string = ".AXIS1.")]
    Axis1,

    /// Usually y-axis.
    #[strum(to_string = ".AXIS2.")]
    Axis2,

    /// Usually z-axis.
    #[strum(to_string = ".AXIS3.")]
    Axis3,
}

impl IFCParse for LayerSetDirectionEnum {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        let variants: [&str; Self::VARIANTS.len()] =
            Self::VARIANTS.try_into().expect("statically known");

        delimited(
            p_space_or_comment(),
            alt(variants
                .map(|v| (v, Self::from_str(v).expect("valid LayerSetDirectionEnum")))
                .map(|(k, v)| k.map(move |_| v))),
            p_space_or_comment(),
        )
    }
}
