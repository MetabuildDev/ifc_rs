use std::collections::HashSet;

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

    pub fn new_storey<'b>(&'b mut self, name: &str) -> IfcStoreyBuilder<'b> {
        let owner_history = self.ifc.data.id_of::<OwnerHistory>().last().unwrap();
        let storey = Storey::new(name).owner_history(owner_history, &mut self.ifc);
        let storey_id = self.ifc.data.insert_new(storey);

        self.storeys.insert(storey_id);

        IfcStoreyBuilder::new(self.ifc, storey_id, owner_history)
    }
}

impl<'a> Drop for IfcBuildingBuilder<'a> {
    fn drop(&mut self) {
        let mut spatial_relation: RelContainedInSpatialStructure =
            RelContainedInSpatialStructure::new(
                "BuildingToStructureElements",
                self.building,
                self.ifc,
            )
            .owner_history(self.owner_history, self.ifc);

        for storey in self.storeys.iter() {
            spatial_relation = spatial_relation.relate_structure(*storey, self.ifc);
        }

        self.ifc.data.insert_new(spatial_relation);
    }
}
