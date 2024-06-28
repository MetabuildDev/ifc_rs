use std::collections::HashSet;

use glam::DVec3;

use crate::prelude::*;

pub struct IfcBuildingBuilder<'a> {
    pub(crate) project: &'a mut IfcProjectBuilder,

    pub(crate) owner_history: TypedId<OwnerHistory>,
    pub(crate) sub_context: TypedId<GeometricRepresentationSubContext>,

    pub(crate) building: TypedId<Building>,
    pub(crate) storeys: HashSet<TypedId<Storey>>,
}

impl<'a> IfcBuildingBuilder<'a> {
    pub(crate) fn new(
        project: &'a mut IfcProjectBuilder,
        building: TypedId<Building>,
        owner_history: TypedId<OwnerHistory>,
    ) -> Self {
        let sub_context = project
            .ifc
            .data
            .id_of::<GeometricRepresentationSubContext>()
            .last()
            .unwrap();

        Self {
            project,
            building,
            owner_history,
            sub_context,
            storeys: HashSet::new(),
        }
    }

    pub fn new_storey<'b>(&'b mut self, name: &str, elevation: f64) -> IfcStoreyBuilder<'b> {
        let position = Axis3D::new(Point3D::from(DVec3::Z * elevation), &mut self.project.ifc);
        let local_placement =
            LocalPlacement::new_relative(position, self.building, &mut self.project.ifc);
        let storey = Storey::new(name)
            .elevation(elevation)
            .owner_history(self.owner_history, &mut self.project.ifc)
            .object_placement(local_placement, &mut self.project.ifc);
        let storey_id = self.project.ifc.data.insert_new(storey);

        self.storeys.insert(storey_id);

        IfcStoreyBuilder::new(self.project, storey_id, self.owner_history)
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
        self.project.ifc.data.insert_new(rel_agg);
    }
}
