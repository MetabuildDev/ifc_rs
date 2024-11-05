use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use ifc_rs_verify_derive::IfcVerify;

use crate::{
    id::TypedId,
    parser::{
        comma::Comma, optional::OptionalParameter, p_space_or_comment_surrounded,
        string::StringPrimitive, IFCParse, IFCParser,
    },
    prelude::*,
};

use super::StructureType;

/// Shading devices are purpose built devices to protect from the sunlight,
/// from natural light, or screening them from view. Shading devices can
/// form part of the facade or can be mounted inside the building,
/// they can be fixed or operable.
///
/// https://ifc43-docs.standards.buildingsmart.org/IFC/RELEASE/IFC4x3/HTML/lexical/IfcShadingDevice.htm
#[derive(IfcVerify)]
pub struct ShadingDevice {
    #[inherited]
    element: Element,

    pub predefined_type: OptionalParameter<ShadingDeviceTypeEnum>,
}

impl ShadingDevice {
    pub fn new(name: impl Into<StringPrimitive>) -> Self {
        Self {
            element: Element::new(Product::new(Object::new(Root::new(name.into())))),

            predefined_type: OptionalParameter::omitted(),
        }
    }

    pub fn predefined_type(mut self, predefined_type: ShadingDeviceTypeEnum) -> Self {
        self.predefined_type = predefined_type.into();
        self
    }
}

impl IFCParse for ShadingDevice {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCSHADINGDEVICE("),

                element: Element::parse(),
                _: Comma::parse(),
                predefined_type: OptionalParameter::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for ShadingDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCSHADINGDEVICE({},{});",
            self.element, self.predefined_type,
        )
    }
}

impl RootBuilder for ShadingDevice {
    fn root_mut(&mut self) -> &mut Root {
        &mut self.element
    }
}

impl ObjectBuilder for ShadingDevice {
    fn object_mut(&mut self) -> &mut Object {
        &mut self.element
    }
}

impl ProductBuilder for ShadingDevice {
    fn product_mut(&mut self) -> &mut Product {
        &mut self.element
    }
}

impl ElementBuilder for ShadingDevice {
    fn element_mut(&mut self) -> &mut Element {
        &mut self.element
    }
}

impl Deref for ShadingDevice {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        &self.element
    }
}

impl DerefMut for ShadingDevice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.element
    }
}

impl IfcType for ShadingDevice {
    fn to_structure(&self) -> Option<&dyn Structure> {
        Some(self)
    }
}
impl Structure for ShadingDevice {
    fn structure_type(&self) -> Option<StructureType<'_>> {
        Some(StructureType::ShadingDevice(self))
    }
}
impl MaterialRelatable for ShadingDevice {}

impl TransformableType for ShadingDevice {
    fn shape(&self) -> Option<TypedId<ProductDefinitionShape>> {
        self.representation.custom().cloned()
    }
}

#[cfg(test)]
mod test {
    use super::ShadingDevice;
    use crate::{parser::IFCParse, prelude::*};

    #[test]
    fn shading_device_round_trip() {
        let example = "IFCSHADINGDEVICE('0DWgwt6o1FOx7466fPk$jl',#2,$,$,$,#33,#25,$,$);";

        let shading_device: ShadingDevice = ShadingDevice::parse().parse(example).unwrap();
        let shading_device_str = shading_device.to_string();

        assert_eq!(example, shading_device_str);
    }
}
