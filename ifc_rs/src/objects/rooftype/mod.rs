use std::ops::Deref;

use ifc_rs_verify_derive::IfcVerify;
use type_enum::RoofTypeEnum;

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

/// The building element type IfcRoofType defines commonly shared information
/// for occurrences of roofs.
///
/// https://standards.buildingsmart.org/MVD/RELEASE/IFC4/ADD2_TC1/RV1_2/HTML/schema/ifcsharedbldgelements/lexical/ifcrooftype.htm
#[derive(IfcVerify)]
pub struct RoofType {
    #[inherited]
    element_type: ElementType,

    /// Identifies the predefined types of a roof element from which the type
    /// required may be set.
    pub predefined_type: RoofTypeEnum,
}

impl RoofType {
    pub fn new(name: impl Into<Label>, predefined_type: RoofTypeEnum) -> Self {
        Self {
            element_type: ElementType::new(TypeProduct::new(TypeObject::new(Root::new(
                name.into(),
            )))),
            predefined_type,
        }
    }
}

impl ElementTypeBuilder for RoofType {
    fn element_type_mut(&mut self) -> &mut ElementType {
        &mut self.element_type
    }
}

impl TypeProductBuilder for RoofType {
    fn type_product_mut(&mut self) -> &mut TypeProduct {
        &mut self.element_type
    }
}

impl TypeObjectBuilder for RoofType {
    fn type_object_mut(&mut self) -> &mut TypeObject {
        &mut self.element_type
    }
}

impl RootBuilder for RoofType {
    fn root_mut(&mut self) -> &mut Root {
        &mut self.element_type
    }
}

impl Deref for RoofType {
    type Target = ElementType;

    fn deref(&self) -> &Self::Target {
        &self.element_type
    }
}

impl IfcType for RoofType {}
impl MaterialRelatable for RoofType {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use crate::parser::IFCParse;

    use super::RoofType;

    #[test]
    fn roof_type_round_trip() {
        let example =
            "IFCROOFTYPE('0RSW$KKbzCZ9QaSm3GoEan',#2,'200mm Concrete',$,$,$,$,$,$,.GABLE_ROOF.);";

        let roof_type = RoofType::parse().parse(example).unwrap();
        let str_type = roof_type.to_string();

        assert_eq!(example, str_type);
    }
}
