use std::collections::{HashMap, HashSet};

use crate::prelude::*;

pub struct IfcStoreyBuilder<'a> {
    pub(crate) project: &'a mut IfcProjectBuilder,

    pub(crate) owner_history: TypedId<OwnerHistory>,
    pub(crate) sub_context: TypedId<GeometricRepresentationSubContext>,

    pub(crate) storey: TypedId<Storey>,

    pub(crate) spaces: HashSet<TypedId<Space>>,
    pub(crate) walls: HashSet<TypedId<Wall>>,
    pub(crate) slabs: HashSet<TypedId<Slab>>,
    pub(crate) roofs: HashSet<TypedId<Roof>>,
    pub(crate) opening_elements: HashSet<TypedId<OpeningElement>>,
    pub(crate) windows: HashSet<TypedId<Window>>,
    pub(crate) shading_devices: HashSet<TypedId<ShadingDevice>>,

    // Wall relations
    pub(crate) wall_type_to_wall: HashMap<TypedId<WallType>, HashSet<TypedId<Wall>>>,

    // Slab relations
    pub(crate) slab_type_to_slab: HashMap<TypedId<SlabType>, HashSet<TypedId<Slab>>>,

    // Roof relations
    pub(crate) roof_type_to_roof: HashMap<TypedId<RoofType>, HashSet<TypedId<Roof>>>,

    // Opening element relations
    pub(crate) opening_elements_to_wall: HashMap<TypedId<OpeningElement>, TypedId<Wall>>,
    pub(crate) opening_elements_to_window: HashMap<TypedId<OpeningElement>, TypedId<Window>>,

    // Window relations
    pub(crate) window_type_to_window: HashMap<TypedId<WindowType>, HashSet<TypedId<Window>>>,

    // Space relations
    pub(crate) space_type_to_space: HashMap<TypedId<SpaceType>, HashSet<TypedId<Space>>>,

    // Shading device relations
    pub(crate) shading_device_type_to_shading_device:
        HashMap<TypedId<ShadingDeviceType>, HashSet<TypedId<ShadingDevice>>>,
}

impl<'a> IfcStoreyBuilder<'a> {
    pub(crate) fn new(
        project: &'a mut IfcProjectBuilder,
        storey: TypedId<Storey>,
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
            storey,
            owner_history,
            sub_context,

            spaces: HashSet::new(),
            walls: HashSet::new(),
            slabs: HashSet::new(),
            roofs: HashSet::new(),
            opening_elements: HashSet::new(),
            windows: HashSet::new(),
            shading_devices: HashSet::new(),

            wall_type_to_wall: HashMap::new(),

            slab_type_to_slab: HashMap::new(),

            roof_type_to_roof: HashMap::new(),

            opening_elements_to_wall: HashMap::new(),
            opening_elements_to_window: HashMap::new(),

            window_type_to_window: HashMap::new(),

            space_type_to_space: HashMap::new(),

            shading_device_type_to_shading_device: HashMap::new(),
        }
    }
}

