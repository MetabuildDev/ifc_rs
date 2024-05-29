mod deserialize;
mod serialize;

use glam::{DVec2, DVec3};

/// An IfcCartesianPoint defines a point by coordinates in an orthogonal, right-handed Cartesian
/// coordinate system. For the purpose of this specification only two and three dimensional
/// Cartesian points are used.
///
/// The first and second coordinate of the point location. If placed in a two dimensional
/// rectangular Cartesian coordinate system, Coordinates[1] is the X coordinate and
/// Coordinates[2] is the Y coordinate.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifccartesianpoint.htm
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D(DVec2);

/// An IfcCartesianPoint defines a point by coordinates in an orthogonal, right-handed Cartesian
/// coordinate system. For the purpose of this specification only two and three dimensional
/// Cartesian points are used.
///
/// The first, second, and third coordinate of the point location. If placed in a three
/// dimensional rectangular Cartesian coordinate system, Coordinates[1] is the X coordinate,
/// Coordinates[2] is the Y coordinate, and Coordinates[3] is the Z coordinate.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifccartesianpoint.htm
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D(DVec3);
