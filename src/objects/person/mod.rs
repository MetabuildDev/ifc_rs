mod deserialize;
mod serialize;

use crate::id::{Id, IdOr};
use crate::ifc_type::IfcType;
use crate::parser::label::Label;
use crate::parser::list::IfcList;
use crate::parser::optional::OptionalParameter;
use crate::IFC;

use super::actor_role::ActorRole;
use super::address::Address;

/// An individual human being.
///
/// https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcactorresource/lexical/ifcperson.htm
#[derive(Debug, Clone)]
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
    pub fn new<'a>(
        id: impl Into<Option<&'a str>>,
        family_name: impl Into<Option<&'a str>>,
        given_name: impl Into<Option<&'a str>>,
        middle_names: impl Into<Option<&'a [&'a str]>>,
        prefix_titles: impl Into<Option<&'a [&'a str]>>,
        suffix_titles: impl Into<Option<&'a [&'a str]>>,
    ) -> Self {
        Self {
            id: id.into().map(|s| s.into()).into(),
            family_name: family_name.into().map(|s| s.into()).into(),
            given_name: given_name.into().map(|s| s.into()).into(),
            middle_names: middle_names
                .into()
                .map(|s| IfcList(s.iter().map(|&s| s.into()).collect()))
                .into(),
            prefix_titles: prefix_titles
                .into()
                .map(|s| IfcList(s.iter().map(|&s| s.into()).collect()))
                .into(),
            suffix_titles: suffix_titles
                .into()
                .map(|s| IfcList(s.iter().map(|&s| s.into()).collect()))
                .into(),
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
            .push(role.into().into_id(ifc).id());

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
            .push(address.into().into_id(ifc).id());

        self
    }

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
}

impl IfcType for Person {}
