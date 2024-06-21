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
            MaterialLayer::new(thickness, is_ventilated).material(material, &mut self.project.ifc);
        self.project.ifc.data.insert_new(material_layer)
    }

    pub fn material_layer_set(
        &mut self,
        layer: impl IntoIterator<Item = TypedId<MaterialLayer>>,
    ) -> TypedId<MaterialLayerSet> {
        let mut material_layer_set = MaterialLayerSet::new();

        for layer in layer {
            material_layer_set = material_layer_set.add_layer(layer, &mut self.project.ifc);
        }

        let id = self.project.ifc.data.insert_new(material_layer_set);
        self.project.material_to_wall_type.entry(id).or_default();
        self.project.material_to_slab_type.entry(id).or_default();
        self.project.material_to_roof_type.entry(id).or_default();

        id
    }

    pub fn material_layer_set_usage(
        &mut self,
        material_layer_set: TypedId<MaterialLayerSet>,
        direction: LayerSetDirectionEnum,
        sense: DirectionSenseEnum,
        offset: f64,
    ) -> TypedId<MaterialLayerSetUsage> {
        let material_layer_set_usage = MaterialLayerSetUsage::new(
            material_layer_set,
            direction,
            sense,
            offset,
            &mut self.project.ifc,
        );

        let id = self.project.ifc.data.insert_new(material_layer_set_usage);
        self.project.material_to_wall.entry(id).or_default();
        self.project.material_to_slab.entry(id).or_default();
        self.project.material_to_roof.entry(id).or_default();

        id
    }

    pub(super) fn calculate_material_layer_set_thickness(
        &self,
        material: TypedId<MaterialLayerSetUsage>,
    ) -> f64 {
        let layer_set_usage = self.project.ifc.data.get(material);
        let layer_set = self
            .project
            .ifc
            .data
            .get(layer_set_usage.spatial_element_structure);

        layer_set
            .material_layers
            .0
            .iter()
            .map(|layer_id| {
                let layer = self.project.ifc.data.get(*layer_id);
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
            MaterialConstituent::new(material, &mut self.project.ifc).name(constituent_name);
        self.project.ifc.data.insert_new(material_constituent)
    }

    pub fn material_constituent_set(
        &mut self,
        constituents: impl IntoIterator<Item = TypedId<MaterialConstituent>>,
    ) -> TypedId<MaterialConstituentSet> {
        let mut material_constituent_set = MaterialConstituentSet::new();

        for constituent in constituents {
            material_constituent_set =
                material_constituent_set.add_constituent(constituent, &mut self.project.ifc);
        }

        let id = self.project.ifc.data.insert_new(material_constituent_set);
        self.project.material_to_window.entry(id).or_default();

        id
    }

    pub fn material(&mut self, name: &str) -> TypedId<Material> {
        let material = Material::new(name);
        self.project.ifc.data.insert_new(material)
    }
}

#[cfg(test)]
mod test {
    use glam::DVec3;

    use crate::prelude::*;

    use super::super::test::create_builder;

    #[test]
    fn builder_materials() {
        let mut builder = create_builder();

        {
            let mut site_builder = builder.new_site("test", DVec3::ZERO);
            let mut building_builder = site_builder.new_building("test", DVec3::ZERO);
            let mut storey_builder = building_builder.new_storey("test", 0.0);

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
