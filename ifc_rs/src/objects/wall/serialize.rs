use std::fmt::Display;

use super::Wall;

impl Display for Wall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IFCWALL({},{});", self.element, self.predefined_type,)
    }
}
