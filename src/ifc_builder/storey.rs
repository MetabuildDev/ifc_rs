use std::collections::{HashMap, HashSet};

use crate::prelude::*;

pub struct IfcStoreyBuilder<'a> {
    pub(crate) ifc: &'a mut IFC,

    pub(crate) owner_history: TypedId<OwnerHistory>,
    pub(crate) sub_context: TypedId<GeometricRepresentationSubContext>,

    pub(crate) storey: TypedId<Storey>,

    pub(crate) walls: HashSet<TypedId<Wall>>,
    pub(crate) slabs: HashSet<TypedId<Slab>>,
    pub(crate) opening_elements: HashSet<TypedId<OpeningElement>>,
    pub(crate) windows: HashSet<TypedId<Window>>,

    // Wall relations
    pub(crate) wall_type_to_wall: HashMap<TypedId<WallType>, HashSet<TypedId<Wall>>>,
    pub(crate) material_to_wall: HashMap<TypedId<MaterialLayerSetUsage>, HashSet<TypedId<Wall>>>,
    pub(crate) material_to_wall_type:
        HashMap<TypedId<MaterialLayerSet>, HashSet<TypedId<WallType>>>,

    // Slab relations
    pub(crate) slab_type_to_slab: HashMap<TypedId<SlabType>, HashSet<TypedId<Slab>>>,
    pub(crate) material_to_slab: HashMap<TypedId<MaterialLayerSetUsage>, HashSet<TypedId<Slab>>>,
    pub(crate) material_to_slab_type:
        HashMap<TypedId<MaterialLayerSet>, HashSet<TypedId<SlabType>>>,

    // Opening element relations
    pub(crate) opening_elements_to_wall: HashMap<TypedId<OpeningElement>, TypedId<Wall>>,
    pub(crate) opening_elements_to_window: HashMap<TypedId<OpeningElement>, TypedId<Window>>,

    // Window relations
    pub(crate) window_type_to_window: HashMap<TypedId<WindowType>, HashSet<TypedId<Window>>>,
    pub(crate) material_to_window:
        HashMap<TypedId<MaterialConstituentSet>, HashSet<TypedId<Window>>>,
}

impl<'a> IfcStoreyBuilder<'a> {
    pub(crate) fn new(
        ifc: &'a mut IFC,
        storey: TypedId<Storey>,
        owner_history: TypedId<OwnerHistory>,
    ) -> Self {
        let sub_context = ifc
            .data
            .id_of::<GeometricRepresentationSubContext>()
            .last()
            .unwrap();

        Self {
            ifc,
            storey,
            owner_history,
            sub_context,

            walls: HashSet::new(),
            slabs: HashSet::new(),
            opening_elements: HashSet::new(),
            windows: HashSet::new(),

            wall_type_to_wall: HashMap::new(),
            material_to_wall: HashMap::new(),
            material_to_wall_type: HashMap::new(),

            slab_type_to_slab: HashMap::new(),
            material_to_slab: HashMap::new(),
            material_to_slab_type: HashMap::new(),

            opening_elements_to_wall: HashMap::new(),
            opening_elements_to_window: HashMap::new(),

            window_type_to_window: HashMap::new(),
            material_to_window: HashMap::new(),
        }
    }
}

