use std::fmt::Display;

use super::AreaUnit;

impl Display for AreaUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCSIUNIT($,.AREAUNIT.,{},{});",
            self.prefix
                .map(|p| p.to_string())
                .unwrap_or("$".to_string()),
            self.name
        )
    }
}
