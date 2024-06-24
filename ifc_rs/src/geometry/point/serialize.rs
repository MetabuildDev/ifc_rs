use std::fmt::Display;

use super::{Point2D, Point3D};

impl Display for Point2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IFCCARTESIANPOINT({p});", p = self.0)
    }
}

impl Display for Point3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IFCCARTESIANPOINT({p});", p = self.0,)
    }
}
