use std::fmt::Display;

use crate::objects::owner_history::OwnerHistory;

impl Display for OwnerHistory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCOWNERHISTORY({owning_user},{owning_application},{state},{change_action},{last_modified_date},{last_modifying_user},{last_modifying_application},{creation_date});",
            owning_user = self.owning_user,
            owning_application = self.owning_application,
            state = self.state,
            change_action = self.change_action,
            last_modified_date = self.last_modified_date,
            last_modifying_user = self.last_modifying_user,
            last_modifying_application = self.last_modifying_application,
            creation_date = self.creation_date,
        )
    }
}
