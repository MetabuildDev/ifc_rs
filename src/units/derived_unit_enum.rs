use std::str::FromStr;

use strum::{Display, EnumString, VariantNames};
use winnow::combinator::{alt, delimited};
use winnow::Parser;

use crate::parser::*;

/// IfcDerivedUnitEnum is an enumeration type for allowed types of derived units.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcderivedunitenum.htm
#[derive(EnumString, VariantNames, Display, Clone, Copy)]
pub enum DerivedUnitEnum {
    #[strum(to_string = ".ANGULARVELOCITYUNIT.")]
    ANGULARVELOCITYUNIT,

    #[strum(to_string = ".AREADENSITYUNIT.")]
    AREADENSITYUNIT,

    #[strum(to_string = ".COMPOUNDPLANEANGLEUNIT.")]
    COMPOUNDPLANEANGLEUNIT,

    #[strum(to_string = ".DYNAMICVISCOSITYUNIT.")]
    DYNAMICVISCOSITYUNIT,

    #[strum(to_string = ".HEATFLUXDENSITYUNIT.")]
    HEATFLUXDENSITYUNIT,

    #[strum(to_string = ".INTEGERCOUNTRATEUNIT.")]
    INTEGERCOUNTRATEUNIT,

    #[strum(to_string = ".ISOTHERMALMOISTURECAPACITYUNIT.")]
    ISOTHERMALMOISTURECAPACITYUNIT,

    #[strum(to_string = ".KINEMATICVISCOSITYUNIT.")]
    KINEMATICVISCOSITYUNIT,

    #[strum(to_string = ".LINEARVELOCITYUNIT.")]
    LINEARVELOCITYUNIT,

    #[strum(to_string = ".MASSDENSITYUNIT.")]
    MASSDENSITYUNIT,

    #[strum(to_string = ".MASSFLOWRATEUNIT.")]
    MASSFLOWRATEUNIT,

    #[strum(to_string = ".MOISTUREDIFFUSIVITYUNIT.")]
    MOISTUREDIFFUSIVITYUNIT,

    #[strum(to_string = ".MOLECULARWEIGHTUNIT.")]
    MOLECULARWEIGHTUNIT,

    #[strum(to_string = ".SPECIFICHEATCAPACITYUNIT.")]
    SPECIFICHEATCAPACITYUNIT,

    #[strum(to_string = ".THERMALADMITTANCEUNIT.")]
    THERMALADMITTANCEUNIT,

    #[strum(to_string = ".THERMALCONDUCTANCEUNIT.")]
    THERMALCONDUCTANCEUNIT,

    #[strum(to_string = ".THERMALRESISTANCEUNIT.")]
    THERMALRESISTANCEUNIT,

    #[strum(to_string = ".THERMALTRANSMITTANCEUNIT.")]
    THERMALTRANSMITTANCEUNIT,

    #[strum(to_string = ".VAPORPERMEABILITYUNIT.")]
    VAPORPERMEABILITYUNIT,

    #[strum(to_string = ".VOLUMETRICFLOWRATEUNIT.")]
    VOLUMETRICFLOWRATEUNIT,

    #[strum(to_string = ".ROTATIONALFREQUENCYUNIT.")]
    ROTATIONALFREQUENCYUNIT,

    #[strum(to_string = ".TORQUEUNIT.")]
    TORQUEUNIT,

    #[strum(to_string = ".MOMENTOFINERTIAUNIT.")]
    MOMENTOFINERTIAUNIT,

    #[strum(to_string = ".LINEARMOMENTUNIT.")]
    LINEARMOMENTUNIT,

    #[strum(to_string = ".LINEARFORCEUNIT.")]
    LINEARFORCEUNIT,

    #[strum(to_string = ".PLANARFORCEUNIT.")]
    PLANARFORCEUNIT,

    #[strum(to_string = ".MODULUSOFELASTICITYUNIT.")]
    MODULUSOFELASTICITYUNIT,

    #[strum(to_string = ".SHEARMODULUSUNIT.")]
    SHEARMODULUSUNIT,

    #[strum(to_string = ".LINEARSTIFFNESSUNIT.")]
    LINEARSTIFFNESSUNIT,

    #[strum(to_string = ".ROTATIONALSTIFFNESSUNIT.")]
    ROTATIONALSTIFFNESSUNIT,

    #[strum(to_string = ".MODULUSOFSUBGRADEREACTIONUNIT.")]
    MODULUSOFSUBGRADEREACTIONUNIT,

    #[strum(to_string = ".ACCELERATIONUNIT.")]
    ACCELERATIONUNIT,

    #[strum(to_string = ".CURVATUREUNIT.")]
    CURVATUREUNIT,

    #[strum(to_string = ".IONCONCENTRATIONUNIT.")]
    IONCONCENTRATIONUNIT,

    #[strum(to_string = ".LUMINOUSINTENSITYDISTRIBUTIONUNIT.")]
    LUMINOUSINTENSITYDISTRIBUTIONUNIT,

    #[strum(to_string = ".MASSPERLENGTHUNIT.")]
    MASSPERLENGTHUNIT,

    #[strum(to_string = ".MODULUSOFLINEARSUBGRADEREACTIONUNIT.")]
    MODULUSOFLINEARSUBGRADEREACTIONUNIT,

    #[strum(to_string = ".MODULUSOFROTATIONALSUBGRADEREACTIONUNIT.")]
    MODULUSOFROTATIONALSUBGRADEREACTIONUNIT,

    #[strum(to_string = ".PHUNIT.")]
    PHUNIT,

    #[strum(to_string = ".ROTATIONALMASSUNIT.")]
    ROTATIONALMASSUNIT,

    #[strum(to_string = ".SECTIONAREAINTEGRALUNIT.")]
    SECTIONAREAINTEGRALUNIT,

    #[strum(to_string = ".SECTIONMODULUSUNIT.")]
    SECTIONMODULUSUNIT,

    #[strum(to_string = ".SOUNDPOWERLEVELUNIT.")]
    SOUNDPOWERLEVELUNIT,

    #[strum(to_string = ".SOUNDPOWERUNIT.")]
    SOUNDPOWERUNIT,

    #[strum(to_string = ".SOUNDPRESSURELEVELUNIT.")]
    SOUNDPRESSURELEVELUNIT,

    #[strum(to_string = ".SOUNDPRESSUREUNIT.")]
    SOUNDPRESSUREUNIT,

    #[strum(to_string = ".TEMPERATUREGRADIENTUNIT.")]
    TEMPERATUREGRADIENTUNIT,

    #[strum(to_string = ".TEMPERATURERATEOFCHANGEUNIT.")]
    TEMPERATURERATEOFCHANGEUNIT,

    #[strum(to_string = ".THERMALEXPANSIONCOEFFICIENTUNIT.")]
    THERMALEXPANSIONCOEFFICIENTUNIT,

    #[strum(to_string = ".WARPINGCONSTANTUNIT.")]
    WARPINGCONSTANTUNIT,

    #[strum(to_string = ".WARPINGMOMENTUNIT.")]
    WARPINGMOMENTUNIT,

    #[strum(to_string = ".USERDEFINED.")]
    USERDEFINED,
}

impl IFCParse for DerivedUnitEnum {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        let variants: [&str; Self::VARIANTS.len()] =
            Self::VARIANTS.try_into().expect("statically known");

        delimited(
            p_space_or_comment(),
            alt(variants
                .map(|v| (v, Self::from_str(v).expect("valid DerivedUnitEnum")))
                .map(|(k, v)| k.map(move |_| v))),
            p_space_or_comment(),
        )
    }
}
