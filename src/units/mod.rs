pub mod assignment;
pub mod conversion_based_unit;
pub mod unit_enum;
pub mod name;
pub mod prefix;
pub mod prelude;
pub mod shared;
pub mod si_unit;

use std::str::FromStr;

use strum::{Display, EnumString, VariantNames};
use winnow::combinator::{alt, delimited};
use winnow::Parser;

use crate::ifc_type::IfcType;
use crate::parser::*;

pub struct Units;

impl Units {
    pub fn parse<'a>() -> impl IFCParser<'a, Box<dyn IfcType>> {
        alt((
            assignment::UnitAssigment::parse_any(),
            conversion_based_unit::ConversionBasedUnit::parse_any(),
            si_unit::SiUnit::parse_any(),
        ))
    }
}

// TODO: there are a lot more (mostly imperial units)
#[derive(EnumString, VariantNames, Display, Clone, Copy)]
pub enum ConversionUnit {
    #[strum(to_string = "'DEGREE'")]
    Degree,

    #[strum(to_string = "'LITRE'")]
    Litre,

    #[strum(to_string = "'MINUTE'")]
    Minute,

    #[strum(to_string = "'HOUR'")]
    Hour,

    #[strum(to_string = "'DAY'")]
    Day,
}

impl IFCParse for ConversionUnit {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        let variants: [&str; Self::VARIANTS.len()] =
            Self::VARIANTS.try_into().expect("statically known");

        delimited(
            p_space_or_comment(),
            alt(variants
                .map(|v| (v, Self::from_str(v).expect("valid ConversionUnit")))
                .map(|(k, v)| k.map(move |_| v))),
            p_space_or_comment(),
        )
    }
}
