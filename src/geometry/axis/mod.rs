mod deserialize;
mod serialize;

use crate::{id::Id, parser::optional::OptionalParameter};

/// The IfcAxis2Placement2D provides location and orientation to place items in a two-dimensional
/// space. The attribute RefDirection defines the x axis, the y axis is derived.
///
/// If the attribute RefDirection is not given, the placement defaults to P[1] (x-axis) as [1.,0.]
/// and P[2] (y-axis) as [0.,1.].
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcaxis2placement2d.htm
#[derive(Debug, Clone)]
pub struct Axis2D {
    /// The geometric position of a reference point, such as the center of a circle, of the item to
    /// be located.
    pub location: Id,
    ///	The direction used to determine the direction of the local X axis. If a value is omited
    ///	that it defaults to [1.0, 0.0].
    pub local_x: OptionalParameter<Id>,
}

/// The IfcAxis2Placement3D provides location and orientations to place items in a
/// three-dimensional space. The attribute Axis defines the Z direction, RefDirection the X
/// direction. The Y direction is derived.
///
/// If the attribute values for Axis and RefDirection are not given, the placement defaults to P[1]
/// (x-axis) as [1.,0.,0.], P[2] (y-axis) as [0.,1.,0.] and P[3] (z-axis) as [0.,0.,1.].
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcaxis2placement3d.htm
#[derive(Debug, Clone)]
pub struct Axis3D {
    /// The geometric position of a reference point, such as the center of a circle, of the item to
    /// be located.
    pub location: Id,
    ///	The exact direction of the local Z Axis. If a value is omited that it defaults to [0.0, 0.0, 0.1]
    pub local_z: OptionalParameter<Id>,
    /// The direction used to determine the direction of the local X Axis. If necessary an
    /// adjustment is made to maintain orthogonality to the Axis direction. If Axis and/or
    /// RefDirection is omitted, these directions are taken from the geometric coordinate system.
    pub local_x: OptionalParameter<Id>,
}