mod deserialize;
mod serialize;

use crate::id::Id;
use crate::parser::list::IfcList;
use crate::parser::optional::OptionalParameter;

/// Identification of a person within an organization.
///
/// https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcactorresource/lexical/ifcpersonandorganization.htm
#[derive(Debug, Clone)]
pub struct PersonAndOrganization {
    /// The person who is related to the organization.
    pub the_person: Id,
    /// The organization to which the person is related.
    pub the_organization: Id,
    /// Roles played by the person within the context of an organization.
    pub roles: OptionalParameter<IfcList<Id>>,
}
