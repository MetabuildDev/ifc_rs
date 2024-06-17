use std::{fmt::Display, ops::Deref};

use ifc_type_derive::IfcVerify;

use crate::id::Id;
use crate::id::IdOr;
use crate::ifc_type::{IfcType, IfcVerify};
use crate::parser::comma::Comma;
use crate::parser::label::Label;
use crate::parser::list::IfcList;
use crate::parser::p_space_or_comment_surrounded;
use crate::parser::IFCParse;
use crate::parser::IFCParser;
use crate::prelude::Project;
use crate::prelude::Root;
use crate::prelude::RootBuilder;
use crate::IFC;

/// The objectified relationship IfcRelDeclares handles the declaration of
/// objects (subtypes of IfcObject) or properties (subtypes of
/// IfcPropertyDefinition) to a project or project library (represented
/// by IfcProject, or IfcProjectLibrary).
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcreldeclares.htm
#[derive(IfcVerify)]
pub struct RelDeclares {
    root: Root,

    /// Reference to the IfcProject to which additional information is assigned.
    pub relating_context: Id,

    /// Set of object or property definitions that are assigned to a context and
    /// to which the unit and representation context definitions of that context apply.
    pub related_definitions: IfcList<Id>,
}

impl RelDeclares {
    pub fn new(id: impl Into<Label>, project: impl Into<IdOr<Project>>, ifc: &mut IFC) -> Self {
        Self {
            root: Root::new(id.into()),
            relating_context: project.into().or_insert(ifc).id(),
            related_definitions: IfcList::empty(),
        }
    }

    pub fn relate_definition<OBJ: IfcType>(
        mut self,
        defintion: impl Into<IdOr<OBJ>>,
        ifc: &mut IFC,
    ) -> Self {
        self.related_definitions
            .0
            .push(defintion.into().or_insert(ifc).id());
        self
    }
}

impl RootBuilder for RelDeclares {
    fn root_mut(&mut self) -> &mut Root {
        &mut self.root
    }
}

impl Deref for RelDeclares {
    type Target = Root;

    fn deref(&self) -> &Self::Target {
        &self.root
    }
}

impl IFCParse for RelDeclares {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCRELDECLARES("),

                root: Root::parse(),
                _ :Comma::parse(),
                relating_context: Id::parse(),
                _ : Comma::parse(),
                related_definitions: IfcList::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for RelDeclares {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCRELDECLARES({},{},{});",
            self.root, self.relating_context, self.related_definitions
        )
    }
}

impl IfcType for RelDeclares {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::RelDeclares;
    use crate::parser::IFCParse;

    #[test]
    fn rel_declares_round_trip() {
        let example = "IFCRELDECLARES('1lEof85zvB$O57GEVffll1',#2,$,$,#10,(#37));";

        let parsed: RelDeclares = RelDeclares::parse().parse(example).unwrap();
        let str = parsed.to_string();

        assert_eq!(example, str);
    }
}
