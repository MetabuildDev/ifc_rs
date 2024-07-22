use std::collections::HashSet;

use glam::DVec3;

use crate::prelude::*;

pub struct VerticalShadingDeviceParameter {
    pub height: f64,
    pub length: f64,
    pub placement: DVec3,
}

impl<'a> IfcStoreyBuilder<'a> {
    pub fn shading_device_type(
        &mut self,
        material: TypedId<MaterialLayerSet>,
        name: &str,
        shading_device_type: ShadingDeviceTypeEnum,
    ) -> TypedId<ShadingDeviceType> {
        let shading_device_type = ShadingDeviceType::new(name, shading_device_type)
            .owner_history(self.owner_history, &mut self.project.ifc)
            .name(name);

        let shading_device_type_id = self.project.ifc.data.insert_new(shading_device_type);

        self.shading_device_type_to_shading_device
            .insert(shading_device_type_id, HashSet::new());
        self.project
            .material_to_shading_device_type
            .entry(material)
            .or_insert_with(|| {
                RelAssociatesMaterial::new(
                    format!("Material{material:?}ToShadingDeviceType"),
                    material,
                    &mut self.project.ifc,
                )
                .owner_history(self.owner_history, &mut self.project.ifc)
            })
            .relate_push(shading_device_type_id, &mut self.project.ifc);

        shading_device_type_id
    }

    pub fn vertical_shading_device(
        &mut self,
        material: TypedId<MaterialLayerSetUsage>,
        shading_device_type: TypedId<ShadingDeviceType>,
        name: &str,
        shading_device_information: VerticalShadingDeviceParameter,
    ) -> TypedId<ShadingDevice> {
        let wall_thickness = self.calculate_material_layer_set_thickness(material);

        let product_shape = ProductDefinitionShape::new_rectangular_shape(
            shading_device_information.length,
            wall_thickness,
            shading_device_information.height,
            Direction3D::from(DVec3::new(0.0, 0.0, 1.0)),
            self.sub_context,
            &mut self.project.ifc,
        );

        let position = Axis3D::new(
            Point3D::from(shading_device_information.placement),
            &mut self.project.ifc,
        );

        let local_placement =
            LocalPlacement::new_relative(position, self.storey, &mut self.project.ifc);

        let shading_device = ShadingDevice::new(name)
            .owner_history(self.owner_history, &mut self.project.ifc)
            .object_placement(local_placement, &mut self.project.ifc)
            .representation(product_shape, &mut self.project.ifc);

        self.shading_device(material, shading_device_type, shading_device)
    }

    fn shading_device(
        &mut self,
        material: TypedId<MaterialLayerSetUsage>,
        shading_device_type: TypedId<ShadingDeviceType>,
        shading_device: ShadingDevice,
    ) -> TypedId<ShadingDevice> {
        let shading_device_id = self.project.ifc.data.insert_new(shading_device);

        self.shading_devices.insert(shading_device_id);
        self.shading_device_type_to_shading_device
            .entry(shading_device_type)
            .or_default()
            .insert(shading_device_id);
        self.project
            .material_to_shading_device
            .entry(material)
            .or_insert_with(|| {
                RelAssociatesMaterial::new(
                    format!("Material{material:?}ToShadingDevices"),
                    material,
                    &mut self.project.ifc,
                )
                .owner_history(self.owner_history, &mut self.project.ifc)
            })
            .relate_push(shading_device_id, &mut self.project.ifc);

        shading_device_id
    }
}
