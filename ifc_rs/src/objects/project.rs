use std::ops::DerefMut;
use std::{fmt::Display, ops::Deref};

use ifc_rs_verify_derive::IfcVerify;

use crate::parser::label::Label;
use crate::parser::{p_space_or_comment_surrounded, IFCParse, IFCParser};
use crate::prelude::*;

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
#[derive(IfcVerify)]
pub struct Project {
    #[inherited]
    context: Context,
}

impl Project {
    pub fn new(name: impl Into<Label>) -> Self {
        Self {
            context: Context::new(Root::new(name.into())),
        }
    }
}

impl ContextBuilder for Project {
    fn context_mut(&mut self) -> &mut Context {
        &mut self.context
    }
}

impl RootBuilder for Project {
    fn root_mut(&mut self) -> &mut Root {
        &mut self.context
    }
}

impl Deref for Project {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl DerefMut for Project {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.context
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
