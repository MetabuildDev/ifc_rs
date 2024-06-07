mod deserialize;
mod serialize;

use crate::id::Id;
use crate::ifc_type::IfcType;
use crate::parser::label::Label;

///  IfcApplication holds the information about an IFC compliant application
///  developed by an application developer who is a member of buildingSMART.
///
/// https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcutilityresource/lexical/ifcapplication.htm
#[derive(Debug, Clone)]
pub struct Application {
    /// Name of the application developer, being requested to be member
    /// of buildingSMART. (Person/org id)
    pub application_developer: Id,
    /// The version number of this software as specified by the developer of the application.
    pub version: Label,
    /// The full name of the application as specified by the application developer.
    pub application_full_name: Label,
    /// Short identifying name for the application.
    pub application_identifier: Label,
}

impl IfcType for Application {}
