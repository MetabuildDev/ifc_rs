use glam::DVec3;
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

    {
        let mut building_builder = builder.new_building("ExampleBuilding");

        let material_layer = building_builder.material_layer("ExampleMaterial", 0.02, false);
        let material_layer_set = building_builder.material_layer_set([material_layer]);
        let material_layer_set_usage = building_builder.material_layer_set_usage(
            material_layer_set,
            LayerSetDirectionEnum::Axis2,
            DirectionSenseEnum::Positive,
            0.0,
        );

        let wall_type = building_builder.wall_type(
            material_layer_set,
            "ExampleWallType",
            WallTypeEnum::NotDefined,
        );

        building_builder.vertical_wall(
            material_layer_set_usage,
            wall_type,
            "ExampleWall",
            VerticalWallParameter {
                height: 2.0,
                length: 4.0,
                placement: DVec3::new(0.0, 0.0, 0.0),
            },
        );

        let slab_type = building_builder.slab_type(
            material_layer_set,
            "ExampleSlabType",
            SlabTypeEnum::NotDefined,
        );

        building_builder.horizontal_rect_slab(
            material_layer_set_usage,
            slab_type,
            "ExampleSlab",
            HorizontalRectSlabParameter {
                width: 4.0,
                height: 4.0,
                placement: DVec3::new(0.0, 0.0, 0.0),
            },
        );
    }

    std::fs::write("examples/builder_example.ifc", builder.build()).unwrap();
}
