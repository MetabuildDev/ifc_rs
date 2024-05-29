use std::fmt::Display;

use crate::helper::format_double;

use super::RectangleProfileDef;

impl Display for RectangleProfileDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCRECTANGLEPROFILEDEF({},$,{},{},{});",
            self.profile_type,
            self.position,
            format_double(self.x_dim),
            format_double(self.y_dim),
        )
    }
}
