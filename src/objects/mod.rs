use std::fmt::Display;

use winnow::combinator::alt;

use crate::parser::{IFCParse, IFCParser};

pub mod access_state;
pub mod actor_role;
pub mod address;
pub mod application;
pub mod building;
pub mod change_action;
pub mod organization;
pub mod owner_history;
pub mod person;
pub mod person_and_org;
pub mod project;
pub mod rel_aggregates;
pub mod rel_associates_material;
pub mod rel_contained_in_spatial_structure;
pub mod rel_declares;
pub mod rel_defines_by_type;
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
            owner_history::OwnerHistory::parse_any(),
            person::Person::parse_any(),
            person_and_org::PersonAndOrganization::parse_any(),
            wall::Wall::parse_any(),
            walltype::WallType::parse_any(),
            rel_aggregates::RelAggregates::parse_any(),
            building::Building::parse_any(),
            rel_associates_material::RelAssociatesMaterial::parse_any(),
            rel_contained_in_spatial_structure::RelContainedInSpatialStructure::parse_any(),
            project::Project::parse_any(),
            rel_declares::RelDeclares::parse_any(),
            rel_defines_by_type::RelDefinesByType::parse_any(),
        ))
    }
}
