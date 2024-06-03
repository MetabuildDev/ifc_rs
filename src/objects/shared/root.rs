use std::fmt::Display;

use label::Label;
use optional::OptionalParameter;

use crate::{
    id::Id,
    parser::{optional::IFCParse, *},
};

pub struct Root {
    pub global_id: Label,
    pub owner_history: OptionalParameter<Id>,
    pub name: OptionalParameter<Label>,
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
