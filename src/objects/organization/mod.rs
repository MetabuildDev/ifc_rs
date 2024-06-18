mod deserialize;
mod serialize;

use ifc_verify_derive::IfcVerify;

use crate::id::{Id, IdOr, TypedId};
use crate::ifc_type::{IfcType, IfcVerify};
use crate::parser::label::Label;
use crate::parser::list::IfcList;
use crate::parser::optional::OptionalParameter;
use crate::prelude::*;

use super::actor_role::ActorRole;
use super::address::Address;

/// A named and structured grouping with a corporate identity.
///
/// https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcactorresource/lexical/ifcorganization.htm
#[derive(Debug, Clone, IfcVerify)]
pub struct Organization {
    /// Identification of the organization.
    pub id: OptionalParameter<Label>,
    /// The word, or group of words, by which the organization is referred to.
    pub name: Label,
    /// Text that relates the nature of the organization.
    pub description: OptionalParameter<Label>, // TODO: Text
    /// Roles played by the organization.
    pub roles: OptionalParameter<IfcList<TypedId<ActorRole>>>,
    /// Postal and telecommunication addresses of an organization.
    #[ifc_types(TelecomAddress, PostalAddress)]
    pub addresses: OptionalParameter<IfcList<Id>>,
}

impl Organization {
    pub fn new<'a>(
        id: impl Into<Option<&'a str>>,
        name: impl Into<Label>,
        description: impl Into<Option<&'a str>>,
    ) -> Self {
        Self {
            id: id.into().map(|s| s.into()).into(),
            name: name.into(),
            description: description.into().map(|s| s.into()).into(),
            roles: OptionalParameter::omitted(),
            addresses: OptionalParameter::omitted(),
        }
    }

    pub fn add_role(mut self, role: impl Into<IdOr<ActorRole>>, ifc: &mut IFC) -> Self {
        if self.roles.is_omitted() {
            self.roles = IfcList::empty().into();
        }

        self.roles
            .custom_mut()
            .unwrap()
            .0
            .push(role.into().or_insert(ifc));

        self
    }

    pub fn add_address<A: Address>(mut self, address: impl Into<IdOr<A>>, ifc: &mut IFC) -> Self {
        if self.addresses.is_omitted() {
            self.addresses = IfcList::empty().into();
        }

        self.addresses
            .custom_mut()
            .unwrap()
            .0
            .push(address.into().or_insert(ifc).id());

        self
    }
}

impl IfcType for Organization {}
