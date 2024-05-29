use std::fmt::Display;

use super::PolyLine;

impl Display for PolyLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCPOLYLINE(({}));",
            self.points
                .iter()
                .map(|p| format!("{p}"))
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
