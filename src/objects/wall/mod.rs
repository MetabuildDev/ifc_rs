mod deserialize;
mod serialize;

use std::ops::Deref;

use super::{
    owner_history::OwnerHistory,
    shared::{element::Element, object::Object, product::Product, root::Root},
    walltype::WallType,
};
use crate::{
    geometry::{local_placement::LocalPlacement, product_definition_shape::ProductDefinitionShape},
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
    pub fn new<'a>(
        global_id: impl Into<Label>,
        owner_history: impl Into<Option<IdOr<OwnerHistory>>>,
        name: impl Into<Option<&'a str>>,
        description: impl Into<Option<&'a str>>,
        object_type: impl Into<Option<&'a str>>,
        object_placement: impl Into<Option<IdOr<LocalPlacement>>>,
        representation: impl Into<Option<IdOr<ProductDefinitionShape>>>,
        predefined_type: impl Into<Option<IdOr<WallType>>>,
        ifc: &mut IFC,
    ) -> Self {
        Self {
            element: Element::new(
                Product::new(
                    Object::new(
                        Root::new(
                            global_id.into(),
                            owner_history.into().map(|h| h.into_id(ifc).id()).into(),
                            name.into().map(|s| s.into()).into(),
                            description.into().map(|s| s.into()).into(),
                        ),
                        object_type.into().map(|s| s.into()).into(),
                    ),
                    object_placement
                        .into()
                        .map(|p| IdOr::Id(p.into_id(ifc).id()))
                        .into(),
                    representation.into().map(|r| r.into_id(ifc).id()).into(),
                ),
                OptionalParameter::omitted(),
            ),
            predefined_type: predefined_type.into().into(),
        }
    }
}

impl Deref for Wall {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        &self.element
    }
}

impl IfcType for Wall {}

#[cfg(test)]
pub mod test {
    use std::fs::write;

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

            if let Some(tag) = wall.tag.custom().map(|&id| ifc.data.get_untyped(id)) {
                println!("\ttag: {tag}");
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

        let owner_history = OwnerHistory::new(
            PersonAndOrganization::new(
                person_id.clone(),
                Organization::new(None, "organization_name", None),
                Vec::new(),
                &mut ifc,
            ),
            application_id.clone(),
            None,
            ChangeAction::Added,
            None,
            person_id,
            application_id,
            IfcTimestamp::now(),
            &mut ifc,
        );

        let axis = Axis3D::new(Point3D::from(DVec3::new(0.0, 0.0, 0.0)), &mut ifc);
        let axis_id = ifc.data.insert_new(axis);
        let local_placement = LocalPlacement::new(axis_id.clone(), &mut ifc);

        let representation = new_product_definition_shape(&mut ifc, axis_id);

        let wall = Wall::new(
            "global_id_example",
            IdOr::from(owner_history),
            "example_name",
            "example_description",
            "example_object_type",
            IdOr::from(local_placement),
            IdOr::from(representation),
            None,
            &mut ifc,
        );

        ifc.data.insert_new(wall);

        println!("{}", ifc.data);
        println!();
        print_wall_hierarchy(&ifc);

        write("wall-test.ifc", ifc.to_string()).unwrap();
    }
}
