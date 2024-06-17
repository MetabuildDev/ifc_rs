mod deserialize;
mod serialize;

use ifc_type_derive::IfcVerify;

use crate::id::{Id, IdOr};
use crate::ifc_type::{IfcType, IfcVerify};
use crate::parser::label::Label;
use crate::parser::list::IfcList;
use crate::parser::optional::OptionalParameter;
use crate::IFC;

use super::actor_role::ActorRole;
use super::address::Address;

/// An individual human being.
///
/// https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcactorresource/lexical/ifcperson.htm
#[derive(Debug, Clone, IfcVerify)]
pub struct Person {
    /// Identification of the person.
    pub id: OptionalParameter<Label>,
    /// The name by which the family identity of the person may be recognized.
    pub family_name: OptionalParameter<Label>,
    /// The name by which a person is known within a family and by which he or
    /// she may be familiarly recognized.
    pub given_name: OptionalParameter<Label>,
    /// Additional names given to a person that enable their identification
    /// apart from others who may have the same or similar family and given names.
    pub middle_names: OptionalParameter<IfcList<Label>>,
    /// The word, or group of words, which specify the person's social and/or
    /// professional standing and appear before his/her names.
    pub prefix_titles: OptionalParameter<IfcList<Label>>,
    /// The word, or group of words, which specify the person's social
    /// and/or professional standing and appear after his/her names.
    pub suffix_titles: OptionalParameter<IfcList<Label>>,
    /// Roles played by the person.
    pub roles: OptionalParameter<IfcList<Id>>,
    /// Postal and telecommunication addresses of a person.
    pub addresses: OptionalParameter<IfcList<Id>>,
}

impl Person {
    pub fn empty() -> Self {
        Self {
            id: OptionalParameter::omitted(),
            family_name: OptionalParameter::omitted(),
            given_name: OptionalParameter::omitted(),
            middle_names: OptionalParameter::omitted(),
            prefix_titles: OptionalParameter::omitted(),
            suffix_titles: OptionalParameter::omitted(),
            roles: OptionalParameter::omitted(),
            addresses: OptionalParameter::omitted(),
        }
    }

    pub fn id(mut self, id: impl Into<Label>) -> Self {
        self.id = id.into().into();
        self
    }

    pub fn family_name(mut self, family_name: impl Into<Label>) -> Self {
        self.family_name = family_name.into().into();
        self
    }

    pub fn given_name(mut self, given_name: impl Into<Label>) -> Self {
        self.given_name = given_name.into().into();
        self
    }

    pub fn add_middle_name(mut self, middle_name: impl Into<Label>) -> Self {
        if self.middle_names.is_omitted() {
            self.middle_names = OptionalParameter::Custom(IfcList::empty());
        }

        self.middle_names
            .custom_mut()
            .unwrap()
            .0
            .push(middle_name.into());

        self
    }

    pub fn add_suffix_title(mut self, suffix_title: impl Into<Label>) -> Self {
        if self.suffix_titles.is_omitted() {
            self.suffix_titles = OptionalParameter::Custom(IfcList::empty());
        }

        self.suffix_titles
            .custom_mut()
            .unwrap()
            .0
            .push(suffix_title.into());

        self
    }

    pub fn add_prefix_title(mut self, prefix_title: impl Into<Label>) -> Self {
        if self.prefix_titles.is_omitted() {
            self.prefix_titles = OptionalParameter::Custom(IfcList::empty());
        }

        self.prefix_titles
            .custom_mut()
            .unwrap()
            .0
            .push(prefix_title.into());

        self
    }

    pub fn add_role(mut self, role: impl Into<IdOr<ActorRole>>, ifc: &mut IFC) -> Self {
        if self.roles.is_omitted() {
            self.roles = IfcList::empty().into();
        }

        self.roles
            .custom_mut()
            .unwrap()
            .0
            .push(role.into().or_insert(ifc).id());

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

impl IfcType for Person {}
