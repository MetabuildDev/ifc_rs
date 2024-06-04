use winnow::{combinator::delimited, Parser};

use crate::{
    objects::person::Person,
    parser::{
        comma::Comma,
        optional::{IFCParse, OptionalParameter},
        IFCParser,
    },
};

impl IFCParse for Person {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        delimited(
            "IFCPERSON(",
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
                id,
                _,
                family_name,
                _,
                given_name,
                _,
                middle_names,
                _,
                prefix_titles,
                _,
                suffix_titles,
                _,
                roles,
                _,
                addresses,
            )| Self {
                id,
                family_name,
                given_name,
                middle_names,
                prefix_titles,
                suffix_titles,
                roles,
                addresses,
            },
        )
    }
}

#[test]
fn parse_person_works() {
    let data = "IFCPERSON($,'','hannah.schmitz',$,$,$,(#30),(#35,#36));";
    let parsed = Person::parse().parse(data).unwrap();
    assert_eq!(data, parsed.to_string());
}
