use std::collections::HashSet;

use glam::{DVec2, DVec3};

use crate::prelude::*;

pub struct HorizontalRectRoofParameter {
    pub width: f64,
    pub height: f64,
    pub placement: DVec3,
}

pub struct HorizontalArbitraryRoofParameter {
    pub coords: Vec<DVec2>,
    pub placement: DVec3,
}

impl<'a> IfcStoreyBuilder<'a> {
    pub fn horizontal_arbitrary_roof(
        &mut self,
        material: TypedId<MaterialLayerSetUsage>,
        roof_type: TypedId<RoofType>,
        name: &str,
        roof_information: HorizontalArbitraryRoofParameter,
    ) {
        let position = Axis3D::new(Point3D::from(roof_information.placement), self.ifc);
        let roof_thickness = self.calculate_material_layer_set_thickness(material);

        let shape_repr = ShapeRepresentation::new(self.sub_context, self.ifc).add_item(
            ExtrudedAreaSolid::new(
                ArbitraryClosedProfileDef::new(
                    ProfileType::Area,
                    IndexedPolyCurve::new(
                        PointList2D::new(roof_information.coords.into_iter()),
                        self.ifc,
                    ),
                    self.ifc,
                ),
                // horizontal roof (z-up)
                Direction3D::from(DVec3::new(0.0, 0.0, 1.0)),
                roof_thickness,
                self.ifc,
            ),
            self.ifc,
        );

        let product_shape = ProductDefinitionShape::new().add_representation(shape_repr, self.ifc);
        let local_placement = LocalPlacement::new(position, self.ifc);

        let roof = Roof::new(name)
            .owner_history(self.owner_history, self.ifc)
            .object_placement(local_placement, self.ifc)
            .representation(product_shape, self.ifc);

        self.roof(material, roof_type, roof);
    }

    pub fn horizontal_rect_roof(
        &mut self,
        material: TypedId<MaterialLayerSetUsage>,
        roof_type: TypedId<RoofType>,
        name: &str,
        roof_information: HorizontalRectRoofParameter,
    ) {
        let position = Axis3D::new(Point3D::from(roof_information.placement), self.ifc);
        let roof_thickness = self.calculate_material_layer_set_thickness(material);

        let shape_repr = ShapeRepresentation::new(self.sub_context, self.ifc).add_item(
            ExtrudedAreaSolid::new(
                RectangleProfileDef::new(
                    ProfileType::Area,
                    roof_information.width,
                    roof_information.height,
                )
                // center of the rectangle
                .position(
                    Axis2D::new(
                        Point2D::from(DVec2::new(
                            roof_information.width * 0.5,
                            roof_information.height * 0.5,
                        )),
                        self.ifc,
                    ),
                    self.ifc,
                ),
                // horizontal roof (z-up)
                Direction3D::from(DVec3::new(0.0, 0.0, 1.0)),
                roof_thickness,
                self.ifc,
            ),
            self.ifc,
        );

        let product_shape = ProductDefinitionShape::new().add_representation(shape_repr, self.ifc);
        let local_placement = LocalPlacement::new(position, self.ifc);

        let roof = Roof::new(name)
            .owner_history(self.owner_history, self.ifc)
            .object_placement(local_placement, self.ifc)
            .representation(product_shape, self.ifc);

        self.roof(material, roof_type, roof);
    }

    pub fn roof_type(
        &mut self,
        material: TypedId<MaterialLayerSet>,
        name: &str,
        roof_type: RoofTypeEnum,
    ) -> TypedId<RoofType> {
        let roof_type = RoofType::new(name, roof_type)
            .owner_history(self.owner_history, self.ifc)
            .name(name);

        let roof_type_id = self.ifc.data.insert_new(roof_type);

        self.roof_type_to_roof.insert(roof_type_id, HashSet::new());
        self.material_to_roof_type
            .get_mut(&material)
            .unwrap()
            .insert(roof_type_id);

        roof_type_id
    }

    fn roof(
        &mut self,
        material: TypedId<MaterialLayerSetUsage>,
        roof_type: TypedId<RoofType>,
        roof: Roof,
    ) {
        let roof_id = self.ifc.data.insert_new(roof);

        self.roofs.insert(roof_id);
        self.roof_type_to_roof
            .get_mut(&roof_type)
            .unwrap()
            .insert(roof_id);
        self.material_to_roof
            .get_mut(&material)
            .unwrap()
            .insert(roof_id);
    }
}

#[cfg(test)]
mod test {
    use glam::DVec3;

    use crate::ifc_builder::roofs::HorizontalRectRoofParameter;
    use crate::prelude::*;

    use super::super::test::create_builder;

    #[test]
    fn builder_roofs() {
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

            let roof_type = storey_builder.roof_type(
                material_layer_set,
                "ExampleRoofType",
                RoofTypeEnum::FlatRoof,
            );

            storey_builder.horizontal_rect_roof(
                material_layer_set_usage,
                roof_type,
                "ExampleRoof",
                HorizontalRectRoofParameter {
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
