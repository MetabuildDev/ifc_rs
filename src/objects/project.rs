use std::{fmt::Display, ops::Deref};

use crate::parser::p_space_or_comment_surrounded;
use crate::parser::IFCParse;
use crate::parser::IFCParser;

use super::shared::context::Context;

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
