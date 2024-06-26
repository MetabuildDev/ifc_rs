use std::str::FromStr;

use strum::{Display, EnumString, VariantNames};
use winnow::combinator::{alt, delimited};
use winnow::Parser;

use crate::parser::*;

/// An SI unit name is the name of an SI unit. The definitions of the names of SI units are specified in ISO 1000 (clause 2).
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcsiunitname.htm
#[derive(EnumString, VariantNames, Display, Clone, Copy)]
pub enum IfcUnitName {
    /// Unit for electric current.
    #[strum(to_string = ".AMPERE.")]
    Ampere,

    /// Unit for radioactivity.
    #[strum(to_string = ".BECQUEREL.")]
    Becquerel,

    /// Unit for luminousintensity.
    #[strum(to_string = ".CANDELA.")]
    Candela,

    /// Unit for electric charge.
    #[strum(to_string = ".COULOUMB.")]
    Couloumb,

    /// Unit for volume.
    #[strum(to_string = ".CUBIC_METRE.")]
    CubicMetre,

    /// Unit for thermodynamic temperature.
    #[strum(to_string = ".DEGREE_CELSIUS.")]
    DegreeCelcius,

    /// Unit for electric capacitance.
    #[strum(to_string = ".FARAD.")]
    Farad,

    /// Unit for mass.
    #[strum(to_string = ".GRAM.")]
    Gram,

    /// Unit for absorbed radioactive dose.
    #[strum(to_string = ".GRAY.")]
    Gray,

    /// Unit for inductance.
    #[strum(to_string = ".HENRY.")]
    Henry,

    /// Unit for frequency.
    #[strum(to_string = ".HERTZ.")]
    Hertz,

    /// Unit for energy.
    #[strum(to_string = ".JOULE.")]
    Joule,

    /// Unit for thermodynamic temperature.
    #[strum(to_string = ".KELVIN.")]
    Kelvin,

    /// Unit for luminous flux.
    #[strum(to_string = ".LUMEN.")]
    Lumen,

    /// Unit for illuminance.
    #[strum(to_string = ".LUX.")]
    Lux,

    /// Unit for length.
    #[strum(to_string = ".METRE.")]
    Metre,

    /// Unit for amount of substance.
    #[strum(to_string = ".MOLE.")]
    Mole,

    /// Unit for force.
    #[strum(to_string = ".NEWTON.")]
    Newton,

    /// Unit for electric resistance.
    #[strum(to_string = ".OHM.")]
    Ohm,

    /// Unit for pressure.
    #[strum(to_string = ".PASCAL.")]
    Pascal,

    /// Unit for plane angle.
    #[strum(to_string = ".RADIAN.")]
    Radian,

    /// Unit for time.
    #[strum(to_string = ".SECOND.")]
    Second,

    /// Unit for electric conductance.
    #[strum(to_string = ".SIEMENS.")]
    Siemens,

    /// Unit for radioactive dose equivalent.
    #[strum(to_string = ".SIEVERT.")]
    Sievert,

    /// Unit for area.
    #[strum(to_string = ".SQUARE_METRE.")]
    SquareMetre,

    /// Unit for solid angle.
    #[strum(to_string = ".STERADIAN.")]
    Steradian,

    /// Unit for magnetic flux density.
    #[strum(to_string = ".TESLA.")]
    Tesla,

    /// Unit for electric voltage.
    #[strum(to_string = ".VOLT.")]
    Volt,

    /// Unit for power.
    #[strum(to_string = ".WATT.")]
    Watt,

    /// Unit for magnetic flux.
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
