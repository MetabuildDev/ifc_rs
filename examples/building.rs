use ifc4::{
    id::IdOr,
    objects::{
        application::Application,
        building::Building,
        change_action::ChangeAction,
        organization::Organization,
        owner_history::{self, OwnerHistory},
        person::Person,
        person_and_org::PersonAndOrganization,
    },
    parser::timestamp::IfcTimestamp,
    units::{
        assignment::UnitAssigment, length_unit::IfcUnitEnum, name::IfcUnitName, prefix::IfcPrefix,
        si_unit::SiUnit,
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

    let building = Building::new(
        "Building_example_id",
        owner_history,
        "building_01",
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        &mut ifc,
    );

    write("examples/building_example.ifc", ifc.to_string()).unwrap();
}

fn create_person_and_applicaton(
    ifc: &mut IFC,
    person_name: &str,
    application_name: &str,
    application_id: &str,
) -> (IdOr<Person>, IdOr<Application>) {
    let person = Person::new(person_name, person_name, None, None, None, None);
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
