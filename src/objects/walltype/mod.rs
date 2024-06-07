use std::ops::Deref;

use type_enum::WallTypeEnum;

use crate::ifc_type::IfcType;

use super::shared::element_type::ElementType;

mod deserialize;
mod serialize;
pub mod type_enum;

/// The element type IfcWallType defines commonly shared information
/// for occurrences of walls. The set of shared information may include:
///   * common properties within shared property sets
///   * common material information
///   * common material layer definitions
///   * common shape representations
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcwalltype.htm
pub struct WallType {
    element_type: ElementType,

    /// Identifies the predefined types of a wall element from which
    /// the type required may be set.
    pub predefined_type: WallTypeEnum,
}

impl Deref for WallType {
    type Target = ElementType;

    fn deref(&self) -> &Self::Target {
        &self.element_type
    }
}

impl IfcType for WallType {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use crate::parser::IFCParse;

    use super::WallType;

    #[test]
    fn wall_type_round_trip() {
        let example = "IFCWALLTYPE('2aG1gZj7PD2PztLOx2$IVX',#2,'Double Brick - 270',$,$,$,$,$,$,.NOTDEFINED.);";

        let wall_type: WallType = WallType::parse().parse(example).unwrap();
        let str_wall_type = wall_type.to_string();

        assert_eq!(example, str_wall_type);
    }
}
