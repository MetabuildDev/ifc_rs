use std::str::FromStr;

use strum::{Display, EnumString, VariantNames};
use winnow::combinator::{alt, delimited};
use winnow::Parser;

use crate::parser::optional::IFCParse;
use crate::parser::*;

/// The IfcGeometricProjectionEnum defines the various representation types
/// that can be semantically distinguished. Often different levels of detail
/// of the shape representation are controlled by the representation type.
///
/// https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcrepresentationresource/lexical/ifcgeometricprojectionenum.htm
#[derive(Debug, EnumString, VariantNames, Display, Clone, Copy)]
pub enum GeometricProjection {
    /// Geometric display representation that shows an abstract, often 1D element
    /// representation, e.g. representing a wall by its axis line.
    #[strum(to_string = ".GRAPH_VIEW.")]
    GraphView,

    /// Geometric display representation that shows an abstract, often 2D element
    /// representation, e.g. representing a wall by its two foot print edges,
    /// surpressing any inner layer representation.
    #[strum(to_string = ".SKETCH_VIEW.")]
    SketchView,

    /// Geometric display representation that shows a full 3D element representation,
    /// e.g. representing a wall by its volumetric body.
    #[strum(to_string = ".MODEL_VIEW.")]
    ModelView,

    /// Geometric display representation that shows a full 2D element representation,
    /// the level of detail often depends on the target scale,
    /// e.g. representing a wall by its two foot print edges and the edges
    /// of all inner layers. The projection is shown in ground view as seen from above.
    #[strum(to_string = ".PLAN_VIEW.")]
    PlanView,

    /// Geometric display representation that shows a full 2D element representation,
    /// the level of detail often depends on the target scale,
    /// e.g. representing a wall by its two foot print edges and the
    /// edges of all inner layers. The projection is shown in ground view as seen from below.
    #[strum(to_string = ".REFLECTED_PLAN_VIEW.")]
    ReflectedPlanView,

    /// Geometric display representation that shows a full 2D element representation,
    /// the level of detail often depends on the target scale,
    /// e.g. representing a wall by its two inner/outer edges and the
    /// edges of all inner layers, if the element is cut by the section line.
    #[strum(to_string = ".SECTION_VIEW.")]
    SectionView,

    /// Geometric display representation that shows a full 2D element representation,
    /// the level of detail often depends on the target scale,
    /// e.g. representing a wall by its bounding edges if the element is within an elevation view.
    #[strum(to_string = ".ELEVATION_VIEW.")]
    ElevationView,

    /// A user defined specification is given by the value of the
    /// UserDefinedTargetView attribute.
    #[strum(to_string = ".USERDEFINED.")]
    UserDefined,

    /// No specification given.
    #[strum(to_string = ".NOTDEFINED.")]
    NotDefined,
}

impl IFCParse for GeometricProjection {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        let variants: [&str; Self::VARIANTS.len()] =
            Self::VARIANTS.try_into().expect("statically known");

        delimited(
            p_space_or_comment(),
            alt(variants
                .map(|v| (v, Self::from_str(v).expect("valid GeometricProjection")))
                .map(|(k, v)| k.map(move |_| v))),
            p_space_or_comment(),
        )
    }
}
