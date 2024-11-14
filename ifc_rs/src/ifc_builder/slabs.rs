use std::collections::HashSet;

use bevy_math::{DVec2, DVec3};

use crate::prelude::*;

use super::transforms::IfcBuilderTransform;

pub struct IfcSlabBuilder<'a, 'b> {
    pub(crate) storey: &'b mut IfcStoreyBuilder<'a>,

    pub(crate) slab_id: TypedId<Slab>,
    transform: Option<TransformParameter>,
}

impl<'a, 'b> IfcSlabBuilder<'a, 'b> {
    pub fn transform(&mut self, transform: TransformParameter) {
        self.transform = Some(transform);
    }
}

impl IfcSlabBuilder<'_, '_> {
    /// This finishes the builder and returns the id
    #[must_use]
    pub fn finish(self) -> TypedId<Slab> {
        self.slab_id
    }
}

impl<'a, 'b> IfcBuilderTransform for IfcSlabBuilder<'a, 'b> {
    fn ifc(&mut self) -> &mut IFC {
        &mut self.storey.project.ifc
    }

    fn sub_context(&self) -> TypedId<GeometricRepresentationSubContext> {
        self.storey.sub_context
    }
}

impl<'a, 'b> Drop for IfcSlabBuilder<'a, 'b> {
    fn drop(&mut self) {
        // clone for borrowing reasons
        if let Some(transform) = self.transform.take() {
            // apply transform to slab
            self.builder_transform(self.slab_id, &transform, (0.0, 0.0, 0.0));

            // apply transform to attached elements
            // openings
            let opening_ids = self
                .storey
                .opening_elements_to_slab
                .iter()
                .filter_map(|(opening_id, slab)| {
                    (*slab == self.slab_id)
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

            for (slab_opening_id, local_placement) in opening_ids.iter() {
                let mut transform_copy = transform.clone();
                transform_copy.translation -= *local_placement;

                self.builder_transform(*slab_opening_id, &transform_copy, *local_placement);
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

pub struct HorizontalArbitrarySlabParameter {
    pub coords: Vec<DVec2>,
    pub placement: DVec3,
}

pub struct VerticalSlabParameter {
    pub start: DVec2,
    pub end: DVec2,
    pub height: f64,
    pub placement: DVec3,
}

pub struct ArbitrarySlabParameter {
    pub coords: Vec<DVec3>,
    pub direction: DVec3,
    pub placement: DVec3,
}

impl<'a> IfcStoreyBuilder<'a> {
    pub fn horizontal_arbitrary_slab<'b>(
        &'b mut self,
        material: TypedId<MaterialLayerSetUsage>,
        slab_type: TypedId<SlabType>,
        name: &str,
        slab_information: HorizontalArbitrarySlabParameter,
    ) -> IfcSlabBuilder<'a, 'b> {
        let slab_thickness = self.calculate_material_layer_set_thickness(material);

        let product_shape = ProductDefinitionShape::new_horizontal_arbitrary_shape(
            slab_information.coords.into_iter(),
            slab_thickness,
            self.sub_context,
            &mut self.project.ifc,
        );

        self.slab(
            slab_information.placement,
            name,
            product_shape,
            material,
            slab_type,
        )
    }

    pub fn vertical_slab<'b>(
        &'b mut self,
        material: TypedId<MaterialLayerSetUsage>,
        slab_type: TypedId<SlabType>,
        name: &str,
        slab_information: VerticalSlabParameter,
    ) -> IfcSlabBuilder<'a, 'b> {
        let slab_thickness = self.calculate_material_layer_set_thickness(material);

        let direction: DVec3 =
            (slab_information.start.extend(0.0) - slab_information.end.extend(0.0)).cross(DVec3::Z);

        let product_shape = ProductDefinitionShape::new_arbitrary_shape(
            [
                slab_information.start.extend(slab_information.height),
                slab_information.start.extend(0.0),
                slab_information.end.extend(0.0),
                slab_information.end.extend(slab_information.height),
            ]
            .into_iter(),
            slab_thickness,
            direction,
            self.sub_context,
            &mut self.project.ifc,
        );

        self.slab(
            slab_information.placement,
            name,
            product_shape,
            material,
            slab_type,
        )
    }

    pub fn arbitrary_slab<'b>(
        &'b mut self,
        material: TypedId<MaterialLayerSetUsage>,
        slab_type: TypedId<SlabType>,
        name: &str,
        slab_information: ArbitrarySlabParameter,
    ) -> IfcSlabBuilder<'a, 'b> {
        let slab_thickness = self.calculate_material_layer_set_thickness(material);

        let product_shape = ProductDefinitionShape::new_arbitrary_shape(
            slab_information.coords.into_iter(),
            slab_thickness,
            slab_information.direction,
            self.sub_context,
            &mut self.project.ifc,
        );

        self.slab(
            slab_information.placement,
            name,
            product_shape,
            material,
            slab_type,
        )
    }

    pub fn slab_type(
        &mut self,
        material: TypedId<MaterialLayerSet>,
        name: &str,
        slab_type: SlabTypeEnum,
    ) -> TypedId<SlabType> {
        let slab_type = SlabType::new(name, slab_type)
            .owner_history(self.owner_history, &mut self.project.ifc)
            .name(name);

        let slab_type_id = self.project.ifc.data.insert_new(slab_type);

        self.slab_type_to_slab.insert(slab_type_id, HashSet::new());
        self.project
            .material_to_slab_type
            .entry(material)
            .or_insert_with(|| {
                RelAssociatesMaterial::new(
                    format!("Material{material:?}ToSlabType"),
                    material,
                    &mut self.project.ifc,
                )
                .owner_history(self.owner_history, &mut self.project.ifc)
            })
            .relate_push(slab_type_id, &mut self.project.ifc);

        slab_type_id
    }

    fn slab<'b>(
        &'b mut self,
        placement: DVec3,
        name: &str,
        product_shape: ProductDefinitionShape,
        material: TypedId<MaterialLayerSetUsage>,
        slab_type: TypedId<SlabType>,
    ) -> IfcSlabBuilder<'a, 'b> {
        let position = Axis3D::new(Point3D::from(placement), &mut self.project.ifc);

        let local_placement =
            LocalPlacement::new_relative(position, self.storey, &mut self.project.ifc);

        let slab = Slab::new(name)
            .owner_history(self.owner_history, &mut self.project.ifc)
            .object_placement(local_placement, &mut self.project.ifc)
            .representation(product_shape, &mut self.project.ifc);

        let slab_id = self.project.ifc.data.insert_new(slab);

        self.slabs.insert(slab_id);
        self.slab_type_to_slab
            .entry(slab_type)
            .or_default()
            .insert(slab_id);
        self.project
            .material_to_slab
            .entry(material)
            .or_insert_with(|| {
                RelAssociatesMaterial::new(
                    format!("Material{material:?}ToSlab"),
                    material,
                    &mut self.project.ifc,
                )
                .owner_history(self.owner_history, &mut self.project.ifc)
            })
            .relate_push(slab_id, &mut self.project.ifc);

        IfcSlabBuilder {
            storey: self,

            slab_id,
            transform: None,
        }
    }

    /// Tries to get the extrusion direction of this slab (not the normal)
    pub(crate) fn slab_direction(&self, slab_id: TypedId<Slab>) -> Option<Direction3D> {
        self.project
            .ifc
            .data
            .get(slab_id)
            .direction(&self.project.ifc)
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use bevy_math::{DVec2, DVec3};

    use crate::prelude::*;

    use super::super::test::create_builder;

    #[test]
    fn builder_slabs() {
        let mut builder = create_builder();

        {
            let mut site_builder = builder.new_site("test", DVec3::ZERO);
            let mut building_builder = site_builder.new_building("test", DVec3::ZERO);
            let mut storey_builder = building_builder.new_storey("test", 0.0);

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
                    coords: vec![
                        DVec2::new(0.0, 0.0),
                        DVec2::new(0.0, 1.0),
                        DVec2::new(1.0, 1.0),
                        DVec2::new(1.0, 0.0),
                    ],
                    placement: DVec3::new(0.0, 0.0, 0.0),
                },
            );
        }

        let s = builder.build();
        let ifc = IFC::from_str(&s).unwrap();

        assert_eq!(s, ifc.to_string());
    }
}
