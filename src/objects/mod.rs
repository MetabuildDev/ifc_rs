use winnow::combinator::alt;

use crate::{
    ifc_type::IfcType,
    parser::{IFCParse, IFCParser},
};

pub mod access_state;
pub mod actor_role;
pub mod address;
pub mod application;
pub mod building;
pub mod change_action;
pub mod opening_element;
pub mod organization;
pub mod owner_history;
pub mod person;
pub mod person_and_org;
pub mod prelude;
pub mod project;
pub mod shared;
pub mod slab;
pub mod slabtype;
pub mod wall;
pub mod walltype;
pub mod window;
pub mod windowtype;

pub trait Structure: IfcType {}

pub struct Objects;

impl Objects {
    pub fn parse<'a>() -> impl IFCParser<'a, Box<dyn IfcType>> {
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
            slab::Slab::parse_any(),
            slabtype::SlabType::parse_any(),
            building::Building::parse_any(),
            opening_element::OpeningElement::parse_any(),
            window::Window::parse_any(),
            windowtype::WindowType::parse_any(),
            project::Project::parse_any(),
        ))
    }
}
