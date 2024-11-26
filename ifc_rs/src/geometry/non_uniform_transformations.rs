use std::{fmt::Display, ops::Deref};

use ifc_rs_verify_derive::IfcVerify;

use crate::{
    id::IdOr,
    parser::{
        comma::Comma, optional::OptionalParameter, p_space_or_comment_surrounded,
        real::RealPrimitive, IFCParse,
    },
    prelude::*,
};

use super::transform_base::{Transform3DBase, TransformBaseMapping};

#[derive(Debug, Clone, Copy)]
pub struct NonUniformTransformMapping<'a> {
    base: TransformBaseMapping<'a>,

    pub scale_y: Option<f64>,
    pub scale_z: Option<f64>,
}

impl<'a> Deref for NonUniformTransformMapping<'a> {
    type Target = TransformBaseMapping<'a>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

/// A Cartesian transformation operator 3d non uniform defines a geometric transformation in
/// three-dimensional space composed of translation, rotation, mirroring and non uniform scaling.
/// Non uniform scaling is given by three different scaling factors:
///
/// - Scale: the x axis scale factor - Scale2: the y axis scale factor - Scale3: the z axis scale
///   factor
///
/// If the Scale factor (at supertype IfcCartesianTransformationOperator) is omitted, it defaults
/// to 1.0. If the Scale2 or the Scale3 factor is omitted, it defaults to the value of Scale (the x
/// axis scale factor).
#[derive(IfcVerify, Debug)]
pub struct CartesianTransformationOperator3DnonUniform {
    #[inherited]
    base: Transform3DBase,
    /// The scaling value specified for the transformation along the axis 2. This is normally the y
    /// scale factor.
    pub scale_y: OptionalParameter<RealPrimitive>,
    /// The scaling value specified for the transformation along the axis 3. This is normally the z
    /// scale factor.
    pub scale_z: OptionalParameter<RealPrimitive>,
}

impl CartesianTransformationOperator3DnonUniform {
    pub fn new(
        local_origin: impl Into<IdOr<Point3D>>,
        (axis_x, axis_y, axis_z): (
            impl Into<IdOr<Direction3D>>,
            impl Into<IdOr<Direction3D>>,
            impl Into<IdOr<Direction3D>>,
        ),
        (scale, scale_y, scale_z): (f64, f64, f64),
        ifc: &mut IFC,
    ) -> Self {
        Self {
            base: Transform3DBase::new(local_origin, (axis_x, axis_y, axis_z), scale, ifc),
            scale_y: RealPrimitive(scale_y).into(),
            scale_z: RealPrimitive(scale_z).into(),
        }
    }
}

impl<'a> IfcMappedType<'a> for CartesianTransformationOperator3DnonUniform {
    type Target = NonUniformTransformMapping<'a>;

    fn mappings(&'a self, ifc: &'a IFC) -> Self::Target {
        NonUniformTransformMapping {
            base: self.base.mappings(ifc),

            scale_y: self.scale_y.custom().map(|f| f.0),
            scale_z: self.scale_z.custom().map(|f| f.0),
        }
    }
}

impl Deref for CartesianTransformationOperator3DnonUniform {
    type Target = Transform3DBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl IfcType for CartesianTransformationOperator3DnonUniform {}
impl Transform3D for CartesianTransformationOperator3DnonUniform {}

impl Display for CartesianTransformationOperator3DnonUniform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCCARTESIANTRANSFORMATIONOPERATOR3DNONUNIFORM({},{},{});",
            self.base, self.scale_y, self.scale_z
        )
    }
}

impl IFCParse for CartesianTransformationOperator3DnonUniform {
    fn parse<'a>() -> impl crate::parser::IFCParser<'a, Self>
    where
        Self: Sized,
    {
        winnow::seq! {
            CartesianTransformationOperator3DnonUniform {
                _: p_space_or_comment_surrounded("IFCCARTESIANTRANSFORMATIONOPERATOR3DNONUNIFORM("),
                base: Transform3DBase::parse(),
                _: Comma::parse(),
                scale_y: OptionalParameter::parse(),
                _: Comma::parse(),
                scale_z: OptionalParameter::parse(),
                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use crate::parser::IFCParse;

    use super::CartesianTransformationOperator3DnonUniform;

    #[test]
    fn non_uniform_cartesian_transformation_round_trip() {
        let example = "IFCCARTESIANTRANSFORMATIONOPERATOR3DNONUNIFORM(#30,#31,#15,0.5,#28,0.5,1.);";

        let structured = CartesianTransformationOperator3DnonUniform::parse()
            .parse(example)
            .unwrap();
        let stringified = structured.to_string();

        assert_eq!(example, stringified);
    }
}
