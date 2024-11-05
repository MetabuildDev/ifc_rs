use std::{fmt::Display, ops::Deref};

use ifc_rs_verify_derive::IfcVerify;

use crate::{
    parser::{
        comma::Comma, p_space_or_comment_surrounded, string::StringPrimitive, IFCParse, IFCParser,
    },
    prelude::*,
};

use super::shared::{
    element_type::ElementType, type_object::TypeObjectBuilder, type_product::TypeProductBuilder,
};

/// The building element type IfcShadingDeviceType defines commonly shared
/// information for occurrences of shading devices. The set of shared
/// information may include:
///  * common properties with shared property sets
///  * common representations
///  * common materials
///  * common composition of elements
///
/// https://ifc43-docs.standards.buildingsmart.org/IFC/RELEASE/IFC4x3/HTML/lexical/IfcShadingDeviceType.htm
#[derive(IfcVerify)]
pub struct ShadingDeviceType {
    #[inherited]
    element_type: ElementType,

    /// A list of types to further identify the object. Some property sets may
    /// be specifically applicable to one of these types.
    pub predefined_type: ShadingDeviceTypeEnum,
}

impl ShadingDeviceType {
    pub fn new(name: impl Into<StringPrimitive>, predefined_type: ShadingDeviceTypeEnum) -> Self {
        Self {
            element_type: ElementType::new(TypeProduct::new(TypeObject::new(Root::new(
                name.into(),
            )))),
            predefined_type,
        }
    }
}

impl IFCParse for ShadingDeviceType {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCSHADINGDEVICETYPE("),

                element_type: ElementType::parse(),
                _: Comma::parse(),
                predefined_type: ShadingDeviceTypeEnum::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for ShadingDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCSHADINGDEVICETYPE({},{});",
            self.element_type, self.predefined_type,
        )
    }
}

impl ElementTypeBuilder for ShadingDeviceType {
    fn element_type_mut(&mut self) -> &mut ElementType {
        &mut self.element_type
    }
}

impl TypeProductBuilder for ShadingDeviceType {
    fn type_product_mut(&mut self) -> &mut TypeProduct {
        &mut self.element_type
    }
}

impl TypeObjectBuilder for ShadingDeviceType {
    fn type_object_mut(&mut self) -> &mut TypeObject {
        &mut self.element_type
    }
}

impl RootBuilder for ShadingDeviceType {
    fn root_mut(&mut self) -> &mut Root {
        &mut self.element_type
    }
}

impl Deref for ShadingDeviceType {
    type Target = ElementType;

    fn deref(&self) -> &Self::Target {
        &self.element_type
    }
}

impl IfcType for ShadingDeviceType {}
impl MaterialRelatable for ShadingDeviceType {}

#[cfg(test)]
mod test {
    use crate::parser::IFCParse;
    use winnow::Parser;

    use super::ShadingDeviceType;

    #[test]
    fn shading_device_type_round_trip() {
        let example =
            "IFCSHADINGDEVICETYPE('2aG1gZj7PD2PztLOx2$IVX',#2,$,$,$,$,$,$,$,.NOTDEFINED.);";

        let shading_device_type: ShadingDeviceType =
            ShadingDeviceType::parse().parse(example).unwrap();
        let str_shading_device_type = shading_device_type.to_string();

        assert_eq!(example, str_shading_device_type);
    }
}
