use winnow::{combinator::delimited, Parser};

use crate::{
    objects::{change_action::ChangeAction, owner_history::OwnerHistory},
    parser::{
        comma::Comma, optional::OptionalParameter, timestamp::IfcTimestamp, IFCParse, IFCParser,
    },
};

impl IFCParse for OwnerHistory {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        delimited(
            "IFCOWNERHISTORY(",
            (
                OptionalParameter::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
                Comma::parse(),
                ChangeAction::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
                Comma::parse(),
                IfcTimestamp::parse(),
            ),
            ");",
        )
        .map(
            |(
                owning_user,
                _,
                owning_application,
                _,
                state,
                _,
                change_action,
                _,
                last_modified_date,
                _,
                last_modifying_user,
                _,
                last_modifying_application,
                _,
                creation_date,
            )| Self {
                owning_user,
                owning_application,
                state,
                change_action,
                last_modified_date,
                last_modifying_user,
                last_modifying_application,
                creation_date,
            },
        )
    }
}

#[test]
fn parse_owner_history_works() {
    let data = "IFCOWNERHISTORY(#5,#6,$,.ADDED.,1454575675,$,$,1454575675);";
    let parsed = OwnerHistory::parse().parse(data).unwrap();
    assert_eq!(data, parsed.to_string());
}
