use winnow::{combinator::delimited, Parser};

use crate::{
    objects::organization::Organization,
    parser::{comma::Comma, label::Label, optional::OptionalParameter, IFCParse, IFCParser},
};

impl IFCParse for Organization {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        delimited(
            "IFCORGANIZATION(",
            (
                OptionalParameter::parse(),
                Comma::parse(),
                Label::parse(),
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
            |(id, _, name, _, description, _, roles, _, addresses)| Self {
                id,
                name,
                description,
                roles,
                addresses,
            },
        )
    }
}

#[test]
fn parse_organization_works() {
    let data = "IFCORGANIZATION($,'Geometry Gym Pty Ltd',$,$,$);";
    let parsed = Organization::parse().parse(data).unwrap();
    assert_eq!(data, parsed.to_string());
}
