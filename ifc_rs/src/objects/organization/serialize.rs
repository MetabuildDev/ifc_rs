use std::fmt::Display;

use crate::objects::organization::Organization;

impl Display for Organization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCORGANIZATION({id},{name},{description},{roles},{addresses});",
            id = self.id,
            name = self.name,
            description = self.description,
            roles = self.roles,
            addresses = self.addresses,
        )
    }
}
