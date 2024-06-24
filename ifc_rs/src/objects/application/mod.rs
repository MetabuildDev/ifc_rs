mod deserialize;
mod serialize;

use ifc_rs_verify_derive::IfcVerify;

use crate::id::{Id, IdOr};
use crate::ifc_type::{IfcType, IfcVerify};
use crate::objects::person_and_org::PersonOrOrg;
use crate::parser::label::Label;
use crate::prelude::*;
use crate::IFC;

///  IfcApplication holds the information about an IFC compliant application
///  developed by an application developer who is a member of buildingSMART.
///
/// https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcutilityresource/lexical/ifcapplication.htm
#[derive(Debug, Clone, IfcVerify)]
pub struct Application {
    /// Name of the application developer, being requested to be member
    /// of buildingSMART. (Person/org id)
    #[ifc_types(Person, Organization)]
    pub application_developer: Id,
    /// The version number of this software as specified by the developer of the application.
    pub version: Label,
    /// The full name of the application as specified by the application developer.
    pub application_full_name: Label,
    /// Short identifying name for the application.
    pub application_identifier: Label,
}

impl Application {
    pub fn new<T: PersonOrOrg>(
        application_developer: impl Into<IdOr<T>>,
        version: impl Into<Label>,
        application_full_name: impl Into<Label>,
        application_identifier: impl Into<Label>,
        ifc: &mut IFC,
    ) -> Self {
        Self {
            application_developer: application_developer.into().or_insert(ifc).id(),
            version: version.into(),
            application_full_name: application_full_name.into(),
            application_identifier: application_identifier.into(),
        }
    }
}

impl IfcType for Application {}
