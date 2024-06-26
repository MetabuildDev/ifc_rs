use door::Door;
use roof::Roof;
use slab::Slab;
use wall::Wall;
use window::Window;
use winnow::combinator::alt;

use crate::{
    parser::{IFCParse, IFCParser},
    prelude::*,
};

pub(crate) mod access_state;
pub(crate) mod actor_role;
pub(crate) mod address;
pub(crate) mod application;
pub(crate) mod building;
pub(crate) mod change_action;
pub(crate) mod door;
pub(crate) mod doortype;
pub(crate) mod opening_element;
pub(crate) mod organization;
pub(crate) mod owner_history;
pub(crate) mod person;
pub(crate) mod person_and_org;
pub(crate) mod prelude;
pub(crate) mod project;
pub(crate) mod roof;
pub(crate) mod rooftype;
pub(crate) mod shared;
pub(crate) mod site;
pub(crate) mod slab;
pub(crate) mod slabtype;
pub(crate) mod space;
pub(crate) mod spacetype;
pub(crate) mod storey;
pub(crate) mod wall;
pub(crate) mod walltype;
pub(crate) mod window;
pub(crate) mod windowtype;

pub enum StructureType<'a> {
    Wall(&'a Wall),
    Slab(&'a Slab),
    Roof(&'a Roof),
    Window(&'a Window),
    Door(&'a Door),
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
                door::Door::parse_any(),
                doortype::DoorType::parse_any(),
            )),
        ))
    }
}
