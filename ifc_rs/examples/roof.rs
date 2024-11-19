use bevy_math::{DQuat, DVec2, DVec3};
use ifc_rs::prelude::*;

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
        "IfcBuider Example Project",
    );

    let wall_height = 2.0;

    let mut site_builder = builder.new_site("ExampleSite", DVec3::ZERO);

    let mut building_builder = site_builder.new_building("ExampleBuilding", DVec3::ZERO);

    let mut storey_builder = building_builder.new_storey("ExampleStorey", 0.0);

    let material_layer = storey_builder.material_layer(
        "ExampleMaterial",
        MaterialLayer::new(0.02, false).name("ExampleMaterialLayer"),
    );
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
    let _space = storey_builder.space(
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

    let window_type = storey_builder.window_type(
        "ExampleWindowType",
        WindowTypeEnum::Window,
        WindowPartitioningTypeEnum::SinglePanel,
    );

    let material_constituent = storey_builder.material_constituent("Wood", "Framing");
    let material_constituent_set = storey_builder.material_constituent_set([material_constituent]);

    {
        let mut wall = storey_builder.vertical_wall(
            material_layer_set_usage,
            wall_type,
            "ExampleWallDefault",
            VerticalWallParameter {
                height: wall_height,
                length: 4.0,
                placement: DVec3::new(0.0, 0.0, 0.0),
            },
        );

        wall.window_with_opening(
            material_constituent_set,
            window_type,
            "ExampleWindow",
            WindowParameter {
                height: 0.5,
                width: 0.5,
                placement: DVec3::new(2.0, 0.0, 0.5),
            },
            Direction3D::from(DVec3::Z),
        );
    }

    // roof with window
    let roof_type = storey_builder.slab_type(
        material_layer_set,
        "ExampleShadingSurfaceType",
        SlabTypeEnum::Roof,
    );

    {
        let mut slab = storey_builder.horizontal_arbitrary_slab(
            material_layer_set_usage,
            roof_type,
            "SlabRoof1",
            HorizontalArbitrarySlabParameter {
                coords: vec![
                    DVec2::new(0.0, 0.0),
                    DVec2::new(4.0, 0.0),
                    DVec2::new(4.0, 4.0),
                    DVec2::new(0.0, 4.0),
                ],
                placement: DVec3::new(0.0, 0.0, 0.0),
            },
        );

        slab.window_with_opening(
            material_constituent_set,
            window_type,
            "SlabRoofWindow1",
            WindowParameter {
                height: 1.0,
                width: 1.0,
                placement: DVec3::new(2.0, 1.0, 0.0),
            },
        );

        slab.transform(
            TransformParameter::default()
                .translation(DVec3::new(0.0, 0.0, 2.0))
                .rotation(DQuat::from_rotation_x(std::f64::consts::PI * 0.25)),
        );
    }

    {
        let mut slab = storey_builder.horizontal_arbitrary_slab(
            material_layer_set_usage,
            roof_type,
            "SlabRoof2",
            HorizontalArbitrarySlabParameter {
                coords: vec![
                    DVec2::new(0.0, 0.0),
                    DVec2::new(4.0, 0.0),
                    DVec2::new(4.0, 4.0),
                    DVec2::new(0.0, 4.0),
                ],
                placement: DVec3::new(0.0, 0.0, 0.0),
            },
        );

        slab.horizontal_arbitrary_window_with_opening(
            material_constituent_set,
            window_type,
            "SlabRoofWindow2",
            HorizontalArbitraryWindowParameter {
                coords: vec![
                    DVec2::new(1.0, 1.0),
                    DVec2::new(2.0, 1.0),
                    DVec2::new(1.5, 2.0),
                ],
            },
        );

        slab.transform(
            TransformParameter::default()
                .translation(DVec3::new(4.0, 4.0, 2.0))
                .rotation(
                    DQuat::from_rotation_z(std::f64::consts::PI * 0.5)
                        * DQuat::from_rotation_x(std::f64::consts::PI * 0.25),
                ),
        );
    }

    // gable wall
    storey_builder
        .vertical_arbitrary_wall(
            material_layer_set_usage,
            wall_type,
            "ExampleGableWall",
            VerticalArbitraryWallParameter {
                coords: vec![
                    DVec2::new(0.0, 0.0),
                    DVec2::new(4.0, 0.0),
                    DVec2::new(2.0, 2.0),
                ],
                placement: DVec3::ZERO,
            },
        )
        .transform(TransformParameter::default().translation(DVec3::new(0.0, 0.0, 2.0)));

    // arbitrary roof
    {
        let mut arbitrary_roof = storey_builder.arbitrary_slab(
            material_layer_set_usage,
            roof_type,
            "ExampleArbitraryRoof",
            ArbitrarySlabParameter {
                coords: vec![
                    DVec3::new(0.0, 0.0, 0.0),
                    DVec3::new(2.0, 0.0, 2.0),
                    DVec3::new(2.0, 4.0, 2.0),
                    DVec3::new(0.0, 4.0, 0.0),
                ],
                direction: DVec3::new(-1.0, 0.0, 1.0),
                placement: DVec3::new(0.0, 0.0, 2.0),
            },
        );

        arbitrary_roof.arbitrary_window_with_opening(
            material_constituent_set,
            window_type,
            "ArbitraryRoofWindow",
            ArbitraryWindowParameter {
                coords: vec![
                    DVec3::new(0.5, 1.0, 0.5),
                    DVec3::new(1.5, 1.0, 1.5),
                    DVec3::new(1.5, 3.0, 1.5),
                    DVec3::new(0.5, 3.0, 0.5),
                ],
            },
        );
    }

    drop(storey_builder);
    drop(building_builder);
    drop(site_builder);

    std::fs::write("ifc_rs/examples/roof_example.ifc", builder.build()).unwrap();
}
