mod deserialize;
mod serialize;

use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use glam::{DVec2, DVec3};
use ifc_verify_derive::IfcVerify;

use crate::{
    ifc_type::{IfcType, IfcVerify},
    parser::ifc_float::{IfcDVec2, IfcDVec3},
    IFC,
};

pub enum PointType<'a> {
    D2(&'a Point2D),
    D3(&'a Point3D),
}

impl<'a> Display for PointType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PointType::D2(d2) => write!(f, "{d2}"),
            PointType::D3(d3) => write!(f, "{d3}"),
        }
    }
}

pub trait CartesianPoint: IfcType {}

/// An IfcCartesianPoint defines a point by coordinates in an orthogonal, right-handed Cartesian
/// coordinate system. For the purpose of this specification only two and three dimensional
/// Cartesian points are used.
///
/// The first and second coordinate of the point location. If placed in a two dimensional
/// rectangular Cartesian coordinate system, Coordinates[1] is the X coordinate and
/// Coordinates[2] is the Y coordinate.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifccartesianpoint.htm
#[derive(Debug, Clone, Copy, PartialEq, IfcVerify)]
pub struct Point2D(IfcDVec2);

impl Deref for Point2D {
    type Target = IfcDVec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Point2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<DVec2> for Point2D {
    fn from(value: DVec2) -> Self {
        Self(IfcDVec2(value))
    }
}

impl IfcType for Point2D {}
impl CartesianPoint for Point2D {}

/// An IfcCartesianPoint defines a point by coordinates in an orthogonal, right-handed Cartesian
/// coordinate system. For the purpose of this specification only two and three dimensional
/// Cartesian points are used.
///
/// The first, second, and third coordinate of the point location. If placed in a three
/// dimensional rectangular Cartesian coordinate system, Coordinates[1] is the X coordinate,
/// Coordinates[2] is the Y coordinate, and Coordinates[3] is the Z coordinate.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifccartesianpoint.htm
#[derive(Debug, Clone, Copy, PartialEq, IfcVerify)]
pub struct Point3D(IfcDVec3);

impl Deref for Point3D {
    type Target = IfcDVec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Point3D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<DVec3> for Point3D {
    fn from(value: DVec3) -> Self {
        Self(IfcDVec3(value))
    }
}

impl IfcType for Point3D {}
impl CartesianPoint for Point3D {}
