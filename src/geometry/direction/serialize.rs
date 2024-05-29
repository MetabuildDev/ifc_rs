use std::fmt::Display;

use super::{Direction2D, Direction3D};

impl Display for Direction2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IFCDIRECTION({p});", p = self.0)
    }
}

impl Display for Direction3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IFCDIRECTION({p});", p = self.0)
    }
}
