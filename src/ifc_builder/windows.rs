use std::collections::HashSet;

use glam::{DVec2, DVec3};

use crate::prelude::openings::VerticalOpeningParameter;
use crate::prelude::*;

pub struct WindowParameter {
    pub height: f64,
    pub width: f64,
    /// Local to the attached parent
    pub placement: DVec3,
}

impl<'a> IfcBuildingBuilder<'a> {
    pub fn window_type(
        &mut self,
        name: &str,
        window_type: WindowTypeEnum,
        window_partitioning_type: WindowPartitioningTypeEnum,
    ) -> TypedId<WindowType> {
        let window_type = WindowType::new(name, window_type, window_partitioning_type)
            .owner_history(self.owner_history, self.ifc)
            .name(name);

        let window_type_id = self.ifc.data.insert_new(window_type);

        self.window_type_to_window
            .insert(window_type_id, HashSet::new());

        window_type_id
    }

    /// Assumes the given `opening_element` is attached to a wall
    pub fn wall_window(
        &mut self,
        material: TypedId<MaterialConstituentSet>,
        window_type: TypedId<WindowType>,
        opening_element: TypedId<OpeningElement>,
        name: &str,
        window_parameter: WindowParameter,
    ) -> TypedId<Window> {
        let wall = self.opening_elements_to_wall.get(&opening_element).unwrap();
        let wall_material_set_usage = *self
            .material_to_wall
            .iter()
            .find_map(|(mat, walls)| walls.contains(&wall).then_some(mat))
            .unwrap();
        // NOTE: we may want to pass this as an extra param, but for now we just center the window
        // in the opening element gap
        let window_thickness =
            self.calculate_material_layer_set_thickness(wall_material_set_usage) / 3.0;

        let shape_repr = ShapeRepresentation::new(self.sub_context, self.ifc).add_item(
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
                        self.ifc,
                    ),
                    self.ifc,
                ),
                Direction3D::from(DVec3::new(0.0, 0.0, 1.0)),
                window_parameter.height,
                self.ifc,
            ),
            self.ifc,
        );

        let product_shape = ProductDefinitionShape::new().add_representation(shape_repr, self.ifc);

        let mut window = Window::new(name)
            .owner_history(self.owner_history, self.ifc)
            .representation(product_shape, self.ifc);

        if let Some(&opening_element_placement_id) =
            self.ifc.data.get(opening_element).object_placement.custom()
        {
            let position = Axis3D::new(
                Point3D::from(window_parameter.placement + DVec3::new(0., window_thickness, 0.)),
                self.ifc,
            );
            let local_placement = LocalPlacement::new(position, self.ifc)
                .relative_to::<Axis3D>(opening_element_placement_id, self.ifc);

            window = window.object_placement(local_placement, self.ifc);
        }

        let window_id = self.ifc.data.insert_new(window);

        self.windows.insert(window_id);
        self.opening_elements_to_window
            .insert(opening_element, window_id);
        self.window_type_to_window
            .get_mut(&window_type)
            .unwrap()
            .insert(window_id);
        self.material_to_window
            .get_mut(&material)
            .unwrap()
            .insert(window_id);

        window_id
    }

    /// Creates a wall window. Also handle creation of the opening element.
    pub fn wall_window_with_opening(
        &mut self,
        window_material: TypedId<MaterialConstituentSet>,
        window_type: TypedId<WindowType>,
        wall: TypedId<Wall>,
        name: &str,
        window_parameter: WindowParameter,
    ) -> TypedId<Window> {
        let opening_element = self.vertical_wall_opening(
            wall,
            &format!("OpeningElementOfWindow{name}"),
            VerticalOpeningParameter {
                height: window_parameter.height,
                length: window_parameter.width,
                placement: window_parameter.placement,
            },
        );

        let window_id = self.wall_window(
            window_material,
            window_type,
            opening_element,
            name,
            WindowParameter {
                height: 0.5,
                width: 0.5,
                placement: DVec3::new(0.0, 0.0, 0.0),
            },
        );

        window_id
    }
}

#[cfg(test)]
mod test {
    use glam::DVec3;

    use crate::prelude::openings::VerticalOpeningParameter;
    use crate::prelude::windows::WindowParameter;
    use crate::prelude::*;

    use super::super::test::create_builder;

    #[test]
    fn builder_windows() {
        let mut builder = create_builder();

        {
            let mut building_builder = builder.new_building("test");

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

            let wall = building_builder.vertical_wall(
                material_layer_set_usage,
                wall_type,
                "ExampleWallDefault",
                VerticalWallParameter {
                    height: 2.0,
                    length: 4.0,
                    placement: DVec3::new(0.0, 0.0, 0.0),
                },
            );

            let opening_element = building_builder.vertical_wall_opening(
                wall,
                "ExampleOpeningElement",
                VerticalOpeningParameter {
                    height: 0.5,
                    length: 0.5,
                    placement: DVec3::new(2.0, 0.0, 0.5),
                },
            );

            let window_type = building_builder.window_type(
                "ExampleWindowType",
                WindowTypeEnum::Window,
                WindowPartitioningTypeEnum::SinglePanel,
            );

            let material_constituent = building_builder.material_constituent("Wood", "Framing");
            let material_constituent_set =
                building_builder.material_constituent_set([material_constituent]);

            building_builder.wall_window(
                material_constituent_set,
                window_type,
                opening_element,
                "ExampleWindow",
                WindowParameter {
                    height: 0.5,
                    width: 0.5,
                    placement: DVec3::new(0.0, 0.0, 0.0),
                },
            );
        }

        let s = builder.build();
        let ifc = IFC::from_str(&s).unwrap();

        assert_eq!(s, ifc.to_string());
    }
}
