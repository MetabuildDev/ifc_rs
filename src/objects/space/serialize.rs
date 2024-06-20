use std::fmt::Display;

use super::Space;

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCSPACE({},{},{});",
            self.spatial_element_structure, self.predefined_type, self.elevation_with_flooring
        )
    }
}
