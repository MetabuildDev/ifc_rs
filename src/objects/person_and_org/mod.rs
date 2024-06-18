mod deserialize;
mod serialize;

use ifc_verify_derive::IfcVerify;

use crate::id::{IdOr, TypedId};
use crate::ifc_type::{IfcType, IfcVerify};
use crate::parser::list::IfcList;
use crate::parser::optional::OptionalParameter;
use crate::IFC;

use super::actor_role::ActorRole;
use super::organization::Organization;
use super::person::Person;

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
    pub fn new(
        the_person: impl Into<IdOr<Person>>,
        the_organization: impl Into<IdOr<Organization>>,
        ifc: &mut IFC,
    ) -> Self {
        Self {
            the_person: the_person.into().or_insert(ifc),
            the_organization: the_organization.into().or_insert(ifc),
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

pub trait PersonOrOrg: IfcType {}
impl PersonOrOrg for Person {}
impl PersonOrOrg for Organization {}
