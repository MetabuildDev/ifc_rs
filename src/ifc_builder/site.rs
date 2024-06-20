use std::collections::HashSet;

use glam::DVec3;

use crate::prelude::*;

pub struct IfcSiteBuilder<'a> {
    pub(crate) ifc: &'a mut IFC,

    pub(crate) owner_history: TypedId<OwnerHistory>,
    pub(crate) sub_context: TypedId<GeometricRepresentationSubContext>,

    pub(crate) site: TypedId<Site>,
    pub(crate) buildings: HashSet<TypedId<Building>>,
}

impl<'a> IfcSiteBuilder<'a> {
    pub(crate) fn new(
        ifc: &'a mut IFC,
        site: TypedId<Site>,
        owner_history: TypedId<OwnerHistory>,
    ) -> Self {
        let sub_context = ifc
            .data
            .id_of::<GeometricRepresentationSubContext>()
            .last()
            .unwrap();

        Self {
            ifc,
            site,
            owner_history,
            sub_context,
            buildings: HashSet::new(),
        }
    }

    pub fn new_building<'b>(&'b mut self, name: &str, position: DVec3) -> IfcBuildingBuilder<'b> {
        let position = Axis3D::new(Point3D::from(position), &mut self.ifc);
        let local_placement = LocalPlacement::new_relative(position, self.site, &mut self.ifc);
        let owner_history = self.ifc.data.id_of::<OwnerHistory>().last().unwrap();
        let building = Building::new(name)
            .owner_history(owner_history, &mut self.ifc)
            .object_placement(local_placement, &mut self.ifc);
        let building_id = self.ifc.data.insert_new(building);

        self.buildings.insert(building_id);

        IfcBuildingBuilder::new(self.ifc, building_id, owner_history)
    }
}

impl<'a> Drop for IfcSiteBuilder<'a> {
    fn drop(&mut self) {
        // rel aggregates
        let rel_agg = RelAggregates::new(
            "SiteBuildingsLink",
            self.site.id(),
            self.buildings.iter().map(|id| id.id()),
        );
        self.ifc.data.insert_new(rel_agg);
    }
}
