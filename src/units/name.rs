use std::str::FromStr;

use strum::{Display, EnumString, VariantNames};
use winnow::combinator::{alt, delimited};
use winnow::Parser;

use crate::parser::optional::IFCParse;
use crate::parser::*;

/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcsiunitname.htm
#[derive(EnumString, VariantNames, Display, Clone, Copy)]
pub enum IfcUnitName {
    #[strum(to_string = ".AMPERE.")]
    Ampere,

    #[strum(to_string = ".BECQUEREL.")]
    Becquerel,

    #[strum(to_string = ".CANDELA.")]
    Candela,

    #[strum(to_string = ".COULOUMB.")]
    Couloumb,

    #[strum(to_string = ".CUBIC_METRE.")]
    CubicMetre,

    #[strum(to_string = ".DEGREE_CELSIUS.")]
    DegreeCelcius,

    #[strum(to_string = ".FARAD.")]
    Farad,

    #[strum(to_string = ".GRAM.")]
    Gram,

    #[strum(to_string = ".GRAY.")]
    Gray,

    #[strum(to_string = ".HENRY.")]
    Henry,

    #[strum(to_string = ".HERTZ.")]
    Hertz,

    #[strum(to_string = ".JOULE.")]
    Joule,

    #[strum(to_string = ".KELVIN.")]
    Kelvin,

    #[strum(to_string = ".LUMEN.")]
    Lumen,

    #[strum(to_string = ".LUX.")]
    Lux,

    #[strum(to_string = ".METRE.")]
    Metre,

    #[strum(to_string = ".MOLE.")]
    Mole,

    #[strum(to_string = ".NEWTON.")]
    Newton,

    #[strum(to_string = ".OHM.")]
    Ohm,

    #[strum(to_string = ".PASCAL.")]
    Pascal,

    #[strum(to_string = ".RADIAN.")]
    Radian,

    #[strum(to_string = ".SECOND.")]
    Second,

    #[strum(to_string = ".SIEMENS.")]
    Siemens,

    #[strum(to_string = ".SIEVERT.")]
    Sievert,

    #[strum(to_string = ".SQUARE_METRE.")]
    SquareMetre,

    #[strum(to_string = ".STERADIAN.")]
    Steradian,

    #[strum(to_string = ".TESLA.")]
    Tesla,

    #[strum(to_string = ".VOLT.")]
    Volt,

    #[strum(to_string = ".WATT.")]
    Watt,

    #[strum(to_string = ".WEBER.")]
    Weber,
}

impl IFCParse for IfcUnitName {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        let variants: [&str; Self::VARIANTS.len()] =
            Self::VARIANTS.try_into().expect("statically known");

        delimited(
            p_space_or_comment(),
            alt(variants
                .map(|v| (v, Self::from_str(v).expect("valid IfcUnitName")))
                .map(|(k, v)| k.map(move |_| v))),
            p_space_or_comment(),
        )
    }
}
