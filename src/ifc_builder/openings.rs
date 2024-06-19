use glam::{DVec2, DVec3};

use crate::prelude::*;

pub struct VerticalOpeningParameter {
    pub height: f64,
    pub length: f64,
    /// Local to the attached parent
    pub placement: DVec3,
}

impl<'a> IfcStoreyBuilder<'a> {
    pub fn vertical_wall_opening(
        &mut self,
        wall: TypedId<Wall>,
        name: &str,
        opening_information: VerticalOpeningParameter,
    ) -> TypedId<OpeningElement> {
        let position = Axis3D::new(Point3D::from(opening_information.placement), self.ifc);
        let wall_material_set_usage = *self
            .material_to_wall
            .iter()
            .find_map(|(mat, walls)| walls.contains(&wall).then_some(mat))
            .unwrap();
        let opening_thickness =
            self.calculate_material_layer_set_thickness(wall_material_set_usage);

        let shape_repr = ShapeRepresentation::new(self.sub_context, self.ifc).add_item(
            ExtrudedAreaSolid::new(
                RectangleProfileDef::new(
                    ProfileType::Area,
                    opening_information.length,
                    opening_thickness,
                )
                // center of the rectangle
                .position(
                    Axis2D::new(
                        Point2D::from(DVec2::new(
                            opening_information.length * 0.5,
                            opening_thickness * 0.5,
                        )),
                        self.ifc,
                    ),
                    self.ifc,
                ),
                // vertical wall opening (z-up)
                Direction3D::from(DVec3::new(0.0, 0.0, 1.0)),
                opening_information.height,
                self.ifc,
            ),
            self.ifc,
        );

        let product_shape = ProductDefinitionShape::new().add_representation(shape_repr, self.ifc);

        let mut opening_element = OpeningElement::new(name)
            .owner_history(self.owner_history, self.ifc)
            .representation(product_shape, self.ifc);

        if let Some(&wall_placement_id) = self.ifc.data.get::<Wall>(wall).object_placement.custom()
        {
            let local_placement =
                LocalPlacement::new(position, self.ifc).relative_to(wall_placement_id, self.ifc);

            opening_element = opening_element.object_placement(local_placement, self.ifc);
        }

        let opening_element_id = self.ifc.data.insert_new(opening_element);

        self.opening_elements.insert(opening_element_id);
        self.opening_elements_to_wall
            .insert(opening_element_id, wall);

        opening_element_id
    }
}

#[cfg(test)]
mod test {
    use glam::DVec3;

    use crate::prelude::*;

    use super::super::test::create_builder;

    #[test]
    fn builder_openings() {
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

            let wall = storey_builder.vertical_wall(
                material_layer_set_usage,
                wall_type,
                "ExampleWallDefault",
                VerticalWallParameter {
                    height: 2.0,
                    length: 4.0,
                    placement: DVec3::new(0.0, 0.0, 0.0),
                },
            );

            storey_builder.vertical_wall_opening(
                wall,
                "ExampleOpeningElement",
                VerticalOpeningParameter {
                    height: 0.5,
                    length: 0.5,
                    placement: DVec3::new(2.0, 0.0, 0.5),
                },
            );
        }

        let s = builder.build();
        let ifc = IFC::from_str(&s).unwrap();

        assert_eq!(s, ifc.to_string());
    }
}
