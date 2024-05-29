use std::fmt::Display;

use super::ExtrudedAreaSolid;

impl Display for ExtrudedAreaSolid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCEXTRUDEDAREASOLID({},{},{},{});",
            self.swept_area, self.position, self.extruded_direction, self.depth
        )
    }
}
