use std::fmt::Display;

use super::SlabType;

impl Display for SlabType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCSLABTYPE({},{});",
            self.element_type, self.predefined_type,
        )
    }
}
