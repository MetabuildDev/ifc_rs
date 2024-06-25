pub mod prelude;

use std::ops::Deref;

use crate::{ifc_type::IfcType, prelude::*};

pub struct IfcExtractor {
    ifc: IFC,
}

impl IfcExtractor {
    pub fn projects(&self) -> Vec<(TypedId<Project>, &Project)> {
        self.ifc.data.find_all_of_type::<Project>().collect()
    }

    pub fn relations_of<RELATING, RELATED>(
        &self,
        id: TypedId<RELATING>,
    ) -> Vec<(TypedId<RELATED>, &RELATED)>
    where
        RELATING: IfcType,
        RELATED: IfcType,
    {
        self.ifc
            .data
            .find_all_of_type::<RelAggregates>()
            .filter(|&(_, rel_aggregate)| (rel_aggregate.relating_object == id.id()))
            .flat_map(|(_, rel_aggregate)| {
                rel_aggregate.related_objects.0.iter().filter_map(|id| {
                    self.ifc
                        .data
                        .get_untyped(*id)
                        .downcast_ref::<RELATED>()
                        .map(|s| (TypedId::<RELATED>::new(*id), s))
                })
            })
            .collect()
    }

    pub fn contained_structures<S>(&self, id: TypedId<S>) -> Vec<Id>
    where
        S: Structure,
    {
        self.ifc
            .data
            .find_all_of_type::<RelContainedInSpatialStructure>()
            .filter(|&(_, rel_structure)| (rel_structure.relating_structure == id.id()))
            .flat_map(|(_, rel_structure)| rel_structure.related_elements.0.clone())
            .collect()
    }

    pub fn related_type<S>(&self, id: TypedId<S>) -> &dyn IfcType
    where
        S: Structure,
    {
        self.ifc
            .data
            .find_all_of_type::<RelDefinesByType>()
            .find(|(_, rel_types)| {
                rel_types
                    .related_objects
                    .0
                    .iter()
                    .any(|rel_id| *rel_id == id.id())
            })
            .map(|(_, rel_types)| self.ifc.data.get_untyped(rel_types.relating_type))
            .unwrap()
    }

    pub fn related_voids<'a, S>(&'a self, id: TypedId<S>) -> Vec<&'a OpeningElement>
    where
        S: Structure,
    {
        self.ifc
            .data
            .find_all_of_type::<RelVoidsElement>()
            .filter_map(|(_id, rel_voids)| {
                (rel_voids.relating_building_element == id.id())
                    .then(|| self.ifc.data.get(rel_voids.related_opening_element))
            })
            .collect()
    }

    pub fn related_materials<S>(&self, id: TypedId<S>) -> Vec<&MaterialLayer>
    where
        S: Structure,
    {
        self.ifc
            .data
            .find_all_of_type::<RelAssociatesMaterial>()
            .filter(|(_, rel_material)| {
                rel_material
                    .related_objects
                    .0
                    .iter()
                    .any(|obj_id| *obj_id == id.id())
            })
            .filter_map(|(_, rel_material)| {
                self.ifc
                    .data
                    .get_untyped(rel_material.relating_material)
                    .downcast_ref::<MaterialLayerSetUsage>()
            })
            .flat_map(|set_usage| {
                self.ifc
                    .data
                    .get(set_usage.spatial_element_structure)
                    .material_layers
                    .0
                    .iter()
            })
            .map(|material_layer_id| self.ifc.data.get(*material_layer_id))
            .collect()
    }

    pub fn related_constiuents<S>(&self, id: TypedId<S>) -> Vec<&MaterialConstituent>
    where
        S: Structure,
    {
        self.ifc
            .data
            .find_all_of_type::<RelAssociatesMaterial>()
            .filter(|(_, rel_material)| {
                rel_material
                    .related_objects
                    .0
                    .iter()
                    .any(|obj_id| *obj_id == id.id())
            })
            .filter_map(|(_, rel_material)| {
                self.ifc
                    .data
                    .get_untyped(rel_material.relating_material)
                    .downcast_ref::<MaterialConstituentSet>()
            })
            .flat_map(|constituent_set| constituent_set.material_constituents.0.iter())
            .map(|constituent_id| self.ifc.data.get(*constituent_id))
            .collect()
    }
}

impl From<IFC> for IfcExtractor {
    fn from(ifc: IFC) -> Self {
        Self { ifc }
    }
}

impl Deref for IfcExtractor {
    type Target = IFC;

    fn deref(&self) -> &Self::Target {
        &self.ifc
    }
}
