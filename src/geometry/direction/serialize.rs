use std::fmt::Display;

use super::{Direction2D, Direction3D};
use crate::helper::format_double;

impl Display for Direction2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCDIRECTION(({x},{y}));",
            x = format_double(self.0.x),
            y = format_double(self.0.y),
        )
    }
}

impl Display for Direction3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCDIRECTION(({x},{y},{z}));",
            x = format_double(self.0.x),
            y = format_double(self.0.y),
            z = format_double(self.0.z),
        )
    }
}
