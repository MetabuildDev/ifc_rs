use std::collections::HashSet;

use glam::{DVec2, DVec3};

use crate::prelude::*;

pub struct HorizontalRectSlabParameter {
    pub width: f64,
    pub height: f64,
    pub placement: DVec3,
}

pub struct HorizontalArbitrarySlabParameter {
    pub coords: Vec<DVec2>,
    pub placement: DVec3,
}

impl<'a> IfcStoreyBuilder<'a> {
    pub fn horizontal_arbitrary_slab(
        &mut self,
        material: TypedId<MaterialLayerSetUsage>,
        slab_type: TypedId<SlabType>,
        name: &str,
        slab_information: HorizontalArbitrarySlabParameter,
    ) {
        let position = Axis3D::new(Point3D::from(slab_information.placement), self.ifc);
        let slab_thickness = self.calculate_material_layer_set_thickness(material);

        let shape_repr = ShapeRepresentation::new(self.sub_context, self.ifc).add_item(
            ExtrudedAreaSolid::new(
                ArbitraryClosedProfileDef::new(
                    ProfileType::Area,
                    IndexedPolyCurve::new(
                        PointList2D::new(slab_information.coords.into_iter()),
                        self.ifc,
                    ),
                    self.ifc,
                ),
                // horizontal slab (z-up)
                Direction3D::from(DVec3::new(0.0, 0.0, 1.0)),
                slab_thickness,
                self.ifc,
            ),
            self.ifc,
        );

        let product_shape = ProductDefinitionShape::new().add_representation(shape_repr, self.ifc);
        let local_placement = LocalPlacement::new(position, self.ifc);

        let slab = Slab::new(name)
            .owner_history(self.owner_history, self.ifc)
            .object_placement(local_placement, self.ifc)
            .representation(product_shape, self.ifc);

        self.slab(material, slab_type, slab);
    }

    pub fn horizontal_rect_slab(
        &mut self,
        material: TypedId<MaterialLayerSetUsage>,
        slab_type: TypedId<SlabType>,
        name: &str,
        slab_information: HorizontalRectSlabParameter,
    ) {
        let position = Axis3D::new(Point3D::from(slab_information.placement), self.ifc);
        let slab_thickness = self.calculate_material_layer_set_thickness(material);

        let shape_repr = ShapeRepresentation::new(self.sub_context, self.ifc).add_item(
            ExtrudedAreaSolid::new(
                RectangleProfileDef::new(
                    ProfileType::Area,
                    slab_information.width,
                    slab_information.height,
                )
                // center of the rectangle
                .position(
                    Axis2D::new(
                        Point2D::from(DVec2::new(
                            slab_information.width * 0.5,
                            slab_information.height * 0.5,
                        )),
                        self.ifc,
                    ),
                    self.ifc,
                ),
                // horizontal slab (z-up)
                Direction3D::from(DVec3::new(0.0, 0.0, 1.0)),
                slab_thickness,
                self.ifc,
            ),
            self.ifc,
        );

        let product_shape = ProductDefinitionShape::new().add_representation(shape_repr, self.ifc);
        let local_placement = LocalPlacement::new(position, self.ifc);

        let slab = Slab::new(name)
            .owner_history(self.owner_history, self.ifc)
            .object_placement(local_placement, self.ifc)
            .representation(product_shape, self.ifc);

        self.slab(material, slab_type, slab);
    }

    pub fn slab_type(
        &mut self,
        material: TypedId<MaterialLayerSet>,
        name: &str,
        slab_type: SlabTypeEnum,
    ) -> TypedId<SlabType> {
        let slab_type = SlabType::new(name, slab_type)
            .owner_history(self.owner_history, self.ifc)
            .name(name);

        let slab_type_id = self.ifc.data.insert_new(slab_type);

        self.slab_type_to_slab.insert(slab_type_id, HashSet::new());
        self.material_to_slab_type
            .get_mut(&material)
            .unwrap()
            .insert(slab_type_id);

        slab_type_id
    }

    fn slab(
        &mut self,
        material: TypedId<MaterialLayerSetUsage>,
        slab_type: TypedId<SlabType>,
        slab: Slab,
    ) {
        let slab_id = self.ifc.data.insert_new(slab);

        self.slabs.insert(slab_id);
        self.slab_type_to_slab
            .get_mut(&slab_type)
            .unwrap()
            .insert(slab_id);
        self.material_to_slab
            .get_mut(&material)
            .unwrap()
            .insert(slab_id);
    }
}

#[cfg(test)]
mod test {
    use glam::DVec3;

    use crate::prelude::*;

    use super::super::test::create_builder;

    #[test]
    fn builder_slabs() {
        let mut builder = create_builder();

        {
            let mut building_builder = builder.new_building("test");
            let mut storey_builder = building_builder.new_storey("test");

            let material_layer = storey_builder.material_layer("ExampleMaterial", 0.02, false);
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

            storey_builder.horizontal_rect_slab(
                material_layer_set_usage,
                slab_type,
                "ExampleSlab",
                HorizontalRectSlabParameter {
                    width: 4.0,
                    height: 4.0,
                    placement: DVec3::new(0.0, 0.0, 0.0),
                },
            );
        }

        let s = builder.build();
        let ifc = IFC::from_str(&s).unwrap();

        assert_eq!(s, ifc.to_string());
    }
}
