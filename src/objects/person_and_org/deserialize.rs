use winnow::{combinator::delimited, Parser};

use crate::{
    id::Id,
    objects::person_and_org::PersonAndOrganization,
    parser::{
        comma::Comma,
        optional::{IFCParse, OptionalParameter},
        IFCParser,
    },
};

impl PersonAndOrganization {
    pub fn parse<'a>() -> impl IFCParser<'a, Self> {
        delimited(
            "IFCPERSONANDORGANIZATION(",
            (
                Id::parse(),
                Comma::parse(),
                Id::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
            ),
            ");",
        )
        .map(|(the_person, _, the_organization, _, roles)| Self {
            the_person,
            the_organization,
            roles,
        })
    }
}

#[test]
fn parse_person_and_org_works() {
    let data = "IFCPERSONANDORGANIZATION(#7,#8,$);";
    let parsed = PersonAndOrganization::parse().parse(data).unwrap();
    assert_eq!(data, parsed.to_string());
}
