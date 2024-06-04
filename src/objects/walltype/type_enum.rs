use std::str::FromStr;

use strum::{Display, EnumString, VariantNames};
use winnow::combinator::{alt, delimited};
use winnow::Parser;

use crate::parser::*;

/// This enumeration defines the different types of walls that can further specify an IfcWall or IfcWallType.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcwalltypeenum.htm
#[derive(EnumString, VariantNames, Display, Clone, Copy)]
pub enum WallTypeEnum {
    /// A movable wall that is either movable, such as folding wall or a sliding wall,
    /// or can be easily removed as a removable partitioning or mounting wall.
    /// Movable walls do normally not define space boundaries and often belong to the furnishing system.
    #[strum(to_string = ".MOVABLE.")]
    Movable,

    /// A wall-like barrier to protect human or vehicle from falling, or to prevent
    /// the spread of fires. Often designed at the edge of balconies, terraces or roofs,
    /// or along edges of bridges.
    #[strum(to_string = ".PARAPET.")]
    Parapet,

    /// A wall designed to partition spaces that often has a light-weight, sandwich-like
    /// construction (e.g. using gypsum board). Partitioning walls are normally non load bearing.
    #[strum(to_string = ".PARTITIONING.")]
    Partitioning,

    /// A pier, or enclosure, or encasement, normally used to enclose plumbing in sanitary rooms.
    /// Such walls often do not extent to the ceiling.
    #[strum(to_string = ".PLUMBINGWALL.")]
    Plumbingwall,

    /// A wall designed to withstand shear loads. Examples of shear wall are diaphragms inside
    /// a box girder, typically on a pier, to resist lateral forces and transfer them to the support.
    #[strum(to_string = ".SHEAR.")]
    Shear,

    /// A massive wall construction for the wall core being the single layer or having multiple
    /// layers attached. Such walls are often masonry or concrete walls (both cast in-situ or precast)
    /// that are load bearing and fire protecting.
    #[strum(to_string = ".SOLIDWALL.")]
    Solidwall,

    /// A standard wall, extruded vertically with a constant thickness along the wall path.
    /// -> The value is deprecated, it is expressed by choosing the subtype IfcWallStandardCase.
    #[strum(to_string = ".STANDARD.")]
    Standard,

    /// A polygonal wall, extruded vertically, where the wall thickness varies along the wall path.
    #[strum(to_string = ".POLYGONAL.")]
    Polygonal,

    /// A stud wall framed with studs and faced with sheetings, sidings, wallboard, or plasterwork.
    /// -> The value is deprecated, it is expressed by choosing the subtype IfcWallElementedCase.
    #[strum(to_string = ".ELEMENTEDWALL.")]
    Elementedwall,

    /// A supporting wall used to protect against soil layers behind. Special types of a retaining
    /// wall may be e.g. Gabion wall and Grib wall. Examples of retaining walls are wing wall,
    /// headwall, stem wall, pierwall and protecting wall.
    #[strum(to_string = ".RETAININGWALL.")]
    Retainingwall,

    /// User-defined wall element.
    #[strum(to_string = ".USERDEFINED.")]
    Userdefined,

    /// Undefined wall element.
    #[strum(to_string = ".NOTDEFINED.")]
    Notdefined,
}

impl IFCParse for WallTypeEnum {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        let variants: [&str; Self::VARIANTS.len()] =
            Self::VARIANTS.try_into().expect("statically known");

        delimited(
            p_space_or_comment(),
            alt(variants
                .map(|v| (v, Self::from_str(v).expect("valid WallTypeEnum")))
                .map(|(k, v)| k.map(move |_| v))),
            p_space_or_comment(),
        )
    }
}
