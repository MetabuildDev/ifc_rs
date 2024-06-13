mod deserialize;
mod serialize;

use std::ops::{Deref, DerefMut};

use super::{
    shared::{
        element::{Element, ElementBuilder},
        object::{Object, ObjectBuilder},
        product::{Product, ProductBuilder},
        root::{Root, RootBuilder},
    },
    Structure,
};
use crate::{
    id::{IdOr, TypedId},
    ifc_type::IfcType,
    objects::slabtype::SlabType,
    parser::{label::Label, optional::OptionalParameter},
    prelude::{ProductDefinitionShape, TransformableType},
    relations::rel_associates_material::MaterialRelatable,
    IFC,
};

/// A slab is a component of the construction that normally encloses a space
/// vertically. The slab may provide the lower support (floor) or upper
/// construction (roof slab) in any space in a building. It shall be noted,
/// that only the core or constructional part of this construction is
/// considered to be a slab. The upper finish (flooring, roofing) and the
/// lower finish (ceiling, suspended ceiling) are considered to be coverings.
/// A special type of slab is the landing, described as a floor section to
/// which one or more stair flights or ramp flights connect. May or may not be
/// adjacent to a building storey floor.
///
/// A slab may have openings, such as floor openings, or recesses. They are
/// defined by an IfcOpeningElement attached to the slab using the inverse
/// relationship HasOpenings pointing to IfcRelVoidsElement.
///
/// A particular usage type for the IfcSlab can be given (if type information
/// is available) by referring to the type object IfcSlabType, using the
/// `IfcRelDefinesByType` relationship, or (if only occurrence information is
/// given) by using the PredefinedType attribute. Values of the enumeration
/// are 'Floor' (the default), 'Roof', 'Landing', 'Baseslab', 'Notdefined'.
/// If the value 'UserDefined' is chosen, the user defined value needs to be
/// given at the attribute ObjectType.
///
/// https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcsharedbldgelements/lexical/ifcslab.htm
pub struct Slab {
    element: Element,

    /// Predefined generic types for a slab that are specified in an
    /// enumeration. There may be a property set given for the predefined types.
    ///
    /// Note: The use of the predefined type directly at the occurrence object
    /// level of IfcSlab is only permitted, if no type object `IfcSlabType`
    /// is assigned.
    pub predefined_type: OptionalParameter<IdOr<SlabType>>,
}

impl Slab {
    pub fn new<'a>(global_id: impl Into<Label>) -> Self {
        Self {
            element: Element::new(Product::new(Object::new(Root::new(global_id.into())))),
            predefined_type: OptionalParameter::omitted(),
        }
    }

    pub fn predefined_type(
        mut self,
        predefined_type: impl Into<IdOr<SlabType>>,
        ifc: &mut IFC,
    ) -> Self {
        let id_or: IdOr<_> = predefined_type.into();
        let id_or: IdOr<_> = id_or.or_insert(ifc).into();
        self.predefined_type = id_or.into();
        self
    }
}

impl RootBuilder for Slab {
    fn root_mut(&mut self) -> &mut Root {
        &mut self.element
    }
}

impl ObjectBuilder for Slab {
    fn object_mut(&mut self) -> &mut Object {
        &mut self.element
    }
}

impl ProductBuilder for Slab {
    fn product_mut(&mut self) -> &mut Product {
        &mut self.element
    }
}

impl ElementBuilder for Slab {
    fn element_mut(&mut self) -> &mut Element {
        &mut self.element
    }
}

impl Deref for Slab {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        &self.element
    }
}

impl DerefMut for Slab {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.element
    }
}

impl IfcType for Slab {}
impl Structure for Slab {}
impl MaterialRelatable for Slab {}

impl TransformableType for Slab {
    fn shape(&self) -> Option<TypedId<ProductDefinitionShape>> {
        self.representation.custom().map(|id| TypedId::new(*id))
    }
}

#[cfg(test)]
pub mod test {
    use winnow::Parser;

    use crate::geometry::axis::Axis3D;
    use crate::geometry::product_definition_shape::ProductDefinitionShape;
    use crate::geometry::representation_context::GeometricRepresentationContext;
    use crate::geometry::representation_subcontext::GeometricRepresentationSubContext;
    use crate::geometry::shape_representation::ShapeRepresentation;
    use crate::id::IdOr;
    use crate::objects::slab::Slab;
    use crate::parser::IFCParse;
    use crate::IFC;

    #[test]
    fn slab_round_trip() {
        let example = "IFCSLAB('1wAj$J2Az2V8wnBiVYd3bU',#2,$,$,$,#29,#24,$,$);";

        let slab: Slab = Slab::parse().parse(example).unwrap();
        let str_slab = slab.to_string();

        assert_eq!(example, str_slab);
    }

    pub fn print_slab_hierarchy(ifc: &IFC) {
        use crate::objects::slab::Slab;

        for slab in ifc.data.find_all_of_type::<Slab>() {
            println!("slab: {slab}");

            if let Some(owner_history) = slab
                .owner_history
                .custom()
                .map(|&id| ifc.data.get_untyped(id))
            {
                println!("\towner_history: {owner_history}");
            }

            if let Some(id_or) = slab.object_placement.custom() {
                match id_or {
                    IdOr::Id(id) => println!("\tpoint3d: {}", ifc.data.get_untyped(*id)),
                    IdOr::Custom(point) => println!("\tpoint3d: {point}"),
                }
            }

            if let Some(representation) = slab
                .representation
                .custom()
                .map(|&id| ifc.data.get_untyped(id))
            {
                println!("\trepresentation: {representation}");

                for repr in representation
                    .downcast_ref::<ProductDefinitionShape>()
                    .unwrap()
                    .representations
                    .iter()
                {
                    let shape = ifc.data.get::<ShapeRepresentation>(*repr);
                    println!("\t\tshape_representation: {shape}");

                    let sub_context = ifc
                        .data
                        .get::<GeometricRepresentationSubContext>(shape.context_of_items);

                    println!("\t\t\tsub context: {sub_context}");

                    let parent_context = ifc
                        .data
                        .get::<GeometricRepresentationContext>(sub_context.parent_context);

                    println!("\t\t\t\tcontext: {parent_context}");
                    println!(
                        "\t\t\t\t\tcoord_dims: {}",
                        parent_context.coord_space_dimension
                    );

                    let world_coord_system =
                        ifc.data.get::<Axis3D>(parent_context.world_coord_system);

                    println!("\t\t\t\t\tworld_coord_system: {world_coord_system}");
                    println!(
                        "\t\t\t\t\t\tcoord_system_point: {}",
                        ifc.data.get_untyped(world_coord_system.location)
                    );

                    for (index, item) in shape.items(&ifc).enumerate() {
                        println!("\t\t\titem {index}: {item}");
                    }
                }
            }

            if let Some(id_or) = slab.predefined_type.custom() {
                match id_or {
                    IdOr::Id(id) => println!("\tslab_type: {}", ifc.data.get_untyped(*id)),
                    IdOr::Custom(wall_type) => println!("\twall_type: {}", wall_type),
                }
            }
        }
    }
}
