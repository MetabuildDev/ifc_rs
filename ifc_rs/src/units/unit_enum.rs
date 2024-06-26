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
    /// Represents units of absorbed dose, such as "Gray"
    #[strum(to_string = ".ABSORBEDDOSEUNIT.")]
    AbsorbedDoseUnit,

    /// Represents units of amount of substance, such as "Mole"
    #[strum(to_string = ".AMOUNTOFSUBSTANCEUNIT.")]
    AmountOfSubstanceUnit,

    /// Represents units of area, such as "Square Meter"
    #[strum(to_string = ".AREAUNIT.")]
    AreaUnit,

    /// Represents units of dose equivalent, such as "Sievert"
    #[strum(to_string = ".DOSEEQUIVALENTUNIT.")]
    DoseEquivalentUnit,

    /// Represents units of electric capacitance, such as "Farad"
    #[strum(to_string = ".ELCTRICCAPACITANCEUNIT.")]
    ElectricCapacitanceUnit,

    /// Represents units of electric charge, such as "Coulomb"
    #[strum(to_string = ".ELCTRICCHARGEUNIT.")]
    ElectricChargeUnit,

    /// Represents units of electric conductance, such as "Siemens"
    #[strum(to_string = ".ELCTRICCONDUCTANCEUNIT.")]
    ElectricConductanceUnit,

    /// Represents units of electric current, such as "Ampere"
    #[strum(to_string = ".ELCTRICCURRENTUNIT.")]
    ElectricCurrentUnit,

    /// Represents units of electric resistance, such as "Ohm"
    #[strum(to_string = ".ELCTRICRESISTANCEUNIT.")]
    ElectricResistanceUnit,

    /// Represents units of electric voltage, such as "Volt"
    #[strum(to_string = ".ELCTRICVOLTAGEUNIT.")]
    ElectricVoltageUnit,

    /// Represents units of energy, such as "Joule"
    #[strum(to_string = ".ENERGYUNIT.")]
    EnergyUnit,

    /// Represents units of force, such as "Newton"
    #[strum(to_string = ".FORCEUNIT.")]
    ForceUnit,

    /// Represents units of illuminance, such as "Lux"
    #[strum(to_string = ".ILLUMINANCEUNIT.")]
    IlluminanceUnit,

    /// Represents units of inductance, such as "Henry"
    #[strum(to_string = ".INDUCTANCEUNIT.")]
    InductanceUnit,

    /// Represents units of length, such as "Meter"
    #[strum(to_string = ".LENGTHUNIT.")]
    LengthUnit,

    /// Represents units of luminous flux, such as "Lumen"
    #[strum(to_string = ".LUMINOUSFLUXUNIT.")]
    LuminousFluxUnit,

    /// Represents units of luminous intensity, such as "Candela"
    #[strum(to_string = ".LUMINOUSINTENSITYUNIT.")]
    LuminousIntensityUnit,

    /// Represents units of magnetic flux density, such as "Tesla"
    #[strum(to_string = ".MAGNETICFLUXDENSITYUNIT.")]
    MagneticFluxDensityUnit,

    /// Represents units of magnetic flux, such as "Weber"
    #[strum(to_string = ".MAGNETICFLUXUNIT.")]
    MagneticFluxUnit,

    /// Represents units of mass, such as "Kilogram"
    #[strum(to_string = ".MASSUNIT.")]
    MassUnit,

    /// Represents units of plane angle, such as "Radian"
    #[strum(to_string = ".PLANEANGLEUNIT.")]
    PlaneAngleUnit,

    /// Represents units of power, such as "Watt"
    #[strum(to_string = ".POWERUNIT.")]
    PowerUnit,

    /// Represents units of pressure, such as "Pascal"
    #[strum(to_string = ".PRESSUREUNIT.")]
    PressureUnit,

    /// Represents units of radioactivity, such as "Becquerel"
    #[strum(to_string = ".RADIOACTIVITYUNIT.")]
    RadioActivityUnit,

    /// Represents units of solid angle, such as "Steradian"
    #[strum(to_string = ".SOLIDANGLEUNIT.")]
    SolidAngleUnit,

    /// Represents units of thermodynamic temperature, such as "Kelvin"
    #[strum(to_string = ".THERMODYNAMICTEMPERATUREUNIT.")]
    ThermoDynamicTemperatureUnit,

    /// Represents units of time, such as "Second"
    #[strum(to_string = ".TIMEUNIT.")]
    TimeUnit,

    /// Represents units of volume, such as "Cubic Meter"
    #[strum(to_string = ".VOLUMEUNIT.")]
    VolumeUnit,

    /// Represents user-defined units, which can be any custom unit defined by the user.
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