impl<'a> Drop for IfcStoreyBuilder<'a> {
    fn drop(&mut self) {
        let mut spatial_relation: RelContainedInSpatialStructure =
            RelContainedInSpatialStructure::new("StoreyToStructureElements", self.storey, self.ifc)
                .owner_history(self.owner_history, self.ifc);

        // TODO: Organise this better

        // walls ----------------------

        // relate wall type to wall
        for (index, (wall_type, walls)) in self.wall_type_to_wall.iter().enumerate() {
            let mut wall_wall_type_relation =
                RelDefinesByType::new(format!("WallTypeToWall{index}"), *wall_type, self.ifc)
                    .owner_history(self.owner_history, self.ifc);

            for wall in walls {
                wall_wall_type_relation = wall_wall_type_relation.relate_obj(*wall, self.ifc)
            }

            self.ifc.data.insert_new(wall_wall_type_relation);
        }

        // relate material set usage to wall
        for (index, (material, walls)) in self.material_to_wall.iter().enumerate() {
            let mut material_wall_association =
                RelAssociatesMaterial::new(format!("MaterialToWall{index}"), *material, self.ifc)
                    .owner_history(self.owner_history, self.ifc);

            for wall in walls {
                material_wall_association = material_wall_association.relate_obj(*wall, self.ifc);
            }

            self.ifc.data.insert_new(material_wall_association);
        }

        // relate material set to wall type
        for (index, (material, wall_types)) in self.material_to_wall_type.iter().enumerate() {
            let mut wall_type_material_association = RelAssociatesMaterial::new(
                format!("MaterialToWallType{index}"),
                *material,
                self.ifc,
            )
            .owner_history(self.owner_history, self.ifc);

            for wall_type in wall_types {
                wall_type_material_association =
                    wall_type_material_association.relate_obj(*wall_type, self.ifc);
            }

            self.ifc.data.insert_new(wall_type_material_association);
        }

        // relate storey to walls
        for wall in self.walls.iter() {
            spatial_relation = spatial_relation.relate_structure(*wall, self.ifc);
        }

        // slabs ----------------------

        // relate slab type to slab
        for (index, (slab_type, slabs)) in self.slab_type_to_slab.iter().enumerate() {
            let mut slab_slab_type_relation =
                RelDefinesByType::new(format!("SlabTypeToSlab{index}"), *slab_type, self.ifc)
                    .owner_history(self.owner_history, self.ifc);

            for slab in slabs {
                slab_slab_type_relation = slab_slab_type_relation.relate_obj(*slab, self.ifc)
            }

            self.ifc.data.insert_new(slab_slab_type_relation);
        }

        // relate material set usage to slab
        for (index, (material, slabs)) in self.material_to_slab.iter().enumerate() {
            let mut material_slab_association =
                RelAssociatesMaterial::new(format!("MaterialToSlab{index}"), *material, self.ifc)
                    .owner_history(self.owner_history, self.ifc);

            for slab in slabs {
                material_slab_association = material_slab_association.relate_obj(*slab, self.ifc);
            }

            self.ifc.data.insert_new(material_slab_association);
        }

        // relate material set to slab type
        for (index, (material, slab_types)) in self.material_to_slab_type.iter().enumerate() {
            let mut slab_type_material_association = RelAssociatesMaterial::new(
                format!("MaterialToSlabType{index}"),
                *material,
                self.ifc,
            )
            .owner_history(self.owner_history, self.ifc);

            for slab_type in slab_types {
                slab_type_material_association =
                    slab_type_material_association.relate_obj(*slab_type, self.ifc);
            }

            self.ifc.data.insert_new(slab_type_material_association);
        }

        // relate storey to slabs
        for slab in self.slabs.iter() {
            spatial_relation = spatial_relation.relate_structure(*slab, self.ifc);
        }

        // opening elements ----------------------

        // relate opening elements to walls
        for (index, (opening_element, wall)) in self.opening_elements_to_wall.iter().enumerate() {
            let opening_element_wall_relation = RelVoidsElement::new(
                format!("OpeningElementToWall{index}"),
                *wall,
                *opening_element,
                self.ifc,
            )
            .owner_history(self.owner_history, self.ifc);

            self.ifc.data.insert_new(opening_element_wall_relation);
        }

        // windows ----------------------

        // relate window type to window
        for (index, (window_type, windows)) in self.window_type_to_window.iter().enumerate() {
            let mut window_window_type_relation =
                RelDefinesByType::new(format!("WindowTypeToWindow{index}"), *window_type, self.ifc)
                    .owner_history(self.owner_history, self.ifc);

            for window in windows {
                window_window_type_relation =
                    window_window_type_relation.relate_obj(*window, self.ifc)
            }

            self.ifc.data.insert_new(window_window_type_relation);
        }

        // relate material set to window
        for (index, (material, windows)) in self.material_to_window.iter().enumerate() {
            let mut material_window_association =
                RelAssociatesMaterial::new(format!("MaterialToWindow{index}"), *material, self.ifc)
                    .owner_history(self.owner_history, self.ifc);

            for window in windows {
                material_window_association =
                    material_window_association.relate_obj(*window, self.ifc);
            }

            self.ifc.data.insert_new(material_window_association);
        }

        // relate opening elements to windows
        for (index, (opening_element, window)) in self.opening_elements_to_window.iter().enumerate()
        {
            let opening_element_window_relation = RelFillsElement::new(
                format!("OpeningElementToWindow{index}"),
                *opening_element,
                *window,
                self.ifc,
            )
            .owner_history(self.owner_history, self.ifc);

            self.ifc.data.insert_new(opening_element_window_relation);
        }

        // relate storey to windows
        for window in self.windows.iter() {
            spatial_relation = spatial_relation.relate_structure(*window, self.ifc);
        }

        self.ifc.data.insert_new(spatial_relation);
    }
}
