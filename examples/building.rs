use glam::DVec3;
use ifc4::{
    geometry::{
        axis::Axis3D, dimension_count::DimensionCount, geometric_projection::GeometricProjection,
        point::Point3D, representation_context::GeometricRepresentationContext,
        representation_subcontext::GeometricRepresentationSubContext,
    },
    id::IdOr,
    parser::timestamp::IfcTimestamp,
    prelude::*,
    units::{
        assignment::UnitAssigment, length_unit::IfcUnitEnum, name::IfcUnitName, si_unit::SiUnit,
    },
    IFC,
};
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
        "Model",
        DimensionCount::Three,
        0.01,
        world_root_id,
        &mut ifc,
    );
    let context_id = ifc.data.insert_new(context);

    let sub_context = GeometricRepresentationSubContext::derive(
        context_id.clone(),
        None,
        GeometricProjection::ModelView,
        None,
        &mut ifc,
    );

    let project = Project::new("project_example_id")
        .owner_history(owner_history.clone(), &mut ifc)
        .unit_assignment(unit_assignment, &mut ifc)
        .add_context(context_id, &mut ifc);

    let building = Building::new("Building_example_id").owner_history(owner_history, &mut ifc);

    write("examples/building_example.ifc", ifc.to_string()).unwrap();
}

fn create_person_and_applicaton(
    ifc: &mut IFC,
    person_name: &str,
    application_name: &str,
    application_id: &str,
) -> (IdOr<Person>, IdOr<Application>) {
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
    person: IdOr<Person>,
    application: IdOr<Application>,
) -> IdOr<OwnerHistory> {
    let owner_history = OwnerHistory::new(
        PersonAndOrganization::new(
            person.clone(),
            Organization::new(None, organization_name, None),
            Vec::new(),
            ifc,
        ),
        application.clone(),
        None,
        ChangeAction::Added,
        None,
        person,
        application,
        IfcTimestamp::now(),
        ifc,
    );

    ifc.data.insert_new(owner_history)
}
