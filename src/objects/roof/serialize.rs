use std::fmt::Display;

use super::Roof;

impl Display for Roof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IFCROOF({},{});", self.element, self.predefined_type,)
    }
}
