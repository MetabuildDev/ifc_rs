use winnow::{combinator::delimited, Parser};

use crate::{
    objects::actor_role::{ActorRole, Role},
    parser::{
        comma::Comma,
        optional::{IFCParse, OptionalParameter},
        IFCParser,
    },
};

impl ActorRole {
    pub fn parse<'a>() -> impl IFCParser<'a, Self> {
        delimited(
            "IFCACTORROLE(",
            (
                Role::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
            ),
            ");",
        )
        .map(|(role, _, user_defined_role, _, description)| Self {
            role,
            user_defined_role,
            description,
        })
    }
}

#[test]
fn parse_actor_role_works() {
    let data = "IFCACTORROLE(.FIELDCONSTRUCTIONMANAGER.,$,$);";
    let parsed = ActorRole::parse().parse(data).unwrap();
    assert_eq!(data, parsed.to_string());

    let data = "IFCACTORROLE(.NOTAVALIDROLE.,$,$);";
    let parsed = ActorRole::parse().parse(data);
    assert!(parsed.is_err());
}
