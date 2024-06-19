use std::{fmt::Display, ops::Deref};

use ifc_verify_derive::IfcVerify;

use crate::id::Id;
use crate::id::IdOr;
use crate::ifc_type::{IfcType, IfcVerify};
use crate::parser::comma::Comma;
use crate::parser::label::Label;
use crate::parser::list::IfcList;
use crate::parser::p_space_or_comment_surrounded;
use crate::parser::IFCParse;
use crate::parser::IFCParser;
use crate::prelude::Root;
use crate::prelude::RootBuilder;
use crate::IFC;

/// The objectified relationship IfcRelDefinesByType defines the relationship
/// between an object type and object occurrences. The IfcRelDefinesByType is
/// a 1-to-N relationship, as it allows for the assignment of one type
/// information to a single or to many objects. Those objects then share the
/// same object type, and the property sets and properties assigned to the
/// object type.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcreldefinesbytype.htm
#[derive(IfcVerify)]
pub struct RelDefinesByType {
    root: Root,

    pub related_objects: IfcList<Id>,

    /// Reference to the type (or style) information for that object or set of objects.
    pub relating_type: Id,
}

impl RelDefinesByType {
    pub fn new<OBJ: IfcType>(
        name: impl Into<Label>,
        relating_type: impl Into<IdOr<OBJ>>,
        ifc: &mut IFC,
    ) -> Self {
        Self {
            root: Root::new(name.into()),
            related_objects: IfcList::empty(),
            relating_type: relating_type.into().or_insert(ifc).id(),
        }
    }

    pub fn relate_obj<OBJ: IfcType>(mut self, object: impl Into<IdOr<OBJ>>, ifc: &mut IFC) -> Self {
        self.related_objects
            .0
            .push(object.into().or_insert(ifc).id());
        self
    }
}

impl RootBuilder for RelDefinesByType {
    fn root_mut(&mut self) -> &mut Root {
        &mut self.root
    }
}

impl Deref for RelDefinesByType {
    type Target = Root;

    fn deref(&self) -> &Self::Target {
        &self.root
    }
}

impl IFCParse for RelDefinesByType {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCRELDEFINESBYTYPE("),

                root: Root::parse(),
                _ :Comma::parse(),
                related_objects: IfcList::parse(),
                _ : Comma::parse(),
                relating_type: Id::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for RelDefinesByType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCRELDEFINESBYTYPE({},{},{});",
            self.root, self.related_objects, self.relating_type
        )
    }
}

impl IfcType for RelDefinesByType {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::RelDefinesByType;
    use crate::parser::IFCParse;

    #[test]
    fn rel_declares_round_trip() {
        let example = "IFCRELDEFINESBYTYPE('1$EkFElNT8TB_VUVG1FtMe',#2,$,$,(#11),#37);";

        let parsed: RelDefinesByType = RelDefinesByType::parse().parse(example).unwrap();
        let str = parsed.to_string();

        assert_eq!(example, str);
    }
}
