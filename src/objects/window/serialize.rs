use std::fmt::Display;

use super::Window;

impl Display for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCWINDOW({},{},{},{},{},{});",
            self.element,
            self.overall_height,
            self.overall_width,
            self.predefined_type,
            self.partitioning_type,
            self.user_defining_partitioning_type
        )
    }
}
