mod deserialize;
mod serialize;

use crate::id::Id;
use crate::objects::access_state::AccessState;
use crate::objects::change_action::ChangeAction;
use crate::parser::optional::OptionalParameter;
use crate::parser::timestamp::IfcTimestamp;

///  IfcOwnerHistory defines all history and identification related information.
///  In order to provide fast access it is directly attached to all
///  independent objects, relationships and properties.
///
/// https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcutilityresource/lexical/ifcownerhistory.htm
#[derive(Debug, Clone)]
pub struct OwnerHistory {
    /// Direct reference to the end user who currently "owns" this object.
    /// Note that IFC includes the concept of ownership transfer from one
    /// user to another and therefore distinguishes between the Owning User
    /// and Creating User.
    pub owning_user: Id,
    /// Direct reference to the application which currently "Owns" this object
    /// on behalf of the owning user, who uses this application.
    /// Note that IFC includes the concept of ownership transfer from one
    /// app to another and therefore distinguishes between the Owning
    /// Application and Creating Application.
    pub owning_application: Id,
    /// Enumeration that defines the current access state of the object.
    pub state: OptionalParameter<AccessState>,
    /// Enumeration that defines the actions associated with changes made to
    /// the object.
    pub change_action: ChangeAction,
    /// Date and Time at which the last modification occurred.
    pub last_modified_date: OptionalParameter<IfcTimestamp>,
    /// User who carried out the last modification.
    pub last_modifying_user: OptionalParameter<Id>,
    /// Application used to carry out the last modification.
    pub last_modifying_application: OptionalParameter<Id>,
    /// Time and date of creation.
    pub creation_date: IfcTimestamp,
}
