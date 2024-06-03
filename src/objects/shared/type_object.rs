use std::{fmt::Display, ops::Deref};

use crate::{
    id::Id,
    parser::{
        optional::{IFCParse, OptionalParameter},
        p_space_or_comment_surrounded, IFCParser,
    },
};

use super::root::Root;

pub struct TypeObject {
    root: Root,

    pub applicable_occurence: OptionalParameter<Id>,
    pub has_property_sets: OptionalParameter<Id>,
}

impl Deref for TypeObject {
    type Target = Root;

    fn deref(&self) -> &Self::Target {
        &self.root
    }
}

impl IFCParse for TypeObject {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                root: Root::parse(),
                _: p_space_or_comment_surrounded(","),
                applicable_occurence: OptionalParameter::parse(),
                _: p_space_or_comment_surrounded(","),
                has_property_sets: OptionalParameter::parse()
            }
        }
    }
}

impl Display for TypeObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{}",
            self.root, self.applicable_occurence, self.has_property_sets
        )
    }
}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use crate::{objects::shared::type_object::TypeObject, parser::optional::IFCParse};

    #[test]
    fn root_round_trip() {
        let example = "'2aG1gZj7PD2PztLOx2$IVX',#2,'Double Brick - 270',$,$,$";

        let type_object: TypeObject = TypeObject::parse().parse(example).unwrap();
        let str_type_object = type_object.to_string();

        assert_eq!(example, str_type_object);
    }
}
