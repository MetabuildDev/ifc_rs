use std::{fmt::Display, ops::Deref};

use ifc_rs_verify_derive::IfcVerify;

use crate::id::Id;
use crate::id::IdOr;
use crate::parser::comma::Comma;
use crate::parser::label::Label;
use crate::parser::list::IfcList;
use crate::parser::p_space_or_comment_surrounded;
use crate::parser::IFCParse;
use crate::parser::IFCParser;
use crate::prelude::*;

/// The objectified relationship IfcRelDefinesByProperties defines the relationships between
/// property set definitions and objects. Properties are aggregated in property sets. Property sets
/// can be either directly assigned to occurrence objects using this relationship, or assigned to
/// an object type and assigned via that type to occurrence objects. The assignment of an
/// IfcPropertySet to an IfcTypeObject is not handled via this objectified relationship, but
/// through the direct relationship HasPropertySets at IfcTypeObject.
///
/// The IfcRelDefinesByProperties is an N-to-N relationship, as it allows for the assignment of one
/// or more property sets to one or more objects. Those objects then share the same property
/// definition.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifckernel/lexical/ifcreldefinesbyproperties.htm
#[derive(IfcVerify)]
pub struct RelDefinesByProperties {
    root: Root,

    /// Reference to the objects (or single object) to which the property definition applies.
    pub related_objects: IfcList<Id>,

    /// Reference to the property set definition for that object or set of objects.
    pub relating_property_definition: Id,
}

impl RelDefinesByProperties {
    pub fn new<SET: IfcType>(
        name: impl Into<Label>,
        relating_propert_definition: impl Into<IdOr<SET>>,
        ifc: &mut IFC,
    ) -> Self {
        Self {
            root: Root::new(name.into()),
            related_objects: IfcList::empty(),
            relating_property_definition: relating_propert_definition.into().or_insert(ifc).id(),
        }
    }

    pub fn relate_obj<OBJ: IfcType>(mut self, object: impl Into<IdOr<OBJ>>, ifc: &mut IFC) -> Self {
        self.related_objects
            .0
            .push(object.into().or_insert(ifc).id());
        self
    }
}

impl RootBuilder for RelDefinesByProperties {
    fn root_mut(&mut self) -> &mut Root {
        &mut self.root
    }
}

impl Deref for RelDefinesByProperties {
    type Target = Root;

    fn deref(&self) -> &Self::Target {
        &self.root
    }
}

impl IFCParse for RelDefinesByProperties {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCRELDEFINESBYPROPERTIES("),

                root: Root::parse(),
                _ :Comma::parse(),
                related_objects: IfcList::parse(),
                _ : Comma::parse(),
                relating_property_definition: Id::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for RelDefinesByProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCRELDEFINESBYPROPERTIES({},{},{});",
            self.root, self.related_objects, self.relating_property_definition
        )
    }
}

impl IfcType for RelDefinesByProperties {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::RelDefinesByProperties;
    use crate::parser::IFCParse;

    #[test]
    fn rel_defines_by_properties_round_trip() {
        let example = "IFCRELDEFINESBYPROPERTIES('1$EkFElNT8TB_VUVG1FtMe',#2,$,$,(#11),#37);";

        let parsed: RelDefinesByProperties =
            RelDefinesByProperties::parse().parse(example).unwrap();
        let str = parsed.to_string();

        assert_eq!(example, str);
    }
}
