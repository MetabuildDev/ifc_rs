use std::fmt::Display;

use crate::objects::address::TelecomAddress;

impl Display for TelecomAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCTELECOMADDRESS({purpose},{description},{user_defined_purpose},{telephone_numbers},{facsimile_numbers},{pager_number},{email_addresses},{homepage_url});",
            purpose = self.purpose,
            description = self.description,
            user_defined_purpose = self.user_defined_purpose,
            telephone_numbers = self.telephone_numbers,
            facsimile_numbers = self.facsimile_numbers,
            pager_number = self.pager_number,
            email_addresses = self.email_addresses,
            homepage_url = self.homepage_url,
        )
    }
}
