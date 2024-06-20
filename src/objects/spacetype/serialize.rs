use std::fmt::Display;

use super::SpaceType;

impl Display for SpaceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCSPACETYPE({},{},{});",
            self.element_type, self.predefined_type, self.long_name
        )
    }
}
