use std::collections::HashSet;

use glam::{DVec2, DVec3};

use crate::prelude::*;

pub struct WindowParameter {
    pub height: f64,
    pub width: f64,
    /// Local to the attached parent
    pub placement: DVec3,
}

impl<'a, 'b> IfcWallBuilder<'a, 'b> {
    /// Creates a wall window. Also handle creation of the opening element.
    pub fn window_with_opening(
        &mut self,
        window_material: TypedId<MaterialConstituentSet>,
        window_type: TypedId<WindowType>,
        name: &str,
        window_parameter: WindowParameter,
    ) -> TypedId<Window> {
        let opening_element = self.vertical_opening(
            &format!("OpeningElementOfWindow{name}"),
            OpeningParameter {
                height: window_parameter.height,
                length: window_parameter.width,
                placement: window_parameter.placement,
            },
        );

        self.wall_window(
            window_material,
            window_type,
            opening_element,
            name,
            WindowParameter {
                height: window_parameter.height,
                width: window_parameter.width,
                placement: DVec3::new(0.0, 0.0, 0.0),
            },
        )
    }

    /// Assumes the given `opening_element` is attached to a wall
    fn wall_window(
        &mut self,
        material: TypedId<MaterialConstituentSet>,
        window_type: TypedId<WindowType>,
        opening_element: TypedId<OpeningElement>,
        name: &str,
        window_parameter: WindowParameter,
    ) -> TypedId<Window> {
        let wall_material_set_usage = self
            .storey
            .project
            .material_to_wall
            .iter()
            .find_map(|(mat, associates)| associates.is_related_to(self.wall_id).then_some(mat))
            .copied()
            .unwrap();

        // NOTE: we may want to pass this as an extra param, but for now we just center the window
        // in the opening element gap
        let window_thickness = self
            .storey
            .calculate_material_layer_set_thickness(wall_material_set_usage);

        let wall_direction = self
            .storey
            .wall_direction(self.wall_id)
            .expect("could not find wall extrude direction");

        self.storey.window(
            material,
            window_type,
            opening_element,
            name,
            window_parameter,
            window_thickness,
            wall_direction,
        )
    }
}

impl<'a, 'b> IfcSlabBuilder<'a, 'b> {
    /// Creates a slab window (e.g. for roofs). Also handle creation of the opening element.
    pub fn window_with_opening(
        &mut self,
        window_material: TypedId<MaterialConstituentSet>,
        window_type: TypedId<WindowType>,
        name: &str,
        window_parameter: WindowParameter,
    ) -> TypedId<Window> {
        let opening_element = self.opening(
            &format!("OpeningElementOfWindow{name}"),
            OpeningParameter {
                height: window_parameter.height,
                length: window_parameter.width,
                placement: window_parameter.placement,
            },
        );

        self.slab_window(
            window_material,
            window_type,
            opening_element,
            name,
            WindowParameter {
                height: window_parameter.height,
                width: window_parameter.width,
                placement: DVec3::new(0.0, 0.0, 0.0),
            },
        )
    }

    /// Assumes the given `opening_element` is attached to a slab
    fn slab_window(
        &mut self,
        material: TypedId<MaterialConstituentSet>,
        window_type: TypedId<WindowType>,
        opening_element: TypedId<OpeningElement>,
        name: &str,
        window_parameter: WindowParameter,
    ) -> TypedId<Window> {
        let slab_material_set_usage = self
            .storey
            .project
            .material_to_slab
            .iter()
            .find_map(|(mat, associates)| associates.is_related_to(self.slab_id).then_some(mat))
            .copied()
            .unwrap();

        // NOTE: we may want to pass this as an extra param, but for now we just center the window
        // in the opening element gap
        let window_thickness = self
            .storey
            .calculate_material_layer_set_thickness(slab_material_set_usage);

        let slab_direction = self
            .storey
            .slab_direction(self.slab_id)
            .expect("could not find slab extrude direction");

        self.storey.window(
            material,
            window_type,
            opening_element,
            name,
            WindowParameter {
                height: window_thickness,
                width: window_parameter.width,
                placement: window_parameter.placement,
            },
            window_parameter.height,
            slab_direction,
        )
    }
}

