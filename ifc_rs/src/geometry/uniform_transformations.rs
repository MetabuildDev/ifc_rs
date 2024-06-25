use std::{fmt::Display, ops::Deref};

use ifc_rs_verify_derive::IfcVerify;

use crate::{
    id::IdOr,
    ifc_type::{IfcType, IfcVerify},
    parser::{p_space_or_comment_surrounded, IFCParse},
    prelude::*,
};

use super::transform_base::{Transform3DBase, TransformBaseMapping};

pub struct TransformMapping<'a> {
    base: TransformBaseMapping<'a>,
}

impl<'a> Deref for TransformMapping<'a> {
    type Target = TransformBaseMapping<'a>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

/// An IfcCartesianTransformationOperator defines a geometric transformation in three-dimensional space.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifccartesiantransformationoperator3d.htm
#[derive(IfcVerify)]
pub struct CartesianTransformationOperator3D {
    #[inherited]
    base: Transform3DBase,
}

impl CartesianTransformationOperator3D {
    pub fn new(
        local_origin: impl Into<IdOr<Point3D>>,
        (axis_x, axis_y, axis_z): (
            impl Into<IdOr<Direction3D>>,
            impl Into<IdOr<Direction3D>>,
            impl Into<IdOr<Direction3D>>,
        ),
        scale: f64,
        ifc: &mut IFC,
    ) -> Self {
        Self {
            base: Transform3DBase::new(local_origin, (axis_x, axis_y, axis_z), scale, ifc),
        }
    }

    pub fn mappings<'a>(&'a self, ifc: &'a IFC) -> TransformMapping<'a> {
        TransformMapping {
            base: self.base.mappings(ifc),
        }
    }
}

impl Deref for CartesianTransformationOperator3D {
    type Target = Transform3DBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl IfcType for CartesianTransformationOperator3D {}
impl Transform3D for CartesianTransformationOperator3D {}

impl Display for CartesianTransformationOperator3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IFCCARTESIANTRANSFORMATIONOPERATOR3D({});", self.base)
    }
}

impl IFCParse for CartesianTransformationOperator3D {
    fn parse<'a>() -> impl crate::parser::IFCParser<'a, Self>
    where
        Self: Sized,
    {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCCARTESIANTRANSFORMATIONOPERATOR3D("),
                base: Transform3DBase::parse(),
                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use crate::parser::IFCParse;

    use super::CartesianTransformationOperator3D;

    #[test]
    fn cartesian_transformation_round_trip() {
        let example = "IFCCARTESIANTRANSFORMATIONOPERATOR3D($,$,#23007,$,$);";

        let structured = CartesianTransformationOperator3D::parse()
            .parse(example)
            .unwrap();
        let stringified = structured.to_string();

        assert_eq!(example, stringified);
    }
}
