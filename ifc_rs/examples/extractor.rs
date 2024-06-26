use anyhow::Result;
use ifc_rs::prelude::*;

fn main() -> Result<()> {
    print_project_hierarchy(IFC::from_file("ifc_rs/resources/AC20-FZK-Haus.ifc")?);

    Ok(())
}

/// example for printing project hierarchy
/// get link from project to site
fn print_project_hierarchy(ifc: IFC) {
    let ifc = IfcExtractor::from(ifc);

    let projects = ifc.projects();

    println!("project count: {}", projects.len());

    let sites = projects
        .iter()
        .flat_map(|(id, _)| ifc.relations_of::<Project, Site>(*id))
        .collect::<Vec<_>>();

    print_sites(&ifc, sites);
}

/// get link from site to building
fn print_sites(ifc: &IfcExtractor, sites: Vec<(TypedId<Site>, &Site)>) {
    println!("site count: {}", sites.len());

    let buildings = sites
        .iter()
        .flat_map(|(id, _)| ifc.relations_of::<Site, Building>(*id))
        .collect::<Vec<_>>();

    print_buildings(ifc, buildings);
}

/// get link from building to storey
fn print_buildings(ifc: &IfcExtractor, buildings: Vec<(TypedId<Building>, &Building)>) {
    println!("building count: {}", buildings.len());

    let storeys = buildings
        .iter()
        .flat_map(|(id, _)| ifc.relations_of::<Building, Storey>(*id))
        .collect::<Vec<_>>();

    print_storeys(ifc, storeys);
}

/// get data out of storey
fn print_storeys(ifc: &IfcExtractor, storeys: Vec<(TypedId<Storey>, &Storey)>) {
    println!("storey count: {}", storeys.len());

    for (storey_index, (storey_id, _storey)) in storeys.into_iter().enumerate() {
        let spaces = ifc.relations_of::<Storey, Space>(storey_id);
        let storey_structures = ifc.contained_structures(storey_id);

        println!(
            "\tstorey {} has {} space(s) and {} related structures",
            storey_index + 1,
            spaces.len(),
            storey_structures.len()
        );

        storey_structures.iter().for_each(|id| {
            let structure_ifc_type = ifc.data.get_untyped(*id);

            if !structure_ifc_type.type_name().ends_with("Dummy") {
                println!("\t\t\tstructure name: {}", structure_ifc_type.type_name());
            }

            if let Some(structure) = structure_ifc_type.to_structure() {
                print_structure(ifc, structure, *id);
            }

            // if let Some(dummy) = structure_ifc_type.downcast_ref::<Dummy>() {
            //     println!("\t\t\t\t{}", dummy.s);
            // }
        });

        for (space_index, (space_id, _space)) in spaces.into_iter().enumerate() {
            let space_structures = ifc.contained_structures(space_id);

            println!(
                "\t\tspace {} of storey {} has {} related structures",
                space_index + 1,
                storey_index + 1,
                space_structures.len()
            );
        }
    }
}

/// print implemented structures
fn print_structure(ifc: &IfcExtractor, structure: &dyn Structure, id: Id) {
    if let Some(structure_type) = structure.structure_type() {
        match structure_type {
            StructureType::Wall(wall) => {
                let wall_id = TypedId::<Wall>::new(id);

                let wall_type = ifc
                    .related_type(wall_id)
                    .downcast_ref::<WallType>()
                    .unwrap();
                let materials = ifc.related_materials(wall_id);
                let shapes = wall.shapes(ifc);
                let items = shapes.iter().flat_map(|shape| shape.items(ifc));

                println!(
                    "{}wall with wall type {:?}, materials {}",
                    indentation(4),
                    wall_type.predefined_type,
                    materials.len(),
                );

                print_items(items, ifc, 5);

                for opening in ifc.related_voids(wall_id) {
                    let opening_shapes = opening.shapes(ifc);
                    let opening_items = opening_shapes.iter().flat_map(|shape| shape.items(ifc));

                    println!(
                        "{}opening of type {:?}",
                        indentation(5),
                        opening.predefined_type
                    );

                    print_items(opening_items, ifc, 6);
                }
            }
            StructureType::Slab(slab) => {
                let shapes = slab.shapes(ifc);
                let items = shapes.iter().flat_map(|shape| shape.items(ifc));

                print_items(items, ifc, 5);
            }
            StructureType::Roof(roof) => {
                let shapes = roof.shapes(ifc);
                let items = shapes.iter().flat_map(|shape| shape.items(ifc));

                print_items(items, ifc, 5);
            }
            StructureType::Window(window) => {
                let shapes = window.shapes(ifc);
                let items = shapes.iter().flat_map(|shape| shape.items(ifc));

                print_items(items, ifc, 5);
            }
            StructureType::Door(door) => {
                let shapes = door.shapes(ifc);
                let items = shapes.iter().flat_map(|shape| shape.items(ifc));

                print_items(items, ifc, 5);
            }
        }
    }
}

