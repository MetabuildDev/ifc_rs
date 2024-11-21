use crate::parser::*;
use std::str::FromStr;
use strum::*;
use winnow::{combinator::*, Parser};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, EnumString, VariantNames, Display)]
pub enum RepresentationType {
    /// 2 or 3 dimensional point(s). Points can be represented by a point list
    #[strum(to_string = "'Point'")]
    Point,
    /// 3 dimensional points represented by a point list. DEPRECATED. Use 'Point' instead.
    #[strum(to_string = "'PointCloud'")]
    PointCloud,
    /// 2 or 3 dimensional curve(s)
    #[strum(to_string = "'Curve'")]
    Curve,
    /// 2 dimensional curve(s)
    #[strum(to_string = "'Curve2D'")]
    Curve2D,
    /// 3 dimensional curve(s)
    #[strum(to_string = "'Curve3D'")]
    Curve3D,
    /// 2 or 3 dimensional surface(s)
    #[strum(to_string = "'Surface'")]
    Surface,
    /// 2 dimensional surface(s) (a region on ground view)
    #[strum(to_string = "'Surface2D'")]
    Surface2D,
    /// 3 dimensional surface(s)
    #[strum(to_string = "'Surface3D'")]
    Surface3D,
    /// swept surface(s) created by sweeping open profiles along a directrix
    #[strum(to_string = "'SectionedSurface'")]
    SectionedSurface,
    /// 2D region(s) represented as a filled area (hatching)
    #[strum(to_string = "'FillArea'")]
    FillArea,
    /// text defined as text literals
    #[strum(to_string = "'Text'")]
    Text,
    /// 3 dimensional b-spline surface(s)
    #[strum(to_string = "'AdvancedSurface'")]
    AdvancedSurface,
    /// points, curves, surfaces (2 or 3 dimensional)
    #[strum(to_string = "'GeometricSet'")]
    GeometricSet,
    /// points, curves (2 or 3 dimensional)
    #[strum(to_string = "'GeometricCurveSet'")]
    GeometricCurveSet,
    /// points, curves (2 or 3 dimensional), hatches and text (2 dimensional)
    #[strum(to_string = "'Annotation2D'")]
    Annotation2D,
    /// face based and shell based surface model(s), or tessellated surface model(s)
    #[strum(to_string = "'SurfaceModel'")]
    SurfaceModel,
    /// Tessellated surface representation(s) only
    #[strum(to_string = "'Tessellation'")]
    Tessellation,
    /// partial geometry of curves that shall not be rendered separately from the main curve
    #[strum(to_string = "'Segment'")]
    Segment,
    /// including swept solid, Boolean results and Brep bodies; more specific types are:
    #[strum(to_string = "'SolidModel'")]
    SolidModel,
    /// swept area solids, by extrusion and revolution, excluding tapered sweeps
    #[strum(to_string = "'SweptSolid'")]
    SweptSolid,
    /// swept area solids created by sweeping a profile along a directrix, and tapered sweeps
    #[strum(to_string = "'AdvancedSweptSolid'")]
    AdvancedSweptSolid,
    /// Faceted Brep's with and without voids
    #[strum(to_string = "'Brep'")]
    Brep,
    /// Brep's based on advanced faces, with b-spline surface geometry, with and without voids
    #[strum(to_string = "'AdvancedBrep'")]
    AdvancedBrep,
    /// Boolean results of operations between solid models, half spaces and Boolean results
    #[strum(to_string = "'CSG'")]
    CSG,
    /// Boolean differences between swept area solids, half spaces and Boolean results
    #[strum(to_string = "'Clipping'")]
    Clipping,
    /// simplistic 3D representation by a bounding box
    #[strum(to_string = "'BoundingBox'")]
    BoundingBox,
    /// cross section based representation of a spine curve and planar cross sections. It can represent a surface or a solid and the interpolations of the between the cross sections is not defined
    #[strum(to_string = "'SectionedSpine'")]
    SectionedSpine,
    /// light source with (depending on type) position, orientation, light colour, intensity and attenuation
    #[strum(to_string = "'LightSource'")]
    LightSource,
    /// representation based on mapped item(s), referring to a representation map. Note: it can be seen as an inserted block reference. The shape representation of the mapped item has a representation type declaring the type of its representation items.
    #[strum(to_string = "'MappedRepresentation'")]
    MappedRepresentation,
}

impl IFCParse for RepresentationType {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        let variants: [&str; Self::VARIANTS.len()] =
            Self::VARIANTS.try_into().expect("statically known");

        delimited(
            p_space_or_comment(),
            alt(variants
                .map(|v| (v, Self::from_str(v).expect("valid RepresentationType")))
                .map(|(k, v)| k.map(move |_| v))),
            p_space_or_comment(),
        )
    }
}
