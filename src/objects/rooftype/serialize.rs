use std::fmt::Display;

use super::RoofType;

impl Display for RoofType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCROOFTYPE({},{});",
            self.element_type, self.predefined_type,
        )
    }
}
