use std::fmt::Display;

use super::Slab;

impl Display for Slab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IFCSLAB({},{});", self.element, self.predefined_type,)
    }
}
