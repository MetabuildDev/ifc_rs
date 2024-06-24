use std::str::FromStr;

use strum::{Display, EnumString, VariantNames};
use winnow::combinator::{alt, delimited};
use winnow::Parser;

use crate::parser::*;

/// This enumeration defines the basic configuration of the roof in terms of
/// the different roof shapes.
///
/// Roofs which are subdivided into more than these basic shapes or roofs with
/// non-regular shapes (free form roofs) have the type FREEFORM.
///
/// https://standards.buildingsmart.org/MVD/RELEASE/IFC4/ADD2_TC1/RV1_2/HTML/schema/ifcsharedbldgelements/lexical/ifcrooftypeenum.htm
#[derive(EnumString, VariantNames, Display, Clone, Copy)]
pub enum RoofTypeEnum {
    /// A roof having no slope, or one with only a slight pitch so as to drain
    /// rainwater.
    #[strum(to_string = ".FLAT_ROOF.")]
    FlatRoof,

    /// A roof having a single slope.
    #[strum(to_string = ".SHED_ROOF.")]
    ShedRoof,

    /// A roof sloping downward in two parts from a central ridge, so as to
    /// form a gable at each end.
    #[strum(to_string = ".GABLE_ROOF.")]
    GableRoof,

    /// A roof having sloping ends and sides meeting at an inclined projecting
    /// angle.
    #[strum(to_string = ".HIP_ROOF.")]
    HipRoof,

    /// A roof having a hipped end truncating a gable.
    #[strum(to_string = ".HIPPED_GABLE_ROOF.")]
    HippedGableRoof,

    /// A ridged roof divided on each side into a shallower slope above a steeper one.
    #[strum(to_string = ".GAMBREL_ROOF.")]
    GambrelRoof,

    /// A roof having on each side a steeper lower part and a shallower upper part.
    #[strum(to_string = ".MANSARD_ROOF.")]
    MansardRoof,

    /// A roof or ceiling having a semicylindrical form.
    #[strum(to_string = ".BARREL_ROOF.")]
    BarrelRoof,

    /// A gable roof in the form of a broad Gothic arch, with gently sloping convex surfaces.
    #[strum(to_string = ".RAINBOW_ROOF.")]
    RainbowRoof,

    /// A roof having two slopes, each descending inward from the eaves.
    #[strum(to_string = ".BUTTERFLY_ROOF.")]
    ButterflyRoof,

    /// A pyramidal hip roof.
    #[strum(to_string = ".PAVILION_ROOF.")]
    PavilionRoof,

    /// A hemispherical hip roof.
    #[strum(to_string = ".DOME_ROOF.")]
    DomeRoof,

    /// User-defined slab element.
    #[strum(to_string = ".USERDEFINED.")]
    UserDefined,

    /// Undefined slab element.
    #[strum(to_string = ".NOTDEFINED.")]
    NotDefined,
}

impl IFCParse for RoofTypeEnum {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        let variants: [&str; Self::VARIANTS.len()] =
            Self::VARIANTS.try_into().expect("statically known");

        delimited(
            p_space_or_comment(),
            alt(variants
                .map(|v| (v, Self::from_str(v).expect("valid RoofTypeEnum")))
                .map(|(k, v)| k.map(move |_| v))),
            p_space_or_comment(),
        )
    }
}
