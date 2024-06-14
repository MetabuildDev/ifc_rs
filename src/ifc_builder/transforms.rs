use glam::DVec3;

use crate::prelude::*;

// omit scale for now
pub struct TransformParameter {
    translation: DVec3,
    x_rotation: DVec3,
    y_rotation: DVec3,
    z_rotation: DVec3,
}

impl TransformParameter {
    pub fn translation(mut self, translation: DVec3) -> Self {
        self.translation = translation;
        self
    }

    pub fn x_rotation(mut self, x_rotation: DVec3) -> Self {
        if x_rotation != DVec3::ZERO {
            self.x_rotation = x_rotation;
        }

        self
    }

    pub fn y_rotation(mut self, y_rotation: DVec3) -> Self {
        if y_rotation != DVec3::ZERO {
            self.y_rotation = y_rotation;
        }

        self
    }

    pub fn z_rotation(mut self, z_rotation: DVec3) -> Self {
        if z_rotation != DVec3::ZERO {
            self.z_rotation = z_rotation;
        }

        self
    }
}

impl Default for TransformParameter {
    fn default() -> Self {
        Self {
            translation: DVec3::ZERO,
            x_rotation: DVec3::X,
            y_rotation: DVec3::Y,
            z_rotation: DVec3::Z,
        }
    }
}

impl<'a> IfcBuildingBuilder<'a> {
    pub fn transform<T: TransformableType>(
        &mut self,
        t: TypedId<T>,
        transform_parameter: &TransformParameter,
    ) {
        let transformable = self.ifc.data.get::<T>(t.id());

        if let Some(shape_id) = transformable.shape() {
            let transform = CartesianTransformationOperator3DnonUniform::new(
                Direction3D::from(transform_parameter.x_rotation),
                Direction3D::from(transform_parameter.y_rotation),
                Point3D::from(transform_parameter.translation),
                1.0,
                Direction3D::from(transform_parameter.z_rotation),
                1.0,
                1.0,
                self.ifc,
            );
            let transform_id = self.ifc.data.insert_new(transform);

            // access to shape is still unique since we don't change it anywhere
            // else inside the following loop just afterwards
            let product_shape = unsafe {
                self.ifc
                    .data
                    .get_mut_unchecked::<ProductDefinitionShape>(shape_id.id())
            };

            let transforms: Vec<_> = product_shape
                .representations
                .0
                .iter()
                .map(|shape_repr| {
                    let representation_map = RepresentationMap::new(
                        Axis3D::new(Point3D::from(DVec3::new(0.0, 0.0, 0.0)), self.ifc),
                        *shape_repr,
                        self.ifc,
                    );
                    let r = ShapeRepresentation::new(self.sub_context, self.ifc)
                        .repr_type("MappedRepresentation")
                        .add_item(
                            MappedItem::new(representation_map, transform_id, self.ifc),
                            self.ifc,
                        );
                    self.ifc.data.insert_new(r).id()
                })
                .collect();

            product_shape.representations.0 = transforms;
        }
    }
}

#[cfg(test)]
mod test {
    use glam::DVec3;

    use crate::prelude::*;

    use super::super::test::create_builder;

    #[test]
    fn builder_transforms() {
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

            building_builder.transform(
                wall,
                &TransformParameter::default().translation(DVec3::new(1.0, 1.0, 0.0)),
            );
        }

        let s = builder.build();
        let ifc = IFC::from_str(&s).unwrap();

        assert_eq!(s, ifc.to_string());
    }
}
