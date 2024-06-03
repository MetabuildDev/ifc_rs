use std::fmt::Display;

use label::Label;
use optional::OptionalParameter;

use crate::{
    id::Id,
    parser::{optional::IFCParse, *},
};

/// IfcRoot is the most abstract and root class for all entity definitions
/// that roots in the kernel or in subsequent layers of the IFC specification.
/// It is therefore the common supertype of all IFC entities, beside those
/// defined in an IFC resource schema. All entities that are subtypes of
/// IfcRoot can be used independently, whereas resource schema entities,
/// that are not subtypes of IfcRoot, are not supposed to be independent entities.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifckernel/lexical/ifcroot.htm
pub struct Root {
    /// Assignment of a globally unique identifier within the entire software world.
    pub global_id: Label,

    /// Assignment of the information about the current ownership of that object,
    /// including owning actor, application, local identification and information
    /// captured about the recent changes of the object.
    pub owner_history: OptionalParameter<Id>,

    /// Optional name for use by the participating software systems or users.
    /// For some subtypes of IfcRoot the insertion of the Name attribute may
    /// be required. This would be enforced by a where rule.
    pub name: OptionalParameter<Label>,

    /// Optional description, provided for exchanging informative comments.
    pub description: OptionalParameter<Label>,
}

impl IFCParse for Root {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                global_id: Label::parse(),
                _: p_space_or_comment_surrounded(","),
                owner_history: OptionalParameter::parse(),
                _: p_space_or_comment_surrounded(","),
                name: OptionalParameter::parse(),
                _: p_space_or_comment_surrounded(","),
                description: OptionalParameter::parse(),
            }
        }
    }
}

impl Display for Root {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{},{}",
            self.global_id, self.owner_history, self.name, self.description
        )
    }
}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use crate::objects::shared::root::{optional::IFCParse, Root};

    #[test]
    fn root_round_trip() {
        let example = "'2aG1gZj7PD2PztLOx2$IVX',#2,'Double Brick - 270',$";

        let root: Root = Root::parse().parse(example).unwrap();
        let str_root = root.to_string();

        assert_eq!(example, str_root);
    }
}
