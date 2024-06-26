use roof::Roof;
use slab::Slab;
use wall::Wall;
use window::Window;
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
pub mod roof;
pub mod rooftype;
pub mod shared;
pub mod site;
pub mod slab;
pub mod slabtype;
pub mod space;
pub mod spacetype;
pub mod storey;
pub mod wall;
pub mod walltype;
pub mod window;
pub mod windowtype;

pub enum StructureType<'a> {
    Wall(&'a Wall),
    Slab(&'a Slab),
    Roof(&'a Roof),
    Window(&'a Window),
}

pub trait Structure: IfcType {
    fn structure_type(&self) -> Option<StructureType<'_>> {
        None
    }
}

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
            roof::Roof::parse_any(),
            rooftype::RoofType::parse_any(),
            building::Building::parse_any(),
            storey::Storey::parse_any(),
            site::Site::parse_any(),
            opening_element::OpeningElement::parse_any(),
            window::Window::parse_any(),
            windowtype::WindowType::parse_any(),
            alt((
                project::Project::parse_any(),
                space::Space::parse_any(),
                spacetype::SpaceType::parse_any(),
            )),
        ))
    }
}
