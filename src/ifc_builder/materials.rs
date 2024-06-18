use std::collections::HashSet;

use crate::prelude::*;

impl<'a> IfcStoreyBuilder<'a> {
    pub fn material_layer(
        &mut self,
        material_name: &str,
        thickness: f64,
        is_ventilated: bool,
    ) -> TypedId<MaterialLayer> {
        let material = self.material(material_name);
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
        let layer_set_usage = self.ifc.data.get(material);
        let layer_set = self.ifc.data.get(layer_set_usage.spatial_element_structure);

        layer_set
            .material_layers
            .0
            .iter()
            .map(|layer_id| {
                let layer = self.ifc.data.get(*layer_id);
                layer.layer_thickness.0
            })
            .sum()
    }

    pub fn material_constituent(
        &mut self,
        material_name: &str,
        constituent_name: &str,
    ) -> TypedId<MaterialConstituent> {
        let material = self.material(material_name);
        let material_constituent =
            MaterialConstituent::new(material, self.ifc).name(constituent_name);
        self.ifc.data.insert_new(material_constituent)
    }

    pub fn material_constituent_set(
        &mut self,
        constituents: impl IntoIterator<Item = TypedId<MaterialConstituent>>,
    ) -> TypedId<MaterialConstituentSet> {
        let mut material_constituent_set = MaterialConstituentSet::new();

        for constituent in constituents {
            material_constituent_set =
                material_constituent_set.add_constituent(constituent, self.ifc);
        }

        let id = self.ifc.data.insert_new(material_constituent_set);
        self.material_to_window.insert(id, HashSet::new());

        id
    }

    pub fn material(&mut self, name: &str) -> TypedId<Material> {
        let material = Material::new(name);
        self.ifc.data.insert_new(material)
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
            let mut storey_builder = building_builder.new_storey("test");

            let material_layer = storey_builder.material_layer("ExampleMaterial", 0.02, false);
            let material_layer_set = storey_builder.material_layer_set([material_layer]);
            storey_builder.material_layer_set_usage(
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
