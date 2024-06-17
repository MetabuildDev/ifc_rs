mod deserialize;
mod serialize;

use glam::{DVec2, DVec3};
use ifc_type_derive::IfcVerify;

use crate::{
    ifc_type::{IfcType, IfcVerify},
    parser::ifc_float::{IfcDVec2, IfcDVec3},
    IFC,
};

/// The IfcDirection provides a direction in two or three dimensional space depending on the number
/// of DirectionRatio's provided. The IfcDirection does not imply a vector length, and the
/// direction ratios does not have to be normalized.
///
/// The components in the direction of X axis (DirectionRatios[1]), of Y axis (DirectionRatios[2])
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcdirection.htm
#[derive(Debug, Clone, Copy, PartialEq, IfcVerify)]
pub struct Direction2D(IfcDVec2);

impl From<DVec2> for Direction2D {
    fn from(value: DVec2) -> Self {
        Self(IfcDVec2(value))
    }
}

impl IfcType for Direction2D {}

/// The IfcDirection provides a direction in two or three dimensional space depending on the number
/// of DirectionRatio's provided. The IfcDirection does not imply a vector length, and the
/// direction ratios does not have to be normalized.
///
/// The components in the direction of X axis (DirectionRatios[1]), of Y axis (DirectionRatios[2]),
/// and of Z axis (DirectionRatios[3])
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcdirection.htm
#[derive(Debug, Clone, Copy, PartialEq, IfcVerify)]
pub struct Direction3D(IfcDVec3);

impl From<DVec3> for Direction3D {
    fn from(value: DVec3) -> Self {
        Self(IfcDVec3(value))
    }
}

impl IfcType for Direction3D {}
