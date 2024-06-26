use std::ops::Deref;

use ifc_rs_verify_derive::IfcVerify;
use type_enum::SlabTypeEnum;

use crate::{
    parser::label::Label, prelude::*, relations::rel_associates_material::MaterialRelatable,
};

use super::shared::{
    element_type::ElementType, type_object::TypeObjectBuilder, type_product::TypeProductBuilder,
};

mod deserialize;
mod serialize;
pub(crate) mod type_enum;

/// The element type IfcSlabType defines commonly shared information for
/// occurrences of slabs. The set of shared information may include:
///
/// * common properties within shared property sets
/// * common material information
/// * common material layer definitions
/// * common shape representations
///
/// Note: it is illegal to share shape representations as representation maps
/// for occurrences of IfcSlabStandardCase.
///
/// It is used to define a slab specification (i.e. the specific product
/// information, that is common to all occurrences of that product type).
/// Slab types may be exchanged without being already assigned to occurrences.
///
/// Note:The product representations are defined as representation maps (at
/// the level of the supertype IfcTypeProduct, which gets assigned by an
/// element occurrence instance through the IfcShapeRepresentation.Item[1]
/// being an IfcMappedItem.
///
/// The occurrences of the IfcSlabType within building models are represented
/// by instances of IfcSlabStandardCase if the IfcSlabType has a single
/// associated IfcMaterialLayerSet; otherwise they are represented by instances
/// of IfcSlab, or IfcSlabElementedCase.
#[derive(IfcVerify)]
pub struct SlabType {
    #[inherited]
    element_type: ElementType,

    /// Identifies the predefined types of a slab element from which the type
    /// required may be set.
    pub predefined_type: SlabTypeEnum,
}

impl SlabType {
    pub fn new(name: impl Into<Label>, predefined_type: SlabTypeEnum) -> Self {
        Self {
            element_type: ElementType::new(TypeProduct::new(TypeObject::new(Root::new(
                name.into(),
            )))),
            predefined_type,
        }
    }
}

impl ElementTypeBuilder for SlabType {
    fn element_type_mut(&mut self) -> &mut ElementType {
        &mut self.element_type
    }
}

impl TypeProductBuilder for SlabType {
    fn type_product_mut(&mut self) -> &mut TypeProduct {
        &mut self.element_type
    }
}

impl TypeObjectBuilder for SlabType {
    fn type_object_mut(&mut self) -> &mut TypeObject {
        &mut self.element_type
    }
}

impl RootBuilder for SlabType {
    fn root_mut(&mut self) -> &mut Root {
        &mut self.element_type
    }
}

impl Deref for SlabType {
    type Target = ElementType;

    fn deref(&self) -> &Self::Target {
        &self.element_type
    }
}

impl IfcType for SlabType {}
impl MaterialRelatable for SlabType {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use crate::parser::IFCParse;

    use super::SlabType;

    #[test]
    fn slab_type_round_trip() {
        let example =
            "IFCSLABTYPE('0RSW$KKbzCZ9QaSm3GoEan',#2,'200mm Concrete',$,$,$,$,$,$,.FLOOR.);";

        let slab_type: SlabType = SlabType::parse().parse(example).unwrap();
        let str_slab_type = slab_type.to_string();

        assert_eq!(example, str_slab_type);
    }
}
