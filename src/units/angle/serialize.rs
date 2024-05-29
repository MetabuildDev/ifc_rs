use std::fmt::Display;

use super::AngleUnit;

impl Display for AngleUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCCONVERSIONBASEDUNIT(#{},.PLANEANGLEUNIT.,{},{});",
            self.dimensional_exponents_id, self.parameter_1, self.parameter_2
        )
    }
}
