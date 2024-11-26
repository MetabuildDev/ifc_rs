use std::{fmt::Display, ops::Deref};

use comma::Comma;
use ifc_rs_verify_derive::IfcVerify;
use string::StringPrimitive;

use crate::{parser::*, prelude::*};

use super::extended_base::{ExtendedPropertyBase, ExtendedPropertyBuilder};

/// The IfcMaterialProperties assigns a set of material properties to associated material
/// definitions. The set may be identified by a Name and a Description. The IfcProperty
/// (instantiable subtypes) is used to express the individual material properties by name,
/// description, value and unit.
///
/// https://ifc43-docs.standards.buildingsmart.org/IFC/RELEASE/IFC4x3/HTML/lexical/IfcMaterialProperties.htm
#[derive(IfcVerify)]
pub struct MaterialProperties {
    base: ExtendedPropertyBase,

    /// Reference to the material definition to which the set of properties is assigned.
    pub material: TypedId<Material>,
}

impl MaterialProperties {
    pub fn new(
        name: impl Into<StringPrimitive>,
        material: TypedId<Material>,
        properties: impl IntoIterator<Item = Id>,
    ) -> Self {
        Self {
            base: ExtendedPropertyBase::new(name.into(), properties),
            material,
        }
    }
}

impl ExtendedPropertyBuilder for MaterialProperties {
    fn property_mut(&mut self) -> &mut ExtendedPropertyBase {
        &mut self.base
    }
}

impl Deref for MaterialProperties {
    type Target = ExtendedPropertyBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl IFCParse for MaterialProperties {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            MaterialProperties {
                _: p_space_or_comment_surrounded("IFCMATERIALPROPERTIES("),

                base: ExtendedPropertyBase::parse(),
                _: Comma::parse(),
                material: TypedId::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for MaterialProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IFCMATERIALPROPERTIES({},{});", self.base, self.material)
    }
}

impl IfcType for MaterialProperties {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::{IFCParse, MaterialProperties};

    #[test]
    fn material_properties_round_trip() {
        let example = "IFCMATERIALPROPERTIES('Material','Cool Material',(#110,#111,#112,#113,#114,#115,#116,#117,#118,#119),#499);";

        let material = MaterialProperties::parse().parse(example).unwrap();
        let str_material = material.to_string();

        assert_eq!(example, str_material);
    }
}
