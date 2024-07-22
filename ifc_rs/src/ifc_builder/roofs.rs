use std::collections::HashSet;

use glam::{DVec2, DVec3};

use crate::prelude::*;

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
        let roof_thickness = self.calculate_material_layer_set_thickness(material);

        let product_shape = ProductDefinitionShape::new_horizontal_arbitrary_shape(
            roof_information.coords.into_iter(),
            roof_thickness,
            self.sub_context,
            &mut self.project.ifc,
        );

        let position = Axis3D::new(
            Point3D::from(roof_information.placement),
            &mut self.project.ifc,
        );

        let local_placement =
            LocalPlacement::new_relative(position, self.storey, &mut self.project.ifc);

        let roof = Roof::new(name)
            .owner_history(self.owner_history, &mut self.project.ifc)
            .object_placement(local_placement, &mut self.project.ifc)
            .representation(product_shape, &mut self.project.ifc);

        self.roof(material, roof_type, roof);
    }

    pub fn roof_type(
        &mut self,
        material: TypedId<MaterialLayerSet>,
        name: &str,
        roof_type: RoofTypeEnum,
    ) -> TypedId<RoofType> {
        let roof_type = RoofType::new(name, roof_type)
            .owner_history(self.owner_history, &mut self.project.ifc)
            .name(name);

        let roof_type_id = self.project.ifc.data.insert_new(roof_type);

        self.roof_type_to_roof.insert(roof_type_id, HashSet::new());
        self.project
            .material_to_roof_type
            .entry(material)
            .or_insert_with(|| {
                RelAssociatesMaterial::new(
                    format!("Material{material:?}ToRoofType"),
                    material,
                    &mut self.project.ifc,
                )
                .owner_history(self.owner_history, &mut self.project.ifc)
            })
            .relate_push(roof_type_id, &mut self.project.ifc);

        roof_type_id
    }

    fn roof(
        &mut self,
        material: TypedId<MaterialLayerSetUsage>,
        roof_type: TypedId<RoofType>,
        roof: Roof,
    ) {
        let roof_id = self.project.ifc.data.insert_new(roof);

        self.roofs.insert(roof_id);
        self.roof_type_to_roof
            .entry(roof_type)
            .or_default()
            .insert(roof_id);
        self.project
            .material_to_roof
            .entry(material)
            .or_insert_with(|| {
                RelAssociatesMaterial::new(
                    format!("Material{material:?}ToRoof"),
                    material,
                    &mut self.project.ifc,
                )
                .owner_history(self.owner_history, &mut self.project.ifc)
            })
            .relate_push(roof_id, &mut self.project.ifc);
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use glam::{DVec2, DVec3};

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

            storey_builder.horizontal_arbitrary_roof(
                material_layer_set_usage,
                roof_type,
                "ExampleRoof",
                HorizontalArbitraryRoofParameter {
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
