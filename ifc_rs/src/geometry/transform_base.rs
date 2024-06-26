use std::fmt::Display;

use ifc_rs_verify_derive::IfcVerify;

use crate::{
    id::IdOr,
    parser::{comma::Comma, ifc_float::IfcFloat, optional::OptionalParameter, IFCParse},
    prelude::*,
};

pub trait Transform3D: IfcType {}

pub struct TransformBaseMapping<'a> {
    pub translation: Option<&'a Point3D>,

    pub axis_x: Option<&'a Direction3D>,
    pub axis_y: Option<&'a Direction3D>,
    pub axis_z: Option<&'a Direction3D>,

    pub scale: Option<f64>,
}

#[derive(IfcVerify)]
pub struct Transform3DBase {
    /// The direction used to determine U[1], the derived X axis direction.
    pub axis_x: OptionalParameter<IdOr<Direction3D>>,
    /// The direction used to determine U[2], the derived Y axis direction.
    pub axis_y: OptionalParameter<IdOr<Direction3D>>,
    /// The required translation, specified as a cartesian point. The actual translation included
    /// in the transformation is from the geometric origin to the local origin.
    pub local_origin: OptionalParameter<IdOr<Point3D>>,
    /// The scaling value specified for the transformation.
    pub scale: OptionalParameter<IfcFloat>,
    /// The exact direction of U[3], the derived Z axis direction.
    pub axis_z: OptionalParameter<IdOr<Direction3D>>,
}

impl Transform3DBase {
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
            axis_x: IdOr::Id(axis_x.into().or_insert(ifc)).into(),
            axis_y: IdOr::Id(axis_y.into().or_insert(ifc)).into(),
            local_origin: IdOr::Id(local_origin.into().or_insert(ifc)).into(),
            scale: IfcFloat(scale).into(),
            axis_z: IdOr::Id(axis_z.into().or_insert(ifc)).into(),
        }
    }
}

impl<'a> IfcMappedType<'a> for Transform3DBase {
    type Target = TransformBaseMapping<'a>;

    fn mappings(&'a self, ifc: &'a IFC) -> Self::Target {
        TransformBaseMapping {
            translation: self.local_origin.custom().map(|c| match c {
                IdOr::Id(id) => ifc.data.get(*id),
                IdOr::Custom(custom) => custom,
            }),

            axis_x: self.axis_x.custom().map(|c| match c {
                IdOr::Id(id) => ifc.data.get(*id),
                IdOr::Custom(custom) => custom,
            }),

            axis_y: self.axis_y.custom().map(|c| match c {
                IdOr::Id(id) => ifc.data.get(*id),
                IdOr::Custom(custom) => custom,
            }),

            axis_z: self.axis_z.custom().map(|c| match c {
                IdOr::Id(id) => ifc.data.get(*id),
                IdOr::Custom(custom) => custom,
            }),

            scale: self.scale.custom().map(|f| f.0),
        }
    }
}

impl IFCParse for Transform3DBase {
    fn parse<'a>() -> impl crate::parser::IFCParser<'a, Self>
    where
        Self: Sized,
    {
        winnow::seq! {
            Self {
                axis_x: OptionalParameter::parse(),
                _: Comma::parse(),
                axis_y: OptionalParameter::parse(),
                _: Comma::parse(),
                local_origin: OptionalParameter::parse(),
                _: Comma::parse(),
                scale: OptionalParameter::parse(),
                _: Comma::parse(),
                axis_z: OptionalParameter::parse(),
            }
        }
    }
}

impl Display for Transform3DBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{},{},{}",
            self.axis_x, self.axis_y, self.local_origin, self.scale, self.axis_z,
        )
    }
}