fn print_items<'a>(shapes: impl Iterator<Item = ShapeItemEnum<'a>>, ifc: &IFC, level: u32) {
    for (index, shape_item) in shapes.enumerate() {
        match shape_item {
            ShapeItemEnum::MappedItem(mapped_item) => {
                println!("{}item {} is {}", indentation(level), index, mapped_item);

                let ((origin, shape), transform) = mapped_item.mappings(ifc);

                let axis_mapping = origin.mappings(ifc);

                println!("{}origin {origin}", indentation(level + 1));
                println!(
                    "{}location {}",
                    indentation(level + 2),
                    axis_mapping.location
                );
                println!(
                    "{}local_z {:?}",
                    indentation(level + 2),
                    axis_mapping.local_z
                );
                println!(
                    "{}local_x {:?}",
                    indentation(level + 2),
                    axis_mapping.local_x
                );

                match &transform {
                    MappedTransform::Uniform(uniform) => {
                        let transform_mapping = uniform.mappings(ifc);

                        println!("{}uniform transform {transform}", indentation(level + 1));
                        println!(
                            "{}translation {:?}",
                            indentation(level + 2),
                            transform_mapping.translation
                        );
                        println!(
                            "{}x_axis {:?}",
                            indentation(level + 2),
                            transform_mapping.axis_x
                        );
                        println!(
                            "{}y_axis {:?}",
                            indentation(level + 2),
                            transform_mapping.axis_y
                        );
                        println!(
                            "{}z_axis {:?}",
                            indentation(level + 2),
                            transform_mapping.axis_z
                        );
                        println!(
                            "{}scale {:?}",
                            indentation(level + 2),
                            transform_mapping.scale
                        );
                    }
                    MappedTransform::NonUniform(non_uniform) => {
                        let non_uniform_transform_mapping = non_uniform.mappings(ifc);

                        println!(
                            "{}non uniform transform {transform}",
                            indentation(level + 1)
                        );
                        println!(
                            "{}translation {:?}",
                            indentation(level + 2),
                            non_uniform_transform_mapping.translation
                        );
                        println!(
                            "{}x_axis {:?}",
                            indentation(level + 2),
                            non_uniform_transform_mapping.axis_x
                        );
                        println!(
                            "{}y_axis {:?}",
                            indentation(level + 2),
                            non_uniform_transform_mapping.axis_y
                        );
                        println!(
                            "{}z_axis {:?}",
                            indentation(level + 2),
                            non_uniform_transform_mapping.axis_z
                        );
                        println!(
                            "{}scale {:?}",
                            indentation(level + 2),
                            non_uniform_transform_mapping.scale
                        );
                        println!(
                            "{}scale_y {:?}",
                            indentation(level + 2),
                            non_uniform_transform_mapping.scale_y
                        );
                        println!(
                            "{}scale_z {:?}",
                            indentation(level + 2),
                            non_uniform_transform_mapping.scale_z
                        );
                    }
                }

                print_items(shape.items(ifc), ifc, level + 3);
            }

            ShapeItemEnum::ExtrudedAreaSolid(extruded_area_solid) => {
                println!(
                    "{}item {} is {}",
                    indentation(level),
                    index,
                    extruded_area_solid
                );

                let mapped_extruded_area_solid = extruded_area_solid.mappings(ifc);

                println!("{}profile_def:", indentation(level + 1));

                match mapped_extruded_area_solid.profile_def {
                    MappedProfileDef::Rectangle(mapped_rectangle) => {
                        if let Some(axis) = mapped_rectangle.axis {
                            match axis {
                                AxisMappings::D2(mapped_2d_axis) => {
                                    println!(
                                        "{}axis {} {:?}",
                                        indentation(level + 2),
                                        mapped_2d_axis.location,
                                        mapped_2d_axis.local_x
                                    )
                                }
                                AxisMappings::D3(mapped_3d_axis) => println!(
                                    "{}axis {} {:?} {:?}",
                                    indentation(level + 2),
                                    mapped_3d_axis.location,
                                    mapped_3d_axis.local_z,
                                    mapped_3d_axis.local_x,
                                ),
                            }

                            println!("{}x_dim {}", indentation(level + 2), mapped_rectangle.x_dim);
                            println!("{}y_dim {}", indentation(level + 2), mapped_rectangle.y_dim);
                        }
                    }
                    MappedProfileDef::Arbitrary(mapped_arbitrary) => {
                        println!(
                            "{}profile type {}",
                            indentation(level + 2),
                            mapped_arbitrary.profile_type
                        );
                        println!(
                            "{}points {:?}",
                            indentation(level + 2),
                            mapped_arbitrary.points
                        );
                    }
                }

                println!(
                    "{}position {:?}",
                    indentation(level + 1),
                    mapped_extruded_area_solid.position
                );
                println!(
                    "{}direction {}",
                    indentation(level + 1),
                    mapped_extruded_area_solid.extruded_direction
                );
                println!(
                    "{}depth {}",
                    indentation(level + 1),
                    mapped_extruded_area_solid.depth
                );
            }

            _ => (),
            // ShapeItemEnum::Dummy(dummy) => {
            //     println!("{}item {} is {}", indentation(level), index, dummy);
            // }
            // ShapeItemEnum::Other(other) => {
            //     println!("{}item {} is {}", indentation(level), index, other);
            // }
        }
    }
}

fn indentation(level: u32) -> String {
    (0..level).map(|_| "\t").collect::<String>()
}
