mod deserialize;
mod serialize;

use std::ops::Deref;

use super::{shared::element::Element, walltype::WallType};
use crate::{id::IdOr, ifc_type::IfcType, parser::optional::OptionalParameter};

/// The wall represents a vertical construction that may bound or
/// subdivide spaces. Wall are usually vertical, or nearly vertical,
/// planar elements, often designed to bear structural loads.
/// A wall is however not required to be load bearing.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcwall.htm
pub struct Wall {
    element: Element,

    /// Predefined generic type for a wall that is specified in an
    /// enumeration. There may be a property set given specifically
    /// for the predefined types.
    pub predefined_type: OptionalParameter<IdOr<WallType>>,
}

impl Deref for Wall {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        &self.element
    }
}

impl IfcType for Wall {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use crate::objects::wall::Wall;
    use crate::parser::IFCParse;

    #[test]
    fn wall_round_trip() {
        let example = "IFCWALL('0DWgwt6o1FOx7466fPk$jl',#2,$,$,$,#33,#25,$,$);";

        let wall: Wall = Wall::parse().parse(example).unwrap();
        let str_wall = wall.to_string();

        assert_eq!(example, str_wall);
    }
}
