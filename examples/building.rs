use glam::DVec3;
use ifc4::prelude::*;
use std::fs::write;

fn main() {
    let mut ifc = IFC::default();

    let (person, application) =
        create_person_and_applicaton(&mut ifc, "Max", "BuildingExample", "BuildingExample");

    let owner_history = create_owner_history(&mut ifc, "ExampleOrganization", person, application);

    let length = SiUnit::new(IfcUnitEnum::LengthUnit, None, IfcUnitName::Metre);
    let angle = SiUnit::new(IfcUnitEnum::PlaneAngleUnit, None, IfcUnitName::Radian);
    let time = SiUnit::new(IfcUnitEnum::TimeUnit, None, IfcUnitName::Second);

    let unit_assignment = UnitAssigment::new([length.into(), angle.into(), time.into()], &mut ifc);

    let world_root = Axis3D::new(Point3D::from(DVec3::new(0.0, 0.0, 0.0)), &mut ifc);
    let world_root_id = ifc.data.insert_new(world_root);

    let context = GeometricRepresentationContext::new(
        "ExampleContext",
        DimensionCount::Three,
        0.01,
        world_root_id.id_or(),
        &mut ifc,
    );
    let context_id = ifc.data.insert_new(context);

    let project = Project::new("ExampleProject")
        .owner_history(owner_history.id(), &mut ifc)
        .unit_assignment(unit_assignment, &mut ifc)
        .add_context(context_id.id(), &mut ifc);

    let building = Building::new("ExampleBuilding").owner_history(owner_history.id(), &mut ifc);
    let building_id = ifc.data.insert_new(building);

    let project_building_relation = RelAggregates::new("ProjectBuildingLink")
        .relate_project_with_buildings(project, [building_id.id().into()], &mut ifc);
    ifc.data.insert_new(project_building_relation);

    let sub_context = GeometricRepresentationSubContext::derive(
        context_id.id(),
        None,
        GeometricProjection::ModelView,
        None,
        &mut ifc,
    );

    let shape_repr = ShapeRepresentation::new(sub_context, &mut ifc).add_item(
        PolyLine::from_3d(
            [
                DVec3::new(0.0, 0.0, 0.0).into(),
                DVec3::new(1.0, 0.0, 0.0).into(),
                DVec3::new(1.0, 1.0, 0.0).into(),
                DVec3::new(0.0, 1.0, 0.0).into(),
            ]
            .into_iter(),
            &mut ifc,
        ),
        &mut ifc,
    );

    let product_shape = ProductDefinitionShape::new().add_representation(shape_repr, &mut ifc);

    let local_placement = LocalPlacement::new(world_root_id, &mut ifc);

    let wall = Wall::new("ExampleWall")
        .owner_history(owner_history.id(), &mut ifc)
        .object_placement(local_placement, &mut ifc)
        .representation(product_shape, &mut ifc);

    let spatial_relation =
        RelContainedInSpatialStructure::new("BuildingWallLink", building_id, &mut ifc)
            .relate_structure(wall, &mut ifc);
    ifc.data.insert_new(spatial_relation);

    write("examples/building_example.ifc", ifc.to_string()).unwrap();
}

fn create_person_and_applicaton(
    ifc: &mut IFC,
    person_name: &str,
    application_name: &str,
    application_id: &str,
) -> (TypedId<Person>, TypedId<Application>) {
    let person = Person::empty().id(person_name).given_name(person_name);
    let person_id = ifc.data.insert_new(person);

    let application = Application::new(
        person_id.clone(),
        "0.0.1",
        application_name,
        application_id,
        ifc,
    );
    let application_id = ifc.data.insert_new(application);

    (person_id, application_id)
}

fn create_owner_history(
    ifc: &mut IFC,
    organization_name: &str,
    person: TypedId<Person>,
    application: TypedId<Application>,
) -> TypedId<OwnerHistory> {
    let person_and_org = PersonAndOrganization::new(
        person.clone(),
        Organization::new(None, organization_name, None),
        Vec::new(),
        ifc,
    );

    let owner_history = OwnerHistory::new(ChangeAction::Added, IfcTimestamp::now())
        .owning_user(person_and_org, ifc)
        .owning_application(application.id_or(), ifc)
        .last_modified_date(IfcTimestamp::now())
        .last_modifying_user(person, ifc)
        .last_modifying_application(application, ifc);

    ifc.data.insert_new(owner_history)
}
