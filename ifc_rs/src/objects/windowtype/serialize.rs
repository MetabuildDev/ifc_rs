use std::fmt::Display;

use super::WindowType;

impl Display for WindowType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCWINDOWTYPE({},{},{},{},{});",
            self.element_type,
            self.predefined_type,
            self.partitioning_type,
            self.parameter_takes_precedence,
            self.user_defined_partitioning_type
        )
    }
}
