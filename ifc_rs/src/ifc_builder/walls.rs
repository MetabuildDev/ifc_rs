use bevy_math::{DVec2, DVec3};

use crate::prelude::*;

use super::transforms::IfcBuilderTransform;

pub struct IfcWallBuilder<'a, 'b> {
    pub(crate) storey: &'b mut IfcStoreyBuilder<'a>,

    pub(crate) wall_id: TypedId<Wall>,
    transform: Option<TransformParameter>,
}

impl<'a, 'b> IfcWallBuilder<'a, 'b> {
    pub fn transform(&mut self, transform: TransformParameter) {
        self.transform = Some(transform);
    }
}

impl<'a, 'b> IfcBuilderTransform for IfcWallBuilder<'a, 'b> {
    fn ifc(&mut self) -> &mut IFC {
        &mut self.storey.project.ifc
    }

    fn sub_context(&self) -> TypedId<GeometricRepresentationSubContext> {
        self.storey.sub_context
    }
}

impl<'a, 'b> Drop for IfcWallBuilder<'a, 'b> {
    fn drop(&mut self) {
        // clone for borrowing reasons
        if let Some(transform) = self.transform.take() {
            // apply transform to wall
            self.builder_transform(self.wall_id, &transform, (0.0, 0.0, 0.0));

            // apply transform to attached elements
            // openings
            let opening_ids = self
                .storey
                .opening_elements_to_wall
                .iter()
                .filter_map(|(opening_id, wall)| {
                    (*wall == self.wall_id)
                        .then_some(
                            self.storey
                                .project
                                .ifc
                                .data
                                .get(*opening_id)
                                .local_placement(&self.storey.project.ifc)
                                .map(|local_placement| (*opening_id, local_placement.0 .0)),
                        )
                        .flatten()
                })
                .collect::<Vec<_>>();

            for (wall_opening_id, local_placement) in opening_ids.iter() {
                let mut transform_copy = transform.clone();
                transform_copy.translation -= *local_placement;

                self.builder_transform(*wall_opening_id, &transform_copy, *local_placement);
            }

            // windows
            let window_ids = opening_ids
                .iter()
                .filter_map(|(opening_id, local_placement)| {
                    self.storey
                        .opening_elements_to_window
                        .get(opening_id)
                        .map(|window_id| (*window_id, local_placement))
                })
                .collect::<Vec<_>>();

            for (window_id, local_placement) in window_ids {
                let mut transform_copy = transform.clone();
                transform_copy.translation -= *local_placement;

                self.builder_transform(window_id, &transform_copy, *local_placement);
            }
        }
    }
}

pub struct VerticalWallParameter {
    pub height: f64,
    pub length: f64,
    pub placement: DVec3,
}

pub struct VerticalArbitraryWallParameter {
    /// the y component is reinterpreted as z component
    pub coords: Vec<DVec2>,
    pub placement: DVec3,
}

pub struct ArbitraryWallParameter {
    pub coords: Vec<DVec3>,
    pub direction: DVec3,
    pub placement: DVec3,
}

impl<'a> IfcStoreyBuilder<'a> {
    pub fn vertical_wall<'b>(
        &'b mut self,
        material: TypedId<MaterialLayerSetUsage>,
        wall_type: TypedId<WallType>,
        name: &str,
        wall_information: VerticalWallParameter,
    ) -> IfcWallBuilder<'a, 'b> {
        let wall_thickness = self.calculate_material_layer_set_thickness(material);

        let product_shape = ProductDefinitionShape::new_rectangular_shape(
            wall_information.length,
            wall_information.height,
            wall_thickness,
            Direction3D::from(DVec3::Z),
            self.sub_context,
            &mut self.project.ifc,
        );

        let position = Axis3D::new(
            Point3D::from(wall_information.placement),
            &mut self.project.ifc,
        );

        let local_placement =
            LocalPlacement::new_relative(position, self.storey, &mut self.project.ifc);

        let wall = Wall::new(name)
            .owner_history(self.owner_history, &mut self.project.ifc)
            .object_placement(local_placement, &mut self.project.ifc)
            .representation(product_shape, &mut self.project.ifc);

        self.wall(material, wall_type, wall)
    }

    pub fn vertical_arbitrary_wall<'b>(
        &'b mut self,
        material: TypedId<MaterialLayerSetUsage>,
        wall_type: TypedId<WallType>,
        name: &str,
        wall_information: VerticalArbitraryWallParameter,
    ) -> IfcWallBuilder<'a, 'b> {
        let wall_thickness = self.calculate_material_layer_set_thickness(material);

        let product_shape = ProductDefinitionShape::new_vertical_arbitrary_shape(
            wall_information.coords.into_iter(),
            wall_thickness,
            self.sub_context,
            &mut self.project.ifc,
        );

        let position = Axis3D::new(
            Point3D::from(wall_information.placement),
            &mut self.project.ifc,
        );

        let local_placement =
            LocalPlacement::new_relative(position, self.storey, &mut self.project.ifc);

        let wall = Wall::new(name)
            .owner_history(self.owner_history, &mut self.project.ifc)
            .object_placement(local_placement, &mut self.project.ifc)
            .representation(product_shape, &mut self.project.ifc);

        self.wall(material, wall_type, wall)
    }

    pub fn arbitrary_wall<'b>(
        &'b mut self,
        material: TypedId<MaterialLayerSetUsage>,
        wall_type: TypedId<WallType>,
        name: &str,
        wall_information: ArbitraryWallParameter,
    ) -> IfcWallBuilder<'a, 'b> {
        let wall_thickness = self.calculate_material_layer_set_thickness(material);

        let product_shape = ProductDefinitionShape::new_arbitrary_shape(
            wall_information.coords.into_iter(),
            wall_thickness,
            wall_information.direction,
            self.sub_context,
            &mut self.project.ifc,
        );

        let position = Axis3D::new(
            Point3D::from(wall_information.placement),
            &mut self.project.ifc,
        );

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

    fn wall<'b>(
        &'b mut self,
        material: TypedId<MaterialLayerSetUsage>,
        wall_type: TypedId<WallType>,
        wall: Wall,
    ) -> IfcWallBuilder<'a, 'b> {
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

        IfcWallBuilder {
            storey: self,

            wall_id,
            transform: None,
        }
    }

    /// Tries to get the extrusion direction of this slab (not the normal)
    pub(crate) fn wall_direction(&self, wall_id: TypedId<Wall>) -> Option<Direction3D> {
        self.project
            .ifc
            .data
            .get(wall_id)
            .direction(&self.project.ifc)
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use bevy_math::DVec3;

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
