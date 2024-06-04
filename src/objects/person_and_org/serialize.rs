use std::fmt::Display;

use crate::objects::person_and_org::PersonAndOrganization;

impl Display for PersonAndOrganization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCPERSONANDORGANIZATION({the_person},{the_organization},{roles});",
            the_person = self.the_person,
            the_organization = self.the_organization,
            roles = self.roles,
        )
    }
}