impl<'a> IfcStoreyBuilder<'a> {
    pub fn window_type(
        &mut self,
        name: &str,
        window_type: WindowTypeEnum,
        window_partitioning_type: WindowPartitioningTypeEnum,
    ) -> TypedId<WindowType> {
        let window_type = WindowType::new(name, window_type, window_partitioning_type)
            .owner_history(self.owner_history, &mut self.project.ifc)
            .name(name);

        let window_type_id = self.project.ifc.data.insert_new(window_type);

        self.window_type_to_window
            .insert(window_type_id, HashSet::new());

        window_type_id
    }

    fn window(
        &mut self,
        material: TypedId<MaterialConstituentSet>,
        window_type: TypedId<WindowType>,
        opening_element: TypedId<OpeningElement>,
        name: &str,
        window_parameter: WindowParameter,
        window_thickness: f64,
        direction: Direction3D,
    ) -> TypedId<Window> {
        let shape_repr = ShapeRepresentation::new(self.sub_context, &mut self.project.ifc)
            .add_item(
                ExtrudedAreaSolid::new(
                    RectangleProfileDef::new(
                        ProfileType::Area,
                        window_parameter.width,
                        window_thickness,
                    )
                    // center of the rectangle
                    .position(
                        Axis2D::new(
                            Point2D::from(DVec2::new(
                                window_parameter.width * 0.5,
                                window_thickness * 0.5,
                            )),
                            &mut self.project.ifc,
                        ),
                        &mut self.project.ifc,
                    ),
                    direction,
                    window_parameter.height,
                    &mut self.project.ifc,
                ),
                &mut self.project.ifc,
            );

        let product_shape =
            ProductDefinitionShape::new().add_representation(shape_repr, &mut self.project.ifc);

        let position = Axis3D::new(
            Point3D::from(window_parameter.placement + DVec3::new(0., 0., 0.)),
            &mut self.project.ifc,
        );
        let local_placement =
            LocalPlacement::new_relative(position, opening_element, &mut self.project.ifc);

        let window = Window::new(name)
            .owner_history(self.owner_history, &mut self.project.ifc)
            .representation(product_shape, &mut self.project.ifc)
            .object_placement(local_placement, &mut self.project.ifc);

        let window_id = self.project.ifc.data.insert_new(window);

        self.windows.insert(window_id);
        self.opening_elements_to_window
            .insert(opening_element, window_id);
        self.window_type_to_window
            .entry(window_type)
            .or_default()
            .insert(window_id);
        self.project
            .material_to_window
            .entry(material)
            .or_insert_with(|| {
                RelAssociatesMaterial::new(
                    format!("Material{material:?}ToWindows"),
                    material,
                    &mut self.project.ifc,
                )
                .owner_history(self.owner_history, &mut self.project.ifc)
            })
            .relate_push(window_id, &mut self.project.ifc);

        window_id
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use glam::DVec3;

    use crate::prelude::*;

    use super::super::test::create_builder;

    #[test]
    fn builder_windows() {
        let mut builder = create_builder();

        {
            let mut site_builder = builder.new_site("test", DVec3::ZERO);
            let mut building_builder = site_builder.new_building("test", DVec3::ZERO);
            let mut storey_builder = building_builder.new_storey("test", 0.0);

            let material_layer = storey_builder.material_layer("ExampleMaterial", 0.02, false);
            let material_layer_set = storey_builder.material_layer_set([material_layer]);
            let material_layer_set_usage = storey_builder.material_layer_set_usage(
                material_layer_set,
                LayerSetDirectionEnum::Axis2,
                DirectionSenseEnum::Positive,
                0.0,
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
            let material_constituent_set =
                storey_builder.material_constituent_set([material_constituent]);

            {
                let mut wall = storey_builder.vertical_wall(
                    material_layer_set_usage,
                    wall_type,
                    "ExampleWallDefault",
                    VerticalWallParameter {
                        height: 2.0,
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
                        placement: DVec3::new(0.0, 0.0, 0.0),
                    },
                );
            }

            drop(storey_builder);
        }

        let s = builder.build();
        let ifc = IFC::from_str(&s).unwrap();

        assert_eq!(s, ifc.to_string());
    }
}
