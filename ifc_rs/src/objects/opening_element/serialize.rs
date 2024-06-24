
use std::fmt::Display;

use super::OpeningElement;

impl Display for OpeningElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IFCOPENINGELEMENT({},{});", self.element, self.predefined_type,)
    }
}
