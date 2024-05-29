use std::fmt::Display;

use super::RectangleProfileDef;

impl Display for RectangleProfileDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCRECTANGLEPROFILEDEF({},{},{},{},{});",
            self.profile_type, self.profile_name, self.position, self.x_dim, self.y_dim,
        )
    }
}
