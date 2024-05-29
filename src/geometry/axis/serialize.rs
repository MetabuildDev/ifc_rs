use std::fmt::Display;

use super::{Axis2D, Axis3D};

impl Display for Axis2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCAXIS2PLACEMENT2D(#{loc_id}, {x_axis_id});",
            loc_id = self.location,
            x_axis_id = self.local_x
        )
    }
}

impl Display for Axis3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCAXIS2PLACEMENT3D(#{loc_id}, {z_axis_id}, {x_axis_id});",
            loc_id = self.location,
            z_axis_id = self.local_z,
            x_axis_id = self.local_x
        )
    }
}
