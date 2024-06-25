mod deserialize;
mod serialize;

use ifc_rs_verify_derive::IfcVerify;

use crate::{
    id::TypedId,
    ifc_type::{IfcType, IfcVerify},
    parser::optional::OptionalParameter,
    prelude::*,
};

pub enum AxisMappings<'a> {
    D2(MappedAxis2D<'a>),
    D3(MappedAxis3D<'a>),
}

impl<'a> AxisMappings<'a> {
    pub fn map_2d(axis: &'a Axis2D, ifc: &'a IFC) -> Self {
        Self::D2(axis.mappings(ifc))
    }

    pub fn map_3d(axis: &'a Axis3D, ifc: &'a IFC) -> Self {
        Self::D3(axis.mappings(ifc))
    }
}

pub trait AxisPlacement: IfcType {}

pub struct MappedAxis2D<'a> {
    pub location: &'a Point2D,
    pub local_x: Option<&'a Direction2D>,
}

/// The IfcAxis2Placement2D provides location and orientation to place items in a two-dimensional
/// space. The attribute RefDirection defines the x axis, the y axis is derived.
///
/// If the attribute RefDirection is not given, the placement defaults to P[1] (x-axis) as [1.,0.]
/// and P[2] (y-axis) as [0.,1.].
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcaxis2placement2d.htm
#[derive(Debug, Clone, IfcVerify)]
pub struct Axis2D {
    /// The geometric position of a reference point, such as the center of a circle, of the item to
    /// be located.
    pub location: TypedId<Point2D>,
    /// The direction used to determine the direction of the local X axis. If a value is omited
    /// that it defaults to [1.0, 0.0].
    pub local_x: OptionalParameter<TypedId<Direction2D>>,
}

impl Axis2D {
    pub fn new(point: Point2D, ifc: &mut IFC) -> Self {
        let id = ifc.data.insert_new(point);

        Self {
            location: id,
            local_x: OptionalParameter::omitted(),
        }
    }

    pub fn mappings<'a>(&self, ifc: &'a IFC) -> MappedAxis2D<'a> {
        MappedAxis2D {
            location: ifc.data.get(self.location),
            local_x: self.local_x.custom().map(|id| ifc.data.get(*id)),
        }
    }
}

impl IfcType for Axis2D {}
impl AxisPlacement for Axis2D {}

#[derive(Debug)]
pub struct MappedAxis3D<'a> {
    pub location: &'a Point3D,
    pub local_z: Option<&'a Direction3D>,
    pub local_x: Option<&'a Direction3D>,
}

/// The IfcAxis2Placement3D provides location and orientations to place items in a
/// three-dimensional space. The attribute Axis defines the Z direction, RefDirection the X
/// direction. The Y direction is derived.
///
/// If the attribute values for Axis and RefDirection are not given, the placement defaults to P[1]
/// (x-axis) as [1.,0.,0.], P[2] (y-axis) as [0.,1.,0.] and P[3] (z-axis) as [0.,0.,1.].
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcaxis2placement3d.htm
#[derive(Debug, Clone, IfcVerify)]
pub struct Axis3D {
    /// The geometric position of a reference point, such as the center of a circle, of the item to
    /// be located.
    pub location: TypedId<Point3D>,
    /// The exact direction of the local Z Axis. If a value is omited that it defaults to [0.0, 0.0, 0.1]
    pub local_z: OptionalParameter<TypedId<Direction3D>>,
    /// The direction used to determine the direction of the local X Axis. If necessary an
    /// adjustment is made to maintain orthogonality to the Axis direction. If Axis and/or
    /// RefDirection is omitted, these directions are taken from the geometric coordinate system.
    pub local_x: OptionalParameter<TypedId<Direction3D>>,
}

impl Axis3D {
    pub fn new(point: Point3D, ifc: &mut IFC) -> Self {
        let id = ifc.data.insert_new(point);

        Self {
            location: id,
            local_z: OptionalParameter::omitted(),
            local_x: OptionalParameter::omitted(),
        }
    }

    pub fn mappings<'a>(&self, ifc: &'a IFC) -> MappedAxis3D<'a> {
        MappedAxis3D {
            location: ifc.data.get(self.location),
            local_z: self.local_z.custom().map(|id| ifc.data.get(*id)),
            local_x: self.local_x.custom().map(|id| ifc.data.get(*id)),
        }
    }
}

impl IfcType for Axis3D {}
impl AxisPlacement for Axis3D {}
