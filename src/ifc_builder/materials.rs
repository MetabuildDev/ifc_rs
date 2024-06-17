use std::collections::HashSet;

use crate::prelude::*;

impl<'a> IfcBuildingBuilder<'a> {
    pub fn material_layer(
        &mut self,
        name: &str,
        thickness: f64,
        is_ventilated: bool,
    ) -> TypedId<MaterialLayer> {
        let material = self.ifc.data.insert_new(Material::new(name));

        let material_layer =
            MaterialLayer::new(thickness, is_ventilated).material(material, self.ifc);

        self.ifc.data.insert_new(material_layer)
    }

    pub fn material_layer_set(
        &mut self,
        layer: impl IntoIterator<Item = TypedId<MaterialLayer>>,
    ) -> TypedId<MaterialLayerSet> {
        let mut material_layer_set = MaterialLayerSet::new();

        for layer in layer {
            material_layer_set = material_layer_set.add_layer(layer, self.ifc);
        }

        let id = self.ifc.data.insert_new(material_layer_set);
        self.material_to_wall_type.insert(id, HashSet::new());
        self.material_to_slab_type.insert(id, HashSet::new());

        id
    }

    pub fn material_layer_set_usage(
        &mut self,
        material_layer_set: TypedId<MaterialLayerSet>,
        direction: LayerSetDirectionEnum,
        sense: DirectionSenseEnum,
        offset: f64,
    ) -> TypedId<MaterialLayerSetUsage> {
        let material_layer_set_usage =
            MaterialLayerSetUsage::new(material_layer_set, direction, sense, offset, self.ifc);

        let id = self.ifc.data.insert_new(material_layer_set_usage);
        self.material_to_wall.insert(id, HashSet::new());
        self.material_to_slab.insert(id, HashSet::new());

        id
    }

    pub(super) fn calculate_material_layer_set_thickness(
        &self,
        material: TypedId<MaterialLayerSetUsage>,
    ) -> f64 {
        let layer_set_usage = self.ifc.data.get::<MaterialLayerSetUsage>(material.id());
        let layer_set = self
            .ifc
            .data
            .get::<MaterialLayerSet>(layer_set_usage.spatial_element_structure);

        layer_set
            .material_layers
            .0
            .iter()
            .map(|layer_id| {
                let layer = self.ifc.data.get::<MaterialLayer>(*layer_id);
                layer.layer_thickness.0
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    use super::super::test::create_builder;

    #[test]
    fn builder_materials() {
        let mut builder = create_builder();

        {
            let mut building_builder = builder.new_building("test");

            let material_layer = building_builder.material_layer("ExampleMaterial", 0.02, false);
            let material_layer_set = building_builder.material_layer_set([material_layer]);
            building_builder.material_layer_set_usage(
                material_layer_set,
                LayerSetDirectionEnum::Axis2,
                DirectionSenseEnum::Positive,
                0.0,
            );
        }

        let s = builder.build();
        let ifc = IFC::from_str(&s).unwrap();

        assert_eq!(s, ifc.to_string());
    }
}