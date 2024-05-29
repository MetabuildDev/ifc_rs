mod deserialize;
mod serialize;

use glam::{DVec2, DVec3};

/// The IfcDirection provides a direction in two or three dimensional space depending on the number
/// of DirectionRatio's provided. The IfcDirection does not imply a vector length, and the
/// direction ratios does not have to be normalized.
///
/// The components in the direction of X axis (DirectionRatios[1]), of Y axis (DirectionRatios[2])
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcdirection.htm
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Direction2D(DVec2);

/// The IfcDirection provides a direction in two or three dimensional space depending on the number
/// of DirectionRatio's provided. The IfcDirection does not imply a vector length, and the
/// direction ratios does not have to be normalized.
///
/// The components in the direction of X axis (DirectionRatios[1]), of Y axis (DirectionRatios[2]),
/// and of Z axis (DirectionRatios[3])
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcdirection.htm
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Direction3D(DVec3);
