use std::fmt::Display;

use crate::objects::address::{PostalAddress, TelecomAddress};

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

impl Display for PostalAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCPOSTALADDRESS({purpose},{description},{user_defined_purpose},{internal_location},{address_lines},{postal_box},{town},{region},{postal_code},{country});",
            purpose = self.purpose,
            description = self.description,
            user_defined_purpose = self.user_defined_purpose,
            internal_location = self.internal_location,
            address_lines = self.address_lines,
            postal_box = self.postal_box,
            town = self.town,
            region = self.region,
            postal_code = self.postal_code,
            country = self.country,
        )
    }
}
