use std::fmt::Display;

use ifc_rs_verify_derive::IfcVerify;

use crate::{
    id::{IdOr, TypedId},
    ifc_type::{IfcType, IfcVerify},
    parser::{comma::Comma, p_space_or_comment_surrounded, IFCParse},
    prelude::*,
    prelude::{ProductDefinitionShape, RepresentationMap, ShapeItem},
};

pub trait TransformableType: IfcType {
    fn shape(&self) -> Option<TypedId<ProductDefinitionShape>>;
}

/// The IfcMappedItem is the inserted instance of a source definition (to be compared with a block
/// / shared cell / macro definition). The instance is inserted by applying a Cartesian
/// transformation operator as the MappingTarget.
///
///   NOTE  A mapped item is a subtype of representation item. It enables a representation to be
///   used as a representation item in one or more other representations. The mapped item allows
///   for the definition of a representation using other representations.
///
///   EXAMPLE  An IfcMappedItem can reuse other mapped items (ako nested blocks), doing so the
///   IfcRepresentationMap is based on an IfcShapeRepresentation including one or more
///   IfcMappedItem's.
///
///   NOTE  Definition according to ISO/CD 10303-43:1992 A mapped item is a type of
///   representation item that specifies the mapping of a representation as an element of the
///   items of a second representation.
///
///   NOTE  Entity adapted from mapped_item defined in ISO 10303-43.
///
///   HISTORY  New entity in IFC2x.
///
/// Informal Propositions:
///
///   A mapped item shall not be self-defining by participating in the definition of the
///   representation being mapped. The dimensionality of the mapping source and the mapping
///   target has to be the same, if the mapping source is a geometric representation item.
#[derive(IfcVerify)]
pub struct MappedItem {
    /// A representation map that is the source of the mapped item. It can be seen as a block (or
    /// cell or marco) definition.
    pub source: IdOr<RepresentationMap>,
    // FIXME: This should be more general. The docs state that this can be anything that's
    // specializing the [IfcCartesianTransformationOperator](https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifccartesiantransformationoperator.htm)
    // See issue #46
    /// A representation item that is the target onto which the mapping source is mapped. It is
    /// constraint to be a Cartesian transformation operator.
    pub target: IdOr<CartesianTransformationOperator3DnonUniform>,
}

impl MappedItem {
    pub fn new(
        source: impl Into<IdOr<RepresentationMap>>,
        target: impl Into<IdOr<CartesianTransformationOperator3DnonUniform>>,
        ifc: &mut IFC,
    ) -> Self {
        Self {
            source: source.into().or_insert(ifc).into(),
            target: target.into().or_insert(ifc).into(),
        }
    }
}

impl IfcType for MappedItem {}
impl ShapeItem for MappedItem {}

impl Display for MappedItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IFCMAPPEDITEM({},{});", self.source, self.target)
    }
}

impl IFCParse for MappedItem {
    fn parse<'a>() -> impl crate::parser::IFCParser<'a, Self>
    where
        Self: Sized,
    {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCMAPPEDITEM("),
                source: IdOr::parse(),
                _: Comma::parse(),
                target: IdOr::parse(),
                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use crate::parser::IFCParse;

    use super::MappedItem;

    #[test]
    fn mapped_item_round_trip() {
        let example = "IFCMAPPEDITEM(#30,#31);";

        let structured = MappedItem::parse().parse(example).unwrap();
        let stringified = structured.to_string();

        assert_eq!(example, stringified);
    }
}
