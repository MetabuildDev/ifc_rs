use ifc4::prelude::*;

fn main() {
    let mut builder = IfcBuilder::new(
        ApplicationInfo {
            developer: Person::empty().given_name("Mario"),
            version: "0.0.1",
            name: "IfcBuilderApplication",
            short_name: "builder",
        },
        OwnerInfo {
            owner: Person::empty().given_name("Luigi"),
            organization_name: "Metabuild",
        },
        Person::empty().given_name("Bowser"),
        "IfcBuider Example Project",
    );

    let mut building_builder = builder.new_building("ExampleBuilding");

    builder.build("examples/builder.ifc").unwrap();
}
