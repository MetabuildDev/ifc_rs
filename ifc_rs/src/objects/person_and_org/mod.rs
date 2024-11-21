mod deserialize;
mod serialize;

use ifc_rs_verify_derive::IfcVerify;

use crate::{
    id::{IdOr, TypedId},
    parser::{list::IfcList, optional::OptionalParameter},
    prelude::*,
};

/// Identification of a person within an organization.
///
/// https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcactorresource/lexical/ifcpersonandorganization.htm
#[derive(Debug, Clone, IfcVerify)]
pub struct PersonAndOrganization {
    /// The person who is related to the organization.
    pub the_person: TypedId<Person>,
    /// The organization to which the person is related.
    pub the_organization: TypedId<Organization>,
    /// Roles played by the person within the context of an organization.
    pub roles: OptionalParameter<IfcList<TypedId<ActorRole>>>,
}

impl PersonAndOrganization {
    pub fn new(the_person: TypedId<Person>, the_organization: TypedId<Organization>) -> Self {
        Self {
            the_person,
            the_organization,
            roles: OptionalParameter::omitted(),
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
}

impl IfcType for PersonAndOrganization {}
