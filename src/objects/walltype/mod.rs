use std::ops::Deref;

use ifc_verify_derive::IfcVerify;
use type_enum::WallTypeEnum;

use crate::{
    ifc_type::{IfcType, IfcVerify},
    parser::label::Label,
    prelude::{ElementTypeBuilder, Root, RootBuilder, TypeObject, TypeProduct},
    relations::rel_associates_material::MaterialRelatable,
    IFC,
};

use super::shared::{
    element_type::ElementType, type_object::TypeObjectBuilder, type_product::TypeProductBuilder,
};

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
#[derive(IfcVerify)]
pub struct WallType {
    element_type: ElementType,

    /// Identifies the predefined types of a wall element from which
    /// the type required may be set.
    pub predefined_type: WallTypeEnum,
}

impl WallType {
    pub fn new(name: impl Into<Label>, predefined_type: WallTypeEnum) -> Self {
        Self {
            element_type: ElementType::new(TypeProduct::new(TypeObject::new(Root::new(
                name.into(),
            )))),
            predefined_type,
        }
    }
}

impl ElementTypeBuilder for WallType {
    fn element_type_mut(&mut self) -> &mut ElementType {
        &mut self.element_type
    }
}

impl TypeProductBuilder for WallType {
    fn type_product_mut(&mut self) -> &mut TypeProduct {
        &mut self.element_type
    }
}

impl TypeObjectBuilder for WallType {
    fn type_object_mut(&mut self) -> &mut TypeObject {
        &mut self.element_type
    }
}

impl RootBuilder for WallType {
    fn root_mut(&mut self) -> &mut Root {
        &mut self.element_type
    }
}

impl Deref for WallType {
    type Target = ElementType;

    fn deref(&self) -> &Self::Target {
        &self.element_type
    }
}

impl IfcType for WallType {}
impl MaterialRelatable for WallType {}

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
