mod deserialize;
mod serialize;

use crate::{
    ifc_type::IfcType,
    parser::ifc_float::{IfcDVec2, IfcDVec3},
};

/// The IfcDirection provides a direction in two or three dimensional space depending on the number
/// of DirectionRatio's provided. The IfcDirection does not imply a vector length, and the
/// direction ratios does not have to be normalized.
///
/// The components in the direction of X axis (DirectionRatios[1]), of Y axis (DirectionRatios[2])
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcdirection.htm
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Direction2D(IfcDVec2);

impl IfcType for Direction2D {}

/// The IfcDirection provides a direction in two or three dimensional space depending on the number
/// of DirectionRatio's provided. The IfcDirection does not imply a vector length, and the
/// direction ratios does not have to be normalized.
///
/// The components in the direction of X axis (DirectionRatios[1]), of Y axis (DirectionRatios[2]),
/// and of Z axis (DirectionRatios[3])
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcdirection.htm
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Direction3D(IfcDVec3);

impl IfcType for Direction3D {}
