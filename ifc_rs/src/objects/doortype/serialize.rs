use std::fmt::Display;

use super::DoorType;

impl Display for DoorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCDOORTYPE({},{},{},{},{});",
            self.element_type,
            self.predefined_type,
            self.operation_type,
            self.parameter_takes_precedence,
            self.user_defined_partitioning_type
        )
    }
}
