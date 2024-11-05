pub(crate) mod prelude;

use std::ops::Deref;

use crate::prelude::*;

pub trait TypeRelation<T: Structure, R: IfcType> {
    fn related_type(&self, typed_id: TypedId<T>) -> &R;
}

pub struct IfcExtractor {
    ifc: IFC,
}

impl IfcExtractor {
    pub fn projects(&self) -> impl Iterator<Item = (TypedId<Project>, &Project)> {
        self.ifc.data.find_all_of_type::<Project>()
    }

    pub fn relations_of<RELATING, RELATED>(
        &self,
        id: TypedId<RELATING>,
    ) -> impl Iterator<Item = (TypedId<RELATED>, &RELATED)>
    where
        RELATING: IfcType,
        RELATED: IfcType,
    {
        self.ifc
            .data
            .find_all_of_type::<RelAggregates>()
            .filter(move |&(_, rel_aggregate)| rel_aggregate.relating_object == id.id())
            .flat_map(|(_, rel_aggregate)| {
                rel_aggregate.related_objects.0.iter().filter_map(|id| {
                    self.ifc
                        .data
                        .get_untyped(*id)
                        .downcast_ref::<RELATED>()
                        .map(|s| (TypedId::<RELATED>::new(*id), s))
                })
            })
    }

    pub fn contained_structures<S>(&self, id: TypedId<S>) -> impl Iterator<Item = Id> + '_
    where
        S: Structure,
    {
        self.ifc
            .data
            .find_all_of_type::<RelContainedInSpatialStructure>()
            .filter(move |&(_, rel_structure)| rel_structure.relating_structure == id.id())
            .flat_map(|(_, rel_structure)| rel_structure.related_elements.0.clone())
    }

    fn related_type<S>(&self, id: TypedId<S>) -> &dyn IfcType
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

    pub fn related_voids<S>(&self, id: TypedId<S>) -> impl Iterator<Item = &'_ OpeningElement>
    where
        S: Structure,
    {
        self.ifc
            .data
            .find_all_of_type::<RelVoidsElement>()
            .filter(move |&(_, rel_voids)| (rel_voids.relating_building_element == id.id()))
            .map(|(_, rel_voids)| self.ifc.data.get(rel_voids.related_opening_element))
    }

    pub fn related_materials<S>(&self, id: TypedId<S>) -> impl Iterator<Item = &MaterialLayer>
    where
        S: Structure,
    {
        self.ifc
            .data
            .find_all_of_type::<RelAssociatesMaterial>()
            .filter(move |(_, rel_material)| {
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
    }

    pub fn related_constiuents<S>(
        &self,
        id: TypedId<S>,
    ) -> impl Iterator<Item = &MaterialConstituent>
    where
        S: Structure,
    {
        self.ifc
            .data
            .find_all_of_type::<RelAssociatesMaterial>()
            .filter(move |(_, rel_material)| {
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
    }
}

impl TypeRelation<Wall, WallType> for IfcExtractor {
    fn related_type(&self, typed_id: TypedId<Wall>) -> &WallType {
        self.related_type(typed_id)
            .downcast_ref::<WallType>()
            .unwrap()
    }
}

impl TypeRelation<Slab, SlabType> for IfcExtractor {
    fn related_type(&self, typed_id: TypedId<Slab>) -> &SlabType {
        self.related_type(typed_id)
            .downcast_ref::<SlabType>()
            .unwrap()
    }
}

impl TypeRelation<Roof, RoofType> for IfcExtractor {
    fn related_type(&self, typed_id: TypedId<Roof>) -> &RoofType {
        self.related_type(typed_id)
            .downcast_ref::<RoofType>()
            .unwrap()
    }
}

impl TypeRelation<Window, WindowType> for IfcExtractor {
    fn related_type(&self, typed_id: TypedId<Window>) -> &WindowType {
        self.related_type(typed_id)
            .downcast_ref::<WindowType>()
            .unwrap()
    }
}

impl TypeRelation<Door, DoorType> for IfcExtractor {
    fn related_type(&self, typed_id: TypedId<Door>) -> &DoorType {
        self.related_type(typed_id)
            .downcast_ref::<DoorType>()
            .unwrap()
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
