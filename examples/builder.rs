use glam::{DVec2, DVec3};
use ifc4::ifc_builder::spaces::SpaceParameter;
use ifc4::ifc_builder::windows::WindowParameter;
use ifc4::prelude::*;

fn main() {
    let mut builder = IfcProjectBuilder::new(
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
        let mut site_builder = builder.new_site("ExampleSite", DVec3::ZERO);
        let mut building_builder = site_builder.new_building("ExampleBuilding", DVec3::ZERO);
        let mut mk_storey = |elevation: f64, wall_height: f64| {
            let mut storey_builder = building_builder.new_storey("ExampleStorey", elevation);

            let material_layer = storey_builder.material_layer("ExampleMaterial", 0.02, false);
            let material_layer_set = storey_builder.material_layer_set([material_layer]);
            let material_layer_set_usage = storey_builder.material_layer_set_usage(
                material_layer_set,
                LayerSetDirectionEnum::Axis2,
                DirectionSenseEnum::Positive,
                0.0,
            );

            let story_footprint = vec![
                DVec2::ZERO,
                DVec2::new(0.0, 4.0),
                DVec2::new(2.0, 6.0),
                DVec2::new(4.0, 4.0),
                DVec2::new(4.0, 0.0),
                DVec2::ZERO,
            ];

            let space_type = storey_builder.space_type("ExampleWallType", SpaceTypeEnum::Space);
            storey_builder.space(
                space_type,
                "ExampleSpaceDefault",
                SpaceParameter {
                    coords: story_footprint.clone(),
                    height: wall_height,
                    placement: DVec3::new(0.0, 0.0, 0.0),
                },
            );

            let wall_type = storey_builder.wall_type(
                material_layer_set,
                "ExampleWallType",
                WallTypeEnum::NotDefined,
            );

            let wall = storey_builder.vertical_wall(
                material_layer_set_usage,
                wall_type,
                "ExampleWallDefault",
                VerticalWallParameter {
                    height: wall_height,
                    length: 4.0,
                    placement: DVec3::new(0.0, 0.0, 0.0),
                },
            );

            let slab_type = storey_builder.slab_type(
                material_layer_set,
                "ExampleSlabType",
                SlabTypeEnum::NotDefined,
            );

            storey_builder.horizontal_arbitrary_slab(
                material_layer_set_usage,
                slab_type,
                "ExampleSlab",
                HorizontalArbitrarySlabParameter {
                    coords: story_footprint.clone(),
                    placement: DVec3::new(0.0, 0.0, 0.0),
                },
            );

            let window_type = storey_builder.window_type(
                "ExampleWindowType",
                WindowTypeEnum::Window,
                WindowPartitioningTypeEnum::SinglePanel,
            );

            let material_constituent = storey_builder.material_constituent("Wood", "Framing");
            let material_constituent_set =
                storey_builder.material_constituent_set([material_constituent]);

            storey_builder.wall_window_with_opening(
                material_constituent_set,
                window_type,
                wall,
                "ExampleWindow",
                WindowParameter {
                    height: 0.5,
                    width: 0.5,
                    placement: DVec3::new(2.0, 0.0, 0.5),
                },
            );

            let roof_type = storey_builder.roof_type(
                material_layer_set,
                "ExampleRoofType",
                RoofTypeEnum::FlatRoof,
            );

            storey_builder.horizontal_arbitrary_roof(
                material_layer_set_usage,
                roof_type,
                "ExampleRoof",
                HorizontalArbitraryRoofParameter {
                    coords: story_footprint,
                    placement: DVec3::new(0.0, 0.0, wall_height),
                },
            );
        };

        let wall_height = 2.0;
        mk_storey(0.0, wall_height);
        mk_storey(2.0, wall_height);
    }

    std::fs::write("examples/builder_example.ifc", builder.build()).unwrap();
}
