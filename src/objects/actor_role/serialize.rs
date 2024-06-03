use std::fmt::Display;

use crate::objects::actor_role::ActorRole;

impl Display for ActorRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCACTORROLE({role},{user_defined_role},{description});",
            role = self.role,
            user_defined_role = self.user_defined_role,
            description = self.description,
        )
    }
}
