use glam::{DVec2, DVec3};

use crate::prelude::*;

pub struct VerticalWallParameter {
    pub height: f64,
    pub length: f64,
    pub placement: DVec3,
}

impl<'a> IfcStoreyBuilder<'a> {
    pub fn vertical_wall(
        &mut self,
        material: TypedId<MaterialLayerSetUsage>,
        wall_type: TypedId<WallType>,
        name: &str,
        wall_information: VerticalWallParameter,
    ) -> TypedId<Wall> {
        let position = Axis3D::new(
            Point3D::from(wall_information.placement),
            &mut self.project.ifc,
        );
        let wall_thickness = self.calculate_material_layer_set_thickness(material);

        let shape_repr = ShapeRepresentation::new(self.sub_context, &mut self.project.ifc)
            .add_item(
                ExtrudedAreaSolid::new(
                    RectangleProfileDef::new(
                        ProfileType::Area,
                        wall_information.length,
                        wall_thickness,
                    )
                    // center of the rectangle
                    .position(
                        Axis2D::new(
                            Point2D::from(DVec2::new(
                                wall_information.length * 0.5,
                                wall_thickness * 0.5,
                            )),
                            &mut self.project.ifc,
                        ),
                        &mut self.project.ifc,
                    ),
                    // vertical wall (z-up)
                    Direction3D::from(DVec3::new(0.0, 0.0, 1.0)),
                    wall_information.height,
                    &mut self.project.ifc,
                ),
                &mut self.project.ifc,
            );

        let product_shape =
            ProductDefinitionShape::new().add_representation(shape_repr, &mut self.project.ifc);
        let local_placement =
            LocalPlacement::new_relative(position, self.storey, &mut self.project.ifc);

        let wall = Wall::new(name)
            .owner_history(self.owner_history, &mut self.project.ifc)
            .object_placement(local_placement, &mut self.project.ifc)
            .representation(product_shape, &mut self.project.ifc);

        self.wall(material, wall_type, wall)
    }

    pub fn wall_type(
        &mut self,
        material: TypedId<MaterialLayerSet>,
        name: &str,
        wall_type: WallTypeEnum,
    ) -> TypedId<WallType> {
        let wall_type = WallType::new(name, wall_type)
            .owner_history(self.owner_history, &mut self.project.ifc)
            .name(name);

        let wall_type_id = self.project.ifc.data.insert_new(wall_type);

        self.wall_type_to_wall.entry(wall_type_id).or_default();
        self.project
            .material_to_wall_type
            .entry(material)
            .or_insert_with(|| {
                RelAssociatesMaterial::new(
                    format!("Material{material:?}ToWallType"),
                    material,
                    &mut self.project.ifc,
                )
                .owner_history(self.owner_history, &mut self.project.ifc)
            })
            .relate_push(wall_type_id, &mut self.project.ifc);

        wall_type_id
    }

    fn wall(
        &mut self,
        material: TypedId<MaterialLayerSetUsage>,
        wall_type: TypedId<WallType>,
        wall: Wall,
    ) -> TypedId<Wall> {
        let wall_id = self.project.ifc.data.insert_new(wall);

        self.walls.insert(wall_id);
        self.wall_type_to_wall
            .entry(wall_type)
            .or_default()
            .insert(wall_id);
        self.project
            .material_to_wall
            .entry(material)
            .or_insert_with(|| {
                RelAssociatesMaterial::new(
                    format!("Material{material:?}ToWalls"),
                    material,
                    &mut self.project.ifc,
                )
                .owner_history(self.owner_history, &mut self.project.ifc)
            })
            .relate_push(wall_id, &mut self.project.ifc);

        wall_id
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use glam::DVec3;

    use crate::prelude::*;

    use super::super::test::create_builder;

    #[test]
    fn builder_walls() {
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

            storey_builder.vertical_wall(
                material_layer_set_usage,
                wall_type,
                "ExampleWallDefault",
                VerticalWallParameter {
                    height: 2.0,
                    length: 4.0,
                    placement: DVec3::new(0.0, 0.0, 0.0),
                },
            );
        }

        let s = builder.build();
        let ifc = IFC::from_str(&s).unwrap();

        assert_eq!(s, ifc.to_string());
    }
}
