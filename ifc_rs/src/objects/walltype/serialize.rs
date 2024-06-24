use std::fmt::Display;

use super::WallType;

impl Display for WallType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCWALLTYPE({},{});",
            self.element_type, self.predefined_type,
        )
    }
}
