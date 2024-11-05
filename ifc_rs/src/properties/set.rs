use std::{fmt::Display, ops::Deref};

use comma::Comma;
use ifc_rs_verify_derive::IfcVerify;
use label::Label;
use list::IfcList;

use crate::{parser::*, prelude::*};

/// The IfcPropertySet is a container that holds properties within a property tree. These
/// properties are interpreted according to their name attribute. Each individual property has a
/// significant name string. Some property sets are included in the specification of this standard
/// and have a predefined set of properties indicated by assigning a significant name. These
/// property sets are listed under "property sets" within this specification. Property sets
/// applicable to certain objects are listed in the object specification. The naming convention
/// "Pset_Xxx" applies to all those property sets that are defined as part of this specification
/// and it shall be used as the value of the Name attribute.
///
/// In addition any user defined property set can be captured. Property sets that are not declared
/// as part of the IFC specification shall have a Name value not including the "Pset_" prefix.
///
/// IfcPropertySet can be assigned to object occurrences and object types. An IfcPropertySet
/// assigned to an object type is shared among all occurrences of the same object type.
///
/// NOTE  See IfcRelDefinesByType for how to override property sets assigned to an object type
/// within the object occurrence. An IfcPropertySetTemplate may define the underlying structure,
/// i.e. the required name, the applicable object or object types to which the property set can be
/// attached, and the individual properties that can be included. Property sets are related to
/// other objects by using the relationship object that refers to the corresponding object:
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifckernel/lexical/ifcpropertyset.htm
#[derive(IfcVerify)]
pub struct PropertySet {
    root: Root,

    /// Contained set of properties. For property sets defined as part of the IFC Object model, the
    /// property objects within a property set are defined as part of the standard. If a property
    /// is not contained within the set of predefined properties, its value has not been set at
    /// this time.
    pub properties: IfcList<Id>,
}

impl PropertySet {
    pub fn new(name: impl Into<Label>, children: impl IntoIterator<Item = Id>) -> Self {
        Self {
            root: Root::new(name.into()),
            properties: IfcList(children.into_iter().collect()),
        }
    }
}

impl RootBuilder for PropertySet {
    fn root_mut(&mut self) -> &mut Root {
        &mut self.root
    }
}

impl Deref for PropertySet {
    type Target = Root;

    fn deref(&self) -> &Self::Target {
        &self.root
    }
}

impl IFCParse for PropertySet {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCPROPERTYSET("),

                root: Root::parse(),
                _: Comma::parse(),
                properties: IfcList::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for PropertySet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IFCPROPERTYSET({},{});", self.root, self.properties)
    }
}

impl IfcType for PropertySet {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::{IFCParse, PropertySet};

    #[test]
    fn property_set_round_trip() {
        let example = "IFCPROPERTYSET('3nMqHLyZHAegWs5Yyxh1ry',#2,'Pset_WallCommon',$,(#110,#111,#112,#113,#114,#115,#116,#117,#118,#119));";

        let set = PropertySet::parse().parse(example).unwrap();
        let str_set = set.to_string();

        assert_eq!(example, str_set);
    }
}
