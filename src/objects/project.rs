use std::{fmt::Display, ops::Deref};

use crate::geometry::representation_context::GeometricRepresentationContext;
use crate::id::IdOr;
use crate::ifc_type::IfcType;
use crate::parser::label::Label;
use crate::parser::list::IfcList;
use crate::parser::p_space_or_comment_surrounded;
use crate::parser::IFCParse;
use crate::parser::IFCParser;
use crate::units::assignment::UnitAssigment;
use crate::IFC;

use super::owner_history::OwnerHistory;
use super::shared::context::Context;
use super::shared::root::Root;

/// IfcProject indicates the undertaking of some design, engineering,
/// construction, or maintenance activities leading towards a product.
/// The project establishes the context for information to be exchanged
/// or shared, and it may represent a construction project but does not
/// have to. The IfcProject's main purpose in an exchange structure is
/// to provide the root instance and the context for all other
/// information items included.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcproject.htm
pub struct Project {
    context: Context,
}

impl Project {
    pub fn new<'a>(
        global_id: impl Into<Label>,
        owner_history: impl Into<Option<IdOr<OwnerHistory>>>,
        name: impl Into<Option<&'a str>>,
        description: impl Into<Option<&'a str>>,
        object_type: impl Into<Option<&'a str>>,
        long_name: impl Into<Option<&'a str>>,
        phase: impl Into<Option<&'a str>>,
        representation_context: impl IntoIterator<Item = IdOr<GeometricRepresentationContext>>,
        units_in_context: impl Into<Option<IdOr<UnitAssigment>>>,
        ifc: &mut IFC,
    ) -> Self {
        Self {
            context: Context::new(
                Root::new(
                    global_id.into(),
                    owner_history.into().map(|h| h.into_id(ifc).id()).into(),
                    name.into().map(|s| s.into()).into(),
                    description.into().map(|s| s.into()).into(),
                ),
                object_type.into().map(|s| s.into()).into(),
                long_name.into().map(|s| s.into()).into(),
                phase.into().map(|s| s.into()).into(),
                IfcList(
                    representation_context
                        .into_iter()
                        .map(|id_or| id_or.into_id(ifc).id())
                        .collect(),
                ),
                units_in_context.into().map(|u| u.into_id(ifc).id()).into(),
            ),
        }
    }
}

impl Deref for Project {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl IFCParse for Project {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCPROJECT("),

                context: Context::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IFCPROJECT({});", self.context)
    }
}

impl IfcType for Project {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::Project;
    use crate::parser::IFCParse;

    #[test]
    fn project_round_trip() {
        let example = "IFCPROJECT('0$WU4A9R19$vKWO$AdOnKA',#2,'IfcProject',$,$,$,$,(#12),#13);";

        let parsed: Project = Project::parse().parse(example).unwrap();
        let str = parsed.to_string();

        assert_eq!(example, str);
    }
}
