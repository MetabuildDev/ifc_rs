use std::collections::{HashMap, HashSet};

use bevy_math::DVec3;

use crate::prelude::*;

pub struct IfcProjectBuilder {
    pub(crate) ifc: IFC,

    pub(crate) owner_history: TypedId<OwnerHistory>,
    pub(crate) sub_context: TypedId<GeometricRepresentationContext>,

    pub(crate) project: TypedId<Project>,
    pub(crate) sites: HashSet<TypedId<Site>>,

    // Materials
    pub(crate) material_to_wall: HashMap<TypedId<MaterialLayerSetUsage>, RelAssociatesMaterial>,
    pub(crate) material_to_slab: HashMap<TypedId<MaterialLayerSetUsage>, RelAssociatesMaterial>,
    pub(crate) material_to_roof: HashMap<TypedId<MaterialLayerSetUsage>, RelAssociatesMaterial>,
    pub(crate) material_to_window: HashMap<TypedId<MaterialConstituentSet>, RelAssociatesMaterial>,
    pub(crate) material_to_shading_device:
        HashMap<TypedId<MaterialLayerSetUsage>, RelAssociatesMaterial>,
    // TODO: Required??
    pub(crate) material_to_wall_type: HashMap<TypedId<MaterialLayerSet>, RelAssociatesMaterial>,
    pub(crate) material_to_slab_type: HashMap<TypedId<MaterialLayerSet>, RelAssociatesMaterial>,
    pub(crate) material_to_roof_type: HashMap<TypedId<MaterialLayerSet>, RelAssociatesMaterial>,
    pub(crate) material_to_shading_device_type:
        HashMap<TypedId<MaterialLayerSet>, RelAssociatesMaterial>,
}

impl IfcProjectBuilder {
    pub fn new(
        application_info: ApplicationInfo<'_>,
        owner_info: OwnerInfo<'_>,
        modifying_user: Person,
        project_name: &str,
    ) -> Self {
        let mut ifc = IFC::default();

        let application = Application::new(
            application_info.developer,
            application_info.version,
            application_info.name,
            application_info.short_name,
            &mut ifc,
        );
        let application_id = ifc.data.insert_new(application);

        let owner = PersonAndOrganization::new(
            owner_info.owner,
            Organization::new(None, owner_info.organization_name, None),
            &mut ifc,
        );

        let owner_history = OwnerHistory::new(ChangeAction::Added, IfcTimestamp::now())
            .owning_user(owner, &mut ifc)
            .owning_application(application_id, &mut ifc)
            .last_modified_date(IfcTimestamp::now())
            .last_modifying_user(modifying_user, &mut ifc)
            .last_modifying_application(application_id, &mut ifc);

        let owner_history_id = ifc.data.insert_new(owner_history);

        // assume a few defaults here

        // create measurement units used for this project
        let length = SiUnit::new(IfcUnitEnum::LengthUnit, None, IfcUnitName::Metre);
        let angle = SiUnit::new(IfcUnitEnum::PlaneAngleUnit, None, IfcUnitName::Radian);
        let time = SiUnit::new(IfcUnitEnum::TimeUnit, None, IfcUnitName::Second);

        // collect measurement units
        let unit_assignment =
            UnitAssigment::new([length.into(), angle.into(), time.into()], &mut ifc);

        // create world root coordinate
        let world_root = Axis3D::new(Point3D::from(DVec3::new(0.0, 0.0, 0.0)), &mut ifc);

        // create geometry context
        let context =
            GeometricRepresentationContext::new(DimensionCount::Three, world_root, &mut ifc);
        let context_id = ifc.data.insert_new(context);

        let sub_context = GeometricRepresentationSubContext::derive(
            context_id,
            GeometricProjection::ModelView,
            &mut ifc,
        );

        let project = Project::new(project_name)
            .name(project_name)
            .owner_history(owner_history_id, &mut ifc)
            .unit_assignment(unit_assignment, &mut ifc)
            .add_context(context_id, &mut ifc);

        let project_id = ifc.data.insert_new(project);
        ifc.data.insert_new(sub_context);

        Self {
            ifc,
            owner_history: owner_history_id,
            sub_context: context_id,
            project: project_id,
            sites: HashSet::new(),

            material_to_wall_type: HashMap::new(),
            material_to_wall: HashMap::new(),
            material_to_slab: HashMap::new(),
            material_to_slab_type: HashMap::new(),
            material_to_roof: HashMap::new(),
            material_to_roof_type: HashMap::new(),
            material_to_shading_device: HashMap::new(),
            material_to_shading_device_type: HashMap::new(),
            material_to_window: HashMap::new(),
        }
    }

    pub fn new_site<'a>(&'a mut self, name: &str, position: DVec3) -> IfcSiteBuilder<'a> {
        let position = Axis3D::new(Point3D::from(position), &mut self.ifc);
        let local_placement = LocalPlacement::new(position, &mut self.ifc);
        let site = Site::new(name)
            .owner_history(self.owner_history, &mut self.ifc)
            .object_placement(local_placement, &mut self.ifc);
        let site_id = self.ifc.data.insert_new(site);

        self.sites.insert(site_id);

        IfcSiteBuilder::new(self, site_id, self.owner_history)
    }

    pub fn build(mut self) -> String {
        self.material_to_wall
            .into_values()
            .chain(self.material_to_slab.into_values())
            .chain(self.material_to_roof.into_values())
            .chain(self.material_to_window.into_values())
            .chain(self.material_to_wall_type.into_values())
            .chain(self.material_to_slab_type.into_values())
            .chain(self.material_to_roof_type.into_values())
            .chain(self.material_to_shading_device.into_values())
            .chain(self.material_to_shading_device_type.into_values())
            .for_each(|associate_relations| {
                self.ifc.data.insert_new(associate_relations);
            });

        // rel aggregates
        let rel_agg = RelAggregates::new(
            "ProjectSitesLink",
            self.project.id(),
            self.sites.iter().map(|id| id.id()),
        );
        self.ifc.data.insert_new(rel_agg);

        self.ifc.to_string()
    }
}
