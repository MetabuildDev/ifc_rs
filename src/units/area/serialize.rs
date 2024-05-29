use std::fmt::Display;

use super::AreaUnit;

impl Display for AreaUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IFCSIUNIT($,.AREAUNIT.,{},{});", self.prefix, self.name)
    }
}
