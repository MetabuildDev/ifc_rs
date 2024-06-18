mod deserialize;
mod serialize;

use ifc_verify_derive::IfcVerify;

use crate::id::{IdOr, TypedId};
use crate::ifc_type::{IfcType, IfcVerify};
use crate::objects::access_state::AccessState;
use crate::objects::change_action::ChangeAction;
use crate::parser::optional::OptionalParameter;
use crate::parser::timestamp::IfcTimestamp;
use crate::IFC;

use super::application::Application;
use super::person::Person;
use super::person_and_org::PersonAndOrganization;

///  IfcOwnerHistory defines all history and identification related information.
///  In order to provide fast access it is directly attached to all
///  independent objects, relationships and properties.
///
/// https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcutilityresource/lexical/ifcownerhistory.htm
#[derive(Debug, Clone, IfcVerify)]
pub struct OwnerHistory {
    /// Direct reference to the end user who currently "owns" this object.
    /// Note that IFC includes the concept of ownership transfer from one
    /// user to another and therefore distinguishes between the Owning User
    /// and Creating User.
    pub owning_user: OptionalParameter<TypedId<PersonAndOrganization>>,
    /// Direct reference to the application which currently "Owns" this object
    /// on behalf of the owning user, who uses this application.
    /// Note that IFC includes the concept of ownership transfer from one
    /// app to another and therefore distinguishes between the Owning
    /// Application and Creating Application.
    pub owning_application: OptionalParameter<TypedId<Application>>,
    /// Enumeration that defines the current access state of the object.
    pub state: OptionalParameter<AccessState>,
    /// Enumeration that defines the actions associated with changes made to
    /// the object.
    pub change_action: ChangeAction,
    /// Date and Time at which the last modification occurred.
    pub last_modified_date: OptionalParameter<IfcTimestamp>,
    /// User who carried out the last modification.
    pub last_modifying_user: OptionalParameter<TypedId<Person>>,
    /// Application used to carry out the last modification.
    pub last_modifying_application: OptionalParameter<TypedId<Application>>,
    /// Time and date of creation.
    pub creation_date: IfcTimestamp,
}

impl OwnerHistory {
    pub fn new(change_action: ChangeAction, creation_date: IfcTimestamp) -> Self {
        Self {
            owning_user: OptionalParameter::omitted(),
            owning_application: OptionalParameter::omitted(),
            state: OptionalParameter::omitted(),
            change_action,
            last_modified_date: OptionalParameter::omitted(),
            last_modifying_user: OptionalParameter::omitted(),
            last_modifying_application: OptionalParameter::omitted(),
            creation_date,
        }
    }

    pub fn owning_user(
        mut self,
        owning_user: impl Into<IdOr<PersonAndOrganization>>,
        ifc: &mut IFC,
    ) -> Self {
        self.owning_user = owning_user.into().or_insert(ifc).into();
        self
    }

    pub fn owning_application(
        mut self,
        owning_application: impl Into<IdOr<Application>>,
        ifc: &mut IFC,
    ) -> Self {
        self.owning_application = owning_application.into().or_insert(ifc).into();
        self
    }

    pub fn state(mut self, state: AccessState) -> Self {
        self.state = state.into();
        self
    }

    pub fn last_modified_date(mut self, last_modified_date: IfcTimestamp) -> Self {
        self.last_modified_date = last_modified_date.into();
        self
    }

    pub fn last_modifying_user(
        mut self,
        last_modifying_user: impl Into<IdOr<Person>>,
        ifc: &mut IFC,
    ) -> Self {
        self.last_modifying_user = last_modifying_user.into().or_insert(ifc).into();
        self
    }

    pub fn last_modifying_application(
        mut self,
        last_modifying_application: impl Into<IdOr<Application>>,
        ifc: &mut IFC,
    ) -> Self {
        self.last_modifying_application = last_modifying_application.into().or_insert(ifc).into();
        self
    }
}

impl IfcType for OwnerHistory {}
