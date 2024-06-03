use std::fmt::Display;

use crate::objects::person::Person;

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCPERSON({id},{family_name},{given_name},{middle_names},{prefix_titles},{suffix_titles},{roles},{addresses});",
            id = self.id,
            family_name = self.family_name,
            given_name = self.given_name,
            middle_names = self.middle_names,
            prefix_titles = self.prefix_titles,
            suffix_titles = self.suffix_titles,
            roles = self.roles,
            addresses = self.addresses,
        )
    }
}