impl<'a> Drop for IfcStoreyBuilder<'a> {
    fn drop(&mut self) {
        let mut spatial_relation: RelContainedInSpatialStructure =
            RelContainedInSpatialStructure::new(
                "StoreyToStructureElements",
                self.storey,
                &mut self.project.ifc,
            )
            .owner_history(self.owner_history, &mut self.project.ifc);

        // TODO: Organise this better

        // walls ----------------------

        // relate wall type to wall
        for (index, (wall_type, walls)) in self.wall_type_to_wall.iter().enumerate() {
            let mut wall_wall_type_relation = RelDefinesByType::new(
                format!("WallTypeToWall{index}"),
                *wall_type,
                &mut self.project.ifc,
            )
            .owner_history(self.owner_history, &mut self.project.ifc);

            for wall in walls {
                wall_wall_type_relation =
                    wall_wall_type_relation.relate_obj(*wall, &mut self.project.ifc)
            }

            self.project.ifc.data.insert_new(wall_wall_type_relation);
        }

        // relate storey to walls
        for wall in self.walls.iter() {
            spatial_relation = spatial_relation.relate_structure(*wall, &mut self.project.ifc);
        }

        // slabs ----------------------

        // relate slab type to slab
        for (index, (slab_type, slabs)) in self.slab_type_to_slab.iter().enumerate() {
            let mut slab_slab_type_relation = RelDefinesByType::new(
                format!("SlabTypeToSlab{index}"),
                *slab_type,
                &mut self.project.ifc,
            )
            .owner_history(self.owner_history, &mut self.project.ifc);

            for slab in slabs {
                slab_slab_type_relation =
                    slab_slab_type_relation.relate_obj(*slab, &mut self.project.ifc)
            }

            self.project.ifc.data.insert_new(slab_slab_type_relation);
        }

        // relate storey to slabs
        for slab in self.slabs.iter() {
            spatial_relation = spatial_relation.relate_structure(*slab, &mut self.project.ifc);
        }

        // roofs ----------------------

        // relate roof type to roof
        for (index, (roof_type, roofs)) in self.roof_type_to_roof.iter().enumerate() {
            let mut roof_roof_type_relation = RelDefinesByType::new(
                format!("RoofTypeToRoof{index}"),
                *roof_type,
                &mut self.project.ifc,
            )
            .owner_history(self.owner_history, &mut self.project.ifc);

            for roof in roofs {
                roof_roof_type_relation =
                    roof_roof_type_relation.relate_obj(*roof, &mut self.project.ifc)
            }

            self.project.ifc.data.insert_new(roof_roof_type_relation);
        }

        // relate storey to roofs
        for roof in self.roofs.iter() {
            spatial_relation = spatial_relation.relate_structure(*roof, &mut self.project.ifc);
        }

        // opening elements ----------------------

        // relate opening elements to walls
        for (index, (opening_element, wall)) in self.opening_elements_to_wall.iter().enumerate() {
            let opening_element_wall_relation = RelVoidsElement::new(
                format!("OpeningElementToWall{index}"),
                *wall,
                *opening_element,
                &mut self.project.ifc,
            )
            .owner_history(self.owner_history, &mut self.project.ifc);

            self.project
                .ifc
                .data
                .insert_new(opening_element_wall_relation);
        }

        // windows ----------------------

        // relate window type to window
        for (index, (window_type, windows)) in self.window_type_to_window.iter().enumerate() {
            let mut window_window_type_relation = RelDefinesByType::new(
                format!("WindowTypeToWindow{index}"),
                *window_type,
                &mut self.project.ifc,
            )
            .owner_history(self.owner_history, &mut self.project.ifc);

            for window in windows {
                window_window_type_relation =
                    window_window_type_relation.relate_obj(*window, &mut self.project.ifc)
            }

            self.project
                .ifc
                .data
                .insert_new(window_window_type_relation);
        }

        // relate opening elements to windows
        for (index, (opening_element, window)) in self.opening_elements_to_window.iter().enumerate()
        {
            let opening_element_window_relation = RelFillsElement::new(
                format!("OpeningElementToWindow{index}"),
                *opening_element,
                *window,
                &mut self.project.ifc,
            )
            .owner_history(self.owner_history, &mut self.project.ifc);

            self.project
                .ifc
                .data
                .insert_new(opening_element_window_relation);
        }

        // relate storey to windows
        for window in self.windows.iter() {
            spatial_relation = spatial_relation.relate_structure(*window, &mut self.project.ifc);
        }

        // shading devices ----------------------
        for (index, (shading_device_type, shading_devices)) in self
            .shading_device_type_to_shading_device
            .iter()
            .enumerate()
        {
            let mut shading_device_shading_device_type_relation = RelDefinesByType::new(
                format!("ShadingDeviceTypeToShadingDevice{index}"),
                *shading_device_type,
                &mut self.project.ifc,
            )
            .owner_history(self.owner_history, &mut self.project.ifc);

            for shading_device in shading_devices {
                shading_device_shading_device_type_relation =
                    shading_device_shading_device_type_relation
                        .relate_obj(*shading_device, &mut self.project.ifc)
            }

            self.project
                .ifc
                .data
                .insert_new(shading_device_shading_device_type_relation);
        }

        // relate storey to shading devices
        for shading_device in self.shading_devices.iter() {
            spatial_relation =
                spatial_relation.relate_structure(*shading_device, &mut self.project.ifc);
        }

        // insert all spatial relations of this story
        self.project.ifc.data.insert_new(spatial_relation);

        // rel aggregates ----------------------
        let rel_agg = RelAggregates::new(
            "StoreySpacesLink",
            self.storey.id(),
            self.spaces.iter().map(|id| id.id()),
        );
        self.project.ifc.data.insert_new(rel_agg);
    }
}
