mod deserialize;
mod serialize;

use crate::id::Id;
use crate::parser::label::Label;
use crate::parser::list::IfcList;
use crate::parser::optional::OptionalParameter;

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
    pub roles: OptionalParameter<IfcList<Label>>, // TODO: IfcActorRole
    /// Postal and telecommunication addresses of a person.
    pub addresses: OptionalParameter<IfcList<Id>>,
}

// #39= IFCPERSON($,'','hannah.schmitz',$,$,$,$,(#35));
