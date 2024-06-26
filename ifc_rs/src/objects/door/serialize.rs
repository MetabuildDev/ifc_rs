use std::fmt::Display;

use super::Door;

impl Display for Door {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCDOOR({},{},{},{},{},{});",
            self.element,
            self.overall_height,
            self.overall_width,
            self.predefined_type,
            self.operation_type,
            self.user_defining_operation_type
        )
    }
}
