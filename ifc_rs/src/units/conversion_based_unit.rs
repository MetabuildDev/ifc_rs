use std::str::FromStr;
use std::{fmt::Display, ops::Deref};

use strum::{Display, EnumString, VariantNames};

use ifc_rs_verify_derive::IfcVerify;
use winnow::combinator::{alt, delimited};

use super::p_space_or_comment;
use super::{shared::named_unit::NamedUnit, IFCParse, IFCParser};
use crate::{
    id::IdOr,
    parser::optional::OptionalParameter,
    prelude::*,
    units::{comma::Comma, p_space_or_comment_surrounded},
};

/// An IfcConversionBasedUnit is used to define a unit that has a conversion rate to a base unit.
/// To identify some commonly used conversion based units, the standard designations
/// (case insensitive) for the Name attribute are indicated in Table 697.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcconversionbasedunit.htm
#[derive(IfcVerify)]
pub struct ConversionBasedUnit {
    #[inherited]
    named_unit: NamedUnit,

    /// The word, or group of words, by which the conversion based unit is referred to.
    pub name: OptionalParameter<ConversionUnitName>,

    /// The physical quantity from which the converted unit is derived.
    pub conversion_factor: OptionalParameter<IdOr<MeasureWithUnit>>,
}

impl Deref for ConversionBasedUnit {
    type Target = NamedUnit;

    fn deref(&self) -> &Self::Target {
        &self.named_unit
    }
}

impl IFCParse for ConversionBasedUnit {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCCONVERSIONBASEDUNIT("),

                named_unit: NamedUnit::parse(),
                _: Comma::parse(),
                name: OptionalParameter::parse(),
                _: Comma::parse(),
                conversion_factor: OptionalParameter::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for ConversionBasedUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCCONVERSIONBASEDUNIT({},{},{});",
            self.named_unit, self.name, self.conversion_factor
        )
    }
}

impl IfcType for ConversionBasedUnit {}

/// Names that can be used in the IfcConversionBasedUnit:
///
/// An IfcConversionBasedUnit is used to define a unit that has a conversion rate to a base unit.
/// To identify some commonly used conversion based units, the standard designations (case
/// insensitive) for the Name attribute are indicated in Table 697.
#[derive(EnumString, VariantNames, Display, Clone, Copy)]
pub enum ConversionUnitName {
    /// Angle measure equal to π/180 rad
    #[strum(to_string = "'DEGREE'")]
    Degree,

    /// Length measure equal to 25.4 mm
    #[strum(to_string = "'INCH'")]
    Inch,

    /// Length measure equal to 304.8 mm
    #[strum(to_string = "'FOOT'")]
    Foot,

    /// Length measure equal to 914 mm
    #[strum(to_string = "'YARD'")]
    Yard,

    /// Length measure equal to 1609 m
    #[strum(to_string = "'MILE'")]
    Mile,

    /// Area measure equal to 0.0006452 square meters
    #[strum(to_string = "'SQUARE INCH'")]
    SquareInch,

    /// Area measure equal to 0.09290 square meters
    #[strum(to_string = "'SQUARE FOOT'")]
    SquareFoot,

    /// Area measure equal to 0.83612736 square meters
    #[strum(to_string = "'SQUARE YARD'")]
    SquareYard,

    /// Area measure equal to 4046.86 square meters
    #[strum(to_string = "'ACRE'")]
    Acre,

    /// Area measure equal to 2 588 881 square meters
    #[strum(to_string = "'SQUARE MILE'")]
    SquareMile,

    /// Volume measure equal to 0.00001639 cubic meters
    #[strum(to_string = "'CUBIC INCH'")]
    CubicInch,

    /// Volume measure equal to 0.02832 cubic meters
    #[strum(to_string = "'CUBIC FOOT'")]
    CubicFoot,

    /// Volume measure equal to 0.7636 cubic meters
    #[strum(to_string = "'CUBIC YARD'")]
    CubicYard,

    /// Volume measure equal to 0.001 cubic meters
    #[strum(to_string = "'LITRE'")]
    Litre,

    /// Volume measure equal to 0.0000284130625 cubic meters
    #[strum(to_string = "'FLUID OUNCE UK'")]
    FluidOunceUK,

    /// Volume measure equal to 0.00002957353 cubic meters
    #[strum(to_string = "'FLUID OUNCE US'")]
    FluidOunceUS,

    /// Volume measure equal to 0.000568 cubic meters
    #[strum(to_string = "'PINT UK'")]
    PintUK,

    /// Volume measure equal to 0.000473 cubic meters
    #[strum(to_string = "'PINT US'")]
    PintUS,

    /// Volume measure equal to 0.004546 cubic meters
    #[strum(to_string = "'GALLON UK'")]
    GallonUK,

    /// Volume measure equal to 0.003785 cubic meters
    #[strum(to_string = "'GALLON US'")]
    GallonUS,

    /// Mass measure equal to 28.35 g
    #[strum(to_string = "'OUNCE'")]
    Ounce,

    /// Mass measure equal to 0.454 kg
    #[strum(to_string = "'POUND'")]
    Pound,

    /// Mass measure equal to 1016.0469088 kg
    #[strum(to_string = "'TON UK'")]
    TonUK,

    /// Mass measure equal to 907.18474 kg
    #[strum(to_string = "'TON US'")]
    TonUS,

    /// Force measure equal to 4.4482216153 N
    #[strum(to_string = "'LBF'")]
    Lbf,

    /// Force measure equal to 4448.2216153 N
    #[strum(to_string = "'KIP'")]
    Kip,

    /// Pressure measure equal to 6894.7572932 Pa
    #[strum(to_string = "'PSI'")]
    Psi,

    /// Pressure measure equal to 6894757.2932 Pa
    #[strum(to_string = "'KSI'")]
    Ksi,

    /// Time measure equal to 60 s
    #[strum(to_string = "'MINUTE'")]
    Minute,

    /// Time measure equal to 3600 s
    #[strum(to_string = "'HOUR'")]
    Hour,

    /// Time measure equal to 86400 s
    #[strum(to_string = "'DAY'")]
    Day,

    /// Energy measure equal to 1055.056 J
    #[strum(to_string = "'BTU'")]
    Btu,
}

impl IFCParse for ConversionUnitName {
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

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::ConversionBasedUnit;
    use crate::units::IFCParse;

    #[test]
    fn conversion_based_unit_round_trip() {
        let example = "IFCCONVERSIONBASEDUNIT(#52,.PLANEANGLEUNIT.,'DEGREE',#53);";

        let parsed: ConversionBasedUnit = ConversionBasedUnit::parse().parse(example).unwrap();
        let str = parsed.to_string();

        assert_eq!(example, str);
    }
}
