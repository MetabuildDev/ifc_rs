use std::str::FromStr;

use strum::{Display, EnumString, VariantNames};
use winnow::combinator::{alt, delimited};
use winnow::Parser;

use crate::parser::*;

/// IfcUnitEnum is an enumeration type for allowed unit types of IfcNamedUnit.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcmeasureresource/lexical/ifcunitenum.htm
#[derive(EnumString, VariantNames, Display, Clone, Copy)]
pub enum IfcUnitEnum {
    #[strum(to_string = ".ABSORBEDDOSEUNIT.")]
    AbsorbedDoseUnit,

    #[strum(to_string = ".AMOUNTOFSUBSTANCEUNIT.")]
    AmountOfSubstanceUnit,

    #[strum(to_string = ".AREAUNIT.")]
    AreaUnit,

    #[strum(to_string = ".DOSEEQUIVALENTUNIT.")]
    DoseEquivalentUnit,

    #[strum(to_string = ".ELECTRICCAPACITANCEUNIT.")]
    ElectricCapacitanceUnit,

    #[strum(to_string = ".ELECTRICCHARGEUNIT.")]
    ElectricChargeUnit,

    #[strum(to_string = ".ELECTRICCONDUCTANCEUNIT.")]
    ElectricConductanceUnit,

    #[strum(to_string = ".ELECTRICCURRENTUNIT.")]
    ElectricCurrentUnit,

    #[strum(to_string = ".ELECTRICRESISTANCEUNIT.")]
    ElectricResistanceUnit,

    #[strum(to_string = ".ELECTRICVOLTAGEUNIT.")]
    ElectricVoltageUnit,

    #[strum(to_string = ".ENERGYUNIT.")]
    EnergyUnit,

    #[strum(to_string = ".FORCEUNIT.")]
    ForceUnit,

    #[strum(to_string = ".ILLUMINANCEUNIT.")]
    IlluminanceUnit,

    #[strum(to_string = ".INDUCTANCEUNIT.")]
    InductanceUnit,

    #[strum(to_string = ".LENGTHUNIT.")]
    LengthUnit,

    #[strum(to_string = ".LUMINOUSFLUXUNIT.")]
    LuminousFluxUnit,

    #[strum(to_string = ".LUMINOUSINTENSITYUNIT.")]
    LuminousIntensityUnit,

    #[strum(to_string = ".MAGNETICFLUXDENSITYUNIT.")]
    MagneticFluxDensityUnit,

    #[strum(to_string = ".MAGNETICFLUXUNIT.")]
    MagneticFluxUnit,

    #[strum(to_string = ".MASSUNIT.")]
    MassUnit,

    #[strum(to_string = ".PLANEANGLEUNIT.")]
    PlaneAngleUnit,

    #[strum(to_string = ".POWERUNIT.")]
    PowerUnit,

    #[strum(to_string = ".PRESSUREUNIT.")]
    PressureUnit,

    #[strum(to_string = ".RADIOACTIVITYUNIT.")]
    RadioActivityUnit,

    #[strum(to_string = ".SOLIDANGLEUNIT.")]
    SolidAngleUnit,

    #[strum(to_string = ".THERMODYNAMICTEMPERATUREUNIT.")]
    ThermoDynamicTemperatureUnit,

    #[strum(to_string = ".TIMEUNIT.")]
    TimeUnit,

    #[strum(to_string = ".VOLUMEUNIT.")]
    VolumeUnit,

    #[strum(to_string = ".USERDEFINED.")]
    UserDefined,
}

impl IFCParse for IfcUnitEnum {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        let variants: [&str; Self::VARIANTS.len()] =
            Self::VARIANTS.try_into().expect("statically known");

        delimited(
            p_space_or_comment(),
            alt(variants
                .map(|v| (v, Self::from_str(v).expect("valid IfcUnitEnum")))
                .map(|(k, v)| k.map(move |_| v))),
            p_space_or_comment(),
        )
    }
}
