use std::fmt::Display;

use winnow::combinator::alt;

use crate::parser::{IFCParse, IFCParser};

pub mod actor_role;
pub mod address;
pub mod application;
pub mod organization;
pub mod person;
pub mod person_and_org;
pub mod shared;
pub mod wall;
pub mod walltype;

pub struct Objects;

impl Objects {
    pub fn parse<'a>() -> impl IFCParser<'a, Box<dyn Display>> {
        alt((
            actor_role::ActorRole::parse_any(),
            address::PostalAddress::parse_any(),
            address::TelecomAddress::parse_any(),
            application::Application::parse_any(),
            organization::Organization::parse_any(),
            person::Person::parse_any(),
            person_and_org::PersonAndOrganization::parse_any(),
            wall::Wall::parse_any(),
            walltype::WallType::parse_any(),
        ))
    }
}

impl Display for Objects {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unreachable!()
    }
}
