use glam::{DQuat, DVec3};

use crate::prelude::*;

// omit scale for now
pub struct TransformParameter {
    translation: DVec3,
    x_rotation: DVec3,
    y_rotation: DVec3,
    z_rotation: DVec3,
}

impl TransformParameter {
    /// translate/offset by the given vector
    pub fn translation(mut self, translation: DVec3) -> Self {
        self.translation = translation;
        self
    }

    /// rotate from the standard X-Y-Z coordinate system to the one defined by the given rotation
    pub fn rotation(self, rotation: DQuat) -> Self {
        let [x, y, z] = [DVec3::X, DVec3::Y, DVec3::Z].map(|v| rotation * v);
        self.x_rotation(x).y_rotation(y).z_rotation(z)
    }

    /// set the local x axis direction
    pub fn x_rotation(mut self, x_rotation: DVec3) -> Self {
        if x_rotation != DVec3::ZERO {
            self.x_rotation = x_rotation;
        }

        self
    }

    /// set the local y axis direction
    pub fn y_rotation(mut self, y_rotation: DVec3) -> Self {
        if y_rotation != DVec3::ZERO {
            self.y_rotation = y_rotation;
        }

        self
    }

    /// set the local z axis direction
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

impl<'a> IfcStoreyBuilder<'a> {
    pub fn transform<T: TransformableType>(
        &mut self,
        t: TypedId<T>,
        transform_parameter: &TransformParameter,
    ) {
        let transformable = self.project.ifc.data.get(t);

        if let Some(shape_id) = transformable.shape() {
            let transform = CartesianTransformationOperator3DnonUniform::new(
                Point3D::from(transform_parameter.translation),
                (
                    Direction3D::from(transform_parameter.x_rotation),
                    Direction3D::from(transform_parameter.y_rotation),
                    Direction3D::from(transform_parameter.z_rotation),
                ),
                (1.0, 1.0, 1.0),
                &mut self.project.ifc,
            );
            let transform_id = self.project.ifc.data.insert_new(transform);

            // access to shape is still unique since we don't change it anywhere
            // else inside the following loop just afterwards
            let product_shape = self.project.ifc.data.get(shape_id);

            let transforms: Vec<_> = product_shape
                .representations
                .0
                .clone()
                .into_iter()
                .map(|shape_repr| {
                    let representation_map = RepresentationMap::new(
                        Axis3D::new(
                            Point3D::from(DVec3::new(0.0, 0.0, 0.0)),
                            &mut self.project.ifc,
                        ),
                        shape_repr,
                        &mut self.project.ifc,
                    );
                    let r = ShapeRepresentation::new(self.sub_context, &mut self.project.ifc)
                        .repr_type("MappedRepresentation")
                        .add_item(
                            MappedItem::new(
                                representation_map,
                                transform_id,
                                &mut self.project.ifc,
                            ),
                            &mut self.project.ifc,
                        );
                    self.project.ifc.data.insert_new(r)
                })
                .collect();

            self.project.ifc.data.get_mut(shape_id).representations.0 = transforms;
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use glam::DVec3;

    use crate::prelude::*;

    use super::super::test::create_builder;

    #[test]
    fn builder_transforms() {
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

            storey_builder.transform(
                wall,
                &TransformParameter::default().translation(DVec3::new(1.0, 1.0, 0.0)),
            );
        }

        let s = builder.build();
        let ifc = IFC::from_str(&s).unwrap();

        assert_eq!(s, ifc.to_string());
    }
}
