use crate::parser::*;
use std::str::FromStr;
use strum::*;
use winnow::{combinator::*, Parser};

#[derive(Debug, Clone, Copy, EnumString, VariantNames, Display)]
pub enum RepresentationIdentifier {
    /// Point to identify the center of gravity of an element. This value can be used for validation purposes.
    #[strum(to_string = "'CoG'")]
    CoG,
    /// Bounding box as simplified 3D box geometry of an element
    #[strum(to_string = "'Box'")]
    Box,
    /// 2D or 3D annotations
    #[strum(to_string = "'Annotation'")]
    Annotation,
    /// 2D or 3D Axis, or single line, representation of an element
    #[strum(to_string = "'Axis'")]
    Axis,
    /// 2D Foot print, or double line, representation of an element, projected to ground view
    #[strum(to_string = "'FootPrint'")]
    FootPrint,
    /// 3D line representation of a profile being planar, e.g. used for door and window outlines
    #[strum(to_string = "'Profile'")]
    Profile,
    /// 3D Surface representation (an analytical surface of an element plane)
    #[strum(to_string = "'Surface'")]
    Surface,
    /// 3D representation that is not part of the Body representation. This is used, e.g., for opening geometries, if there are to be excluded from an implicit Boolean operation.
    #[strum(to_string = "'Reference'")]
    Reference,
    /// 3D Body representation, e.g. as wireframe, surface, or solid model, of an element
    #[strum(to_string = "'Body'")]
    Body,
    /// 3D Body representation, e.g. as tessellation, or other surface, or boundary representation, added in addition to the solid model (potentially involving Boolean operations) of an element
    #[strum(to_string = "'Body-FallBack'")]
    BodyFallBack,
    /// 3D clearance volume of the element. Such clearance region indicates space that should not intersect with the 'Body' representation of other elements, though may intersect with the 'Clearance' representation of other elements.
    #[strum(to_string = "'Clearance'")]
    Clearance,
    /// Representation of emitting light as a light source within a shape representation
    #[strum(to_string = "'Lighting'")]
    Lighting,
}

impl IFCParse for RepresentationIdentifier {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        let variants: [&str; Self::VARIANTS.len()] =
            Self::VARIANTS.try_into().expect("statically known");

        delimited(
            p_space_or_comment(),
            alt(variants
                .map(|v| {
                    (
                        v,
                        Self::from_str(v).expect("valid RepresentationIdentifier"),
                    )
                })
                .map(|(k, v)| k.map(move |_| v))),
            p_space_or_comment(),
        )
    }
}
