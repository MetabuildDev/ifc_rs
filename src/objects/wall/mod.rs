mod deserialize;
mod serialize;

use std::ops::Deref;

use super::{shared::element::Element, walltype::WallType};
use crate::{id::IdOr, parser::optional::OptionalParameter};

pub struct Wall {
    pub element: Element,

    pub predefined_type: OptionalParameter<IdOr<WallType>>,
}

impl Deref for Wall {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        &self.element
    }
}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use crate::{objects::wall::Wall, parser::optional::IFCParse};

    #[test]
    fn wall_round_trip() {
        let example = "IFCWALL('0DWgwt6o1FOx7466fPk$jl',#2,$,$,$,#33,#25,$,$);";

        let wall: Wall = Wall::parse().parse(example).unwrap();
        let str_wall = wall.to_string();

        assert_eq!(example, str_wall);
    }
}
