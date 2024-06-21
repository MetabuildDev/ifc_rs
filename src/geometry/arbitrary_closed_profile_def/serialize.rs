use std::fmt::Display;

use crate::geometry::arbitrary_closed_profile_def::ArbitraryClosedProfileDef;

impl Display for ArbitraryClosedProfileDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCARBITRARYCLOSEDPROFILEDEF({},{},{});",
            self.profile_type, self.profile_name, self.outer_curve,
        )
    }
}
