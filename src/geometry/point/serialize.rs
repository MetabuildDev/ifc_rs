use std::fmt::Display;

use super::{Point2D, Point3D};
use crate::helper::format_double;

impl Display for Point2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCCARTESIANPOINT(({x},{y}));",
            x = format_double(self.0.x),
            y = format_double(self.0.y),
        )
    }
}

impl Display for Point3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCCARTESIANPOINT(({x},{y},{z}));",
            x = format_double(self.0.x),
            y = format_double(self.0.y),
            z = format_double(self.0.z),
        )
    }
}
