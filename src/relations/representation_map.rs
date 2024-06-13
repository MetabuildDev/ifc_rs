use std::fmt::Display;

use crate::{
    id::IdOr,
    ifc_type::IfcType,
    parser::{comma::Comma, p_space_or_comment_surrounded, IFCParse},
    prelude::{Axis3D, ShapeRepresentation},
    IFC,
};

/// An IfcRepresentationMap defines the base definition (also referred to as block, cell or macro)
/// called MappedRepresentation within the MappingOrigin. The MappingOrigin defines the coordinate
/// system in which the MappedRepresentation is defined.
///
///   NOTE  Definition according to ISO/CD 10303-43:1992 A representation map is the
///   identification of a representation and a representation item in that representation for the
///   purpose of mapping. The representation item defines the origin of the mapping. The
///   representation map is used as the source of a mapping by a mapped item.
///
/// The RepresentationMap is used through an IfcMappeditem in one or several
/// IfcShapeRepresentation's. An Cartesian transformation operator can be applied to transform the
/// MappedRepresentation into the placement coordinate system of the shape representation. The
/// transformation of the representation map is restricted to be a Cartesian transformation mapping
/// (translation, rotation, mirroring and scaling).
///
///   NOTE  The definition of a mapping which is used to specify a new representation item
///   comprises a representation map and a mapped item entity. Without both entities, the mapping
///   is not fully defined. Two entities are specified to allow the same source representation to
///   be mapped into multiple new representations.
///
///   NOTE  Entity adapted from representation_map defined in ISO 10303-43.
///
///   HISTORY  New entity in IFC2x.
pub struct RepresentationMap {
    /// An axis2 placement that defines the position about which the mapped representation is mapped.
    pub origin: IdOr<Axis3D>,
    // FIXME: This should be more general. The docs state that this can be anything that's
    // specializing the [IfcRepresentation](https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcrepresentation.htm).
    // See issue #46
    /// A representation that is mapped to at least one mapped item.
    pub representation: IdOr<ShapeRepresentation>,
}

impl RepresentationMap {
    pub fn new(
        origin: impl Into<IdOr<Axis3D>>,
        representation: impl Into<IdOr<ShapeRepresentation>>,
        ifc: &mut IFC,
    ) -> Self {
        Self {
            origin: origin.into().into_id(ifc),
            representation: representation.into().into_id(ifc),
        }
    }
}

impl IfcType for RepresentationMap {}

impl Display for RepresentationMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCREPRESENTATIONMAP({},{});",
            self.origin, self.representation
        )
    }
}

impl IFCParse for RepresentationMap {
    fn parse<'a>() -> impl crate::parser::IFCParser<'a, Self>
    where
        Self: Sized,
    {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCREPRESENTATIONMAP("),
                origin: IdOr::parse(),
                _: Comma::parse(),
                representation: IdOr::parse(),
                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use crate::parser::IFCParse;

    use super::RepresentationMap;

    #[test]
    fn representation_map_round_trip() {
        let example = "IFCREPRESENTATIONMAP(#30,#31);";

        let structured = RepresentationMap::parse().parse(example).unwrap();
        let stringified = structured.to_string();

        assert_eq!(example, stringified);
    }
}
