use std::collections::HashSet;

use glam::DVec3;

use crate::prelude::*;

pub struct IfcBuildingBuilder<'a> {
    pub(crate) ifc: &'a mut IFC,

    pub(crate) owner_history: TypedId<OwnerHistory>,
    pub(crate) sub_context: TypedId<GeometricRepresentationSubContext>,

    pub(crate) building: TypedId<Building>,
    pub(crate) storeys: HashSet<TypedId<Storey>>,
}

impl<'a> IfcBuildingBuilder<'a> {
    pub(crate) fn new(
        ifc: &'a mut IFC,
        building: TypedId<Building>,
        owner_history: TypedId<OwnerHistory>,
    ) -> Self {
        let sub_context = ifc
            .data
            .id_of::<GeometricRepresentationSubContext>()
            .last()
            .unwrap();

        Self {
            ifc,
            building,
            owner_history,
            sub_context,
            storeys: HashSet::new(),
        }
    }

    pub fn new_storey<'b>(&'b mut self, name: &str, elevation: f64) -> IfcStoreyBuilder<'b> {
        let relative_placement_id = self
            .ifc
            .data
            .get(self.building)
            .object_placement
            .custom()
            .expect("Building Placement Exists")
            .clone();
        let position = Axis3D::new(Point3D::from(DVec3::Z * elevation), &mut self.ifc);
        let local_placement = LocalPlacement::new(position, &mut self.ifc)
            .relative_to(relative_placement_id, &mut self.ifc);
        let owner_history = self.ifc.data.id_of::<OwnerHistory>().last().unwrap();
        let storey = Storey::new(name)
            .owner_history(owner_history, &mut self.ifc)
            .object_placement(local_placement, &mut self.ifc);
        let storey_id = self.ifc.data.insert_new(storey);

        self.storeys.insert(storey_id);

        IfcStoreyBuilder::new(self.ifc, storey_id, owner_history)
    }
}

impl<'a> Drop for IfcBuildingBuilder<'a> {
    fn drop(&mut self) {
        // rel aggregates
        let rel_agg = RelAggregates::new(
            "BuildingStoreysLink",
            self.building.id(),
            self.storeys.iter().map(|id| id.id()),
        );
        self.ifc.data.insert_new(rel_agg);
    }
}
