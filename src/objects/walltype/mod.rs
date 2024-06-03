use std::ops::Deref;

use type_enum::WallTypeEnum;

use super::shared::element_type::ElementType;

mod deserialize;
mod serialize;
pub mod type_enum;

pub struct WallType {
    element_type: ElementType,

    pub predefined_type: WallTypeEnum,
}

impl Deref for WallType {
    type Target = ElementType;

    fn deref(&self) -> &Self::Target {
        &self.element_type
    }
}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::WallType;
    use crate::parser::optional::IFCParse;

    #[test]
    fn wall_type_round_trip() {
        let example = "IFCWALLTYPE('2aG1gZj7PD2PztLOx2$IVX',#2,'Double Brick - 270',$,$,$,$,$,$,.NOTDEFINED.);";

        let wall_type: WallType = WallType::parse().parse(example).unwrap();
        let str_wall_type = wall_type.to_string();

        assert_eq!(example, str_wall_type);
    }
}
