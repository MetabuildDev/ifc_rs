mod deserialize;
mod serialize;

use crate::id::Id;
use crate::parser::label::Label;
use crate::parser::list::IfcList;
use crate::parser::optional::OptionalParameter;

/// A named and structured grouping with a corporate identity.
///
/// https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcactorresource/lexical/ifcorganization.htm
#[derive(Debug, Clone)]
pub struct Organization {
    /// Identification of the organization.
    pub id: OptionalParameter<Label>,
    /// The word, or group of words, by which the organization is referred to.
    pub name: Label,
    /// Text that relates the nature of the organization.
    pub description: OptionalParameter<Label>, // TODO: Text
    /// Roles played by the organization.
    pub roles: OptionalParameter<IfcList<Id>>,
    /// Postal and telecommunication addresses of an organization.
    pub addresses: OptionalParameter<IfcList<Id>>,
}
