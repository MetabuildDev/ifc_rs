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
    walltype::WallType,
    Structure,
};
use crate::{
    id::IdOr,
    ifc_type::IfcType,
    parser::{label::Label, optional::OptionalParameter},
    IFC,
};

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

impl Wall {
    pub fn new<'a>(global_id: impl Into<Label>) -> Self {
        Self {
            element: Element::new(Product::new(Object::new(Root::new(global_id.into())))),
            predefined_type: OptionalParameter::omitted(),
        }
    }

    pub fn predefined_type(
        mut self,
        predefined_type: impl Into<IdOr<WallType>>,
        ifc: &mut IFC,
    ) -> Self {
        self.predefined_type = predefined_type.into().into_id(ifc).into();
        self
    }
}

impl RootBuilder for Wall {
    fn root_mut(&mut self) -> &mut Root {
        &mut self.element
    }
}

impl ObjectBuilder for Wall {
    fn object_mut(&mut self) -> &mut Object {
        &mut self.element
    }
}

impl ProductBuilder for Wall {
    fn product_mut(&mut self) -> &mut Product {
        &mut self.element
    }
}

impl ElementBuilder for Wall {
    fn element_mut(&mut self) -> &mut Element {
        &mut self.element
    }
}

impl Deref for Wall {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        &self.element
    }
}

impl DerefMut for Wall {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.element
    }
}

impl IfcType for Wall {}
impl Structure for Wall {}

#[cfg(test)]
pub mod test {
    use glam::DVec3;
    use winnow::Parser;

    use crate::geometry::axis::Axis3D;
    use crate::geometry::local_placement::LocalPlacement;
    use crate::geometry::point::Point3D;
    use crate::geometry::product_definition_shape::test::new_product_definition_shape;
    use crate::geometry::product_definition_shape::ProductDefinitionShape;
    use crate::geometry::representation_context::GeometricRepresentationContext;
    use crate::geometry::representation_subcontext::GeometricRepresentationSubContext;
    use crate::geometry::shape_representation::ShapeRepresentation;
    use crate::id::IdOr;
    use crate::objects::application::Application;
    use crate::objects::change_action::ChangeAction;
    use crate::objects::organization::Organization;
    use crate::objects::owner_history::OwnerHistory;
    use crate::objects::person::Person;
    use crate::objects::person_and_org::PersonAndOrganization;
    use crate::objects::shared::{product::ProductBuilder, root::RootBuilder};
    use crate::objects::wall::Wall;
    use crate::parser::timestamp::IfcTimestamp;
    use crate::parser::IFCParse;
    use crate::IFC;

    #[test]
    fn wall_round_trip() {
        let example = "IFCWALL('0DWgwt6o1FOx7466fPk$jl',#2,$,$,$,#33,#25,$,$);";

        let wall: Wall = Wall::parse().parse(example).unwrap();
        let str_wall = wall.to_string();

        assert_eq!(example, str_wall);
    }

    pub fn print_wall_hierarchy(ifc: &IFC) {
        use crate::objects::wall::Wall;

        for wall in ifc.data.find_all_of_type::<Wall>() {
            println!("wall: {wall}");

            if let Some(owner_history) = wall
                .owner_history
                .custom()
                .map(|&id| ifc.data.get_untyped(id))
            {
                println!("\towner_history: {owner_history}");
            }

            if let Some(id_or) = wall.object_placement.custom() {
                match id_or {
                    IdOr::Id(id) => println!("\tpoint3d: {}", ifc.data.get_untyped(*id)),
                    IdOr::Custom(point) => println!("\tpoint3d: {point}"),
                }
            }

            if let Some(representation) = wall
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

            if let Some(id_or) = wall.predefined_type.custom() {
                match id_or {
                    IdOr::Id(id) => println!("\twall_type: {}", ifc.data.get_untyped(*id)),
                    IdOr::Custom(wall_type) => println!("\twall_type: {}", wall_type),
                }
            }
        }
    }

    #[test]
    fn create_wall() {
        let mut ifc = IFC::default();

        let person_id = ifc.data.insert_new(Person::empty());
        let application = Application::new(
            person_id.clone(),
            "0.0.1",
            "create_wall_test",
            "IFC4",
            &mut ifc,
        );
        let application_id = ifc.data.insert_new(application);

        let person_and_org = PersonAndOrganization::new(
            person_id.clone(),
            Organization::new(None, "organization_name", None),
            Vec::new(),
            &mut ifc,
        );

        let owner_history = OwnerHistory::new(ChangeAction::Added, IfcTimestamp::now())
            .owning_user(person_and_org, &mut ifc)
            .owning_application(application_id, &mut ifc);

        let axis = Axis3D::new(Point3D::from(DVec3::new(0.0, 0.0, 0.0)), &mut ifc);
        let axis_id = ifc.data.insert_new(axis);
        let local_placement = LocalPlacement::new(axis_id.clone(), &mut ifc);

        let representation = new_product_definition_shape(&mut ifc, axis_id.into());

        let wall = Wall::new("global_id_example")
            .owner_history(owner_history, &mut ifc)
            .object_placement(local_placement, &mut ifc)
            .representation(representation, &mut ifc);

        ifc.data.insert_new(wall);

        println!("{}", ifc.data);
        println!();
        print_wall_hierarchy(&ifc);
    }
}
