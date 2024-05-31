mod deserialize;
mod serialize;

use crate::parser::label::Label;
use crate::parser::list::IfcList;
use crate::parser::optional::OptionalParameter;

///  Address to which telephone, electronic mail and other forms of
///  telecommunications should be addressed.
///
/// https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcactorresource/lexical/ifctelecomaddress.htm
#[derive(Debug, Clone)]
pub struct TelecomAddress {
    // First three attributes from IfcAddress:
    //
    /// Identifies the logical location of the address.
    pub purpose: OptionalParameter<Label>, // TODO: AddressTypeEnum
    /// Text that relates the nature of the address.
    pub description: OptionalParameter<Label>, // TODO: IfcText
    /// Allows for specification of user specific purpose of the address beyond the
    /// enumeration values provided by Purpose attribute of type IfcAddressTypeEnum.
    /// When a value is provided for attribute UserDefinedPurpose, in parallel the
    /// attribute Purpose shall have enumeration value USERDEFINED.
    pub user_defined_purpose: OptionalParameter<Label>,

    /// The list of telephone numbers at which telephone messages may be received.
    pub telephone_numbers: OptionalParameter<IfcList<Label>>,
    /// The list of fax numbers at which fax messages may be received.
    pub facsimile_numbers: OptionalParameter<IfcList<Label>>,
    /// The pager number at which paging messages may be received.
    pub pager_number: OptionalParameter<Label>,
    /// The list of Email addresses at which Email messages may be received.
    pub email_addresses: OptionalParameter<IfcList<Label>>,
    /// The world wide web address at which the preliminary page of information
    /// for the person or organization can be located.
    pub homepage_url: OptionalParameter<Label>,
}
