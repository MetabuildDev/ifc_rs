use winnow::{combinator::delimited, Parser};

use crate::{
    objects::address::{PostalAddress, TelecomAddress},
    parser::{comma::Comma, optional::OptionalParameter, IFCParse, IFCParser},
};

impl TelecomAddress {
    pub fn parse<'a>() -> impl IFCParser<'a, Self> {
        delimited(
            "IFCTELECOMADDRESS(",
            (
                OptionalParameter::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
            ),
            ");",
        )
        .map(
            |(
                purpose,
                _,
                description,
                _,
                user_defined_purpose,
                _,
                telephone_numbers,
                _,
                facsimile_numbers,
                _,
                pager_number,
                _,
                email_addresses,
                _,
                homepage_url,
            )| Self {
                purpose,
                description,
                user_defined_purpose,
                telephone_numbers,
                facsimile_numbers,
                pager_number,
                email_addresses,
                homepage_url,
            },
        )
    }
}

impl PostalAddress {
    pub fn parse<'a>() -> impl IFCParser<'a, Self> {
        delimited(
            "IFCPOSTALADDRESS(",
            (
                OptionalParameter::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
            ),
            ");",
        )
        .map(
            |(
                purpose,
                _,
                description,
                _,
                user_defined_purpose,
                _,
                internal_location,
                _,
                address_lines,
                _,
                postal_box,
                _,
                town,
                _,
                region,
                _,
                postal_code,
                _,
                country,
            )| Self {
                purpose,
                description,
                user_defined_purpose,
                internal_location,
                address_lines,
                postal_box,
                town,
                region,
                postal_code,
                country,
            },
        )
    }
}

#[test]
fn parse_telecom_address_works() {
    let data = "IFCTELECOMADDRESS($,$,$,$,$,$,('23022.debeka@rkwmail.de'),$);";
    let parsed = TelecomAddress::parse().parse(data).unwrap();
    assert_eq!(data, parsed.to_string());
}

#[test]
fn parse_postal_address_works() {
    let data = "IFCPOSTALADDRESS($,$,$,$,$,$,$,$,'10977','Germany');";
    let parsed = PostalAddress::parse().parse(data).unwrap();
    assert_eq!(data, parsed.to_string());
}
