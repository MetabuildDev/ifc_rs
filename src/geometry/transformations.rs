use std::fmt::Display;

use crate::{
    id::IdOr,
    ifc_type::IfcType,
    parser::{comma::Comma, ifc_float::IfcFloat, p_space_or_comment_surrounded, IFCParse},
    prelude::Direction3D,
    IFC,
};

use super::point::Point3D;

/// A Cartesian transformation operator 3d non uniform defines a geometric transformation in
/// three-dimensional space composed of translation, rotation, mirroring and non uniform scaling.
/// Non uniform scaling is given by three different scaling factors:
///
/// - Scale: the x axis scale factor - Scale2: the y axis scale factor - Scale3: the z axis scale
/// factor
///
/// If the Scale factor (at supertype IfcCartesianTransformationOperator) is omitted, it defaults
/// to 1.0. If the Scale2 or the Scale3 factor is omitted, it defaults to the value of Scale (the x
/// axis scale factor).
pub struct CartesianTransformationOperator3DnonUniform {
    /// The direction used to determine U[1], the derived X axis direction.
    pub axis_x: IdOr<Direction3D>,
    /// The direction used to determine U[2], the derived Y axis direction.
    pub axis_y: IdOr<Direction3D>,
    /// The required translation, specified as a cartesian point. The actual translation included
    /// in the transformation is from the geometric origin to the local origin.
    pub local_origin: IdOr<Point3D>,
    /// The scaling value specified for the transformation.
    pub scale: IfcFloat,
    /// The exact direction of U[3], the derived Z axis direction.
    pub axis_z: IdOr<Direction3D>,
    /// The scaling value specified for the transformation along the axis 2. This is normally the y
    /// scale factor.
    pub scale_y: IfcFloat,
    /// The scaling value specified for the transformation along the axis 3. This is normally the z
    /// scale factor.
    pub scale_z: IfcFloat,
}

impl CartesianTransformationOperator3DnonUniform {
    pub fn new(
        axis_x: impl Into<IdOr<Direction3D>>,
        axis_y: impl Into<IdOr<Direction3D>>,
        local_origin: impl Into<IdOr<Point3D>>,
        scale: f64,
        axis_z: impl Into<IdOr<Direction3D>>,
        scale_y: f64,
        scale_z: f64,
        ifc: &mut IFC,
    ) -> Self {
        Self {
            axis_x: axis_x.into().into_id(ifc),
            axis_y: axis_y.into().into_id(ifc),
            local_origin: local_origin.into().into_id(ifc),
            scale: IfcFloat(scale),
            axis_z: axis_z.into().into_id(ifc),
            scale_y: IfcFloat(scale_y),
            scale_z: IfcFloat(scale_z),
        }
    }
}

impl IfcType for CartesianTransformationOperator3DnonUniform {}

impl Display for CartesianTransformationOperator3DnonUniform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCCARTESIANTRANSFORMATIONOPERATOR3DNONUNIFORM({},{},{},{},{},{},{});",
            self.axis_x,
            self.axis_y,
            self.local_origin,
            self.scale,
            self.axis_z,
            self.scale_y,
            self.scale_z
        )
    }
}

impl IFCParse for CartesianTransformationOperator3DnonUniform {
    fn parse<'a>() -> impl crate::parser::IFCParser<'a, Self>
    where
        Self: Sized,
    {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCCARTESIANTRANSFORMATIONOPERATOR3DNONUNIFORM("),
                axis_x: IdOr::parse(),
                _: Comma::parse(),
                axis_y: IdOr::parse(),
                _: Comma::parse(),
                local_origin: IdOr::parse(),
                _: Comma::parse(),
                scale: IfcFloat::parse(),
                _: Comma::parse(),
                axis_z: IdOr::parse(),
                _: Comma::parse(),
                scale_y: IfcFloat::parse(),
                _: Comma::parse(),
                scale_z: IfcFloat::parse(),
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
    fn cartesian_transformation_round_trip() {
        let example = "IFCCARTESIANTRANSFORMATIONOPERATOR3DNONUNIFORM(#30,#31,#15,0.5,#28,0.5,1.);";

        let structured = CartesianTransformationOperator3DnonUniform::parse()
            .parse(example)
            .unwrap();
        let stringified = structured.to_string();

        assert_eq!(example, stringified);
    }
}
