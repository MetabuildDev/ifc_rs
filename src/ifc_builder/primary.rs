use std::collections::HashSet;

use glam::DVec3;

use crate::prelude::*;

pub struct IfcBuilder {
    pub(crate) ifc: IFC,

    pub(crate) buildings: HashSet<TypedId<Building>>,
}

impl IfcBuilder {
    pub fn new(
        application_info: ApplicationInfo<'_>,
        owner_info: OwnerInfo<'_>,
        modifying_user: Person,
        project_name: &str,
    ) -> Self {
        let mut me = Self {
            ifc: IFC::default(),
            buildings: HashSet::new(),
        };

        let application = Application::new(
            application_info.developer,
            application_info.version,
            application_info.name,
            application_info.short_name,
            &mut me.ifc,
        );
        let application_id = me.ifc.data.insert_new(application);

        let owner = PersonAndOrganization::new(
            owner_info.owner,
            Organization::new(None, owner_info.organization_name, None),
            &mut me.ifc,
        );

        let owner_history = OwnerHistory::new(ChangeAction::Added, IfcTimestamp::now())
            .owning_user(owner, &mut me.ifc)
            .owning_application(application_id, &mut me.ifc)
            .last_modified_date(IfcTimestamp::now())
            .last_modifying_user(modifying_user, &mut me.ifc)
            .last_modifying_application(application_id, &mut me.ifc);

        let owner_history_id = me.ifc.data.insert_new(owner_history);

        // assume a few defaults here

        // create measurement units used for this project
        let length = SiUnit::new(IfcUnitEnum::LengthUnit, None, IfcUnitName::Metre);
        let angle = SiUnit::new(IfcUnitEnum::PlaneAngleUnit, None, IfcUnitName::Radian);
        let time = SiUnit::new(IfcUnitEnum::TimeUnit, None, IfcUnitName::Second);

        // collect measurement units
        let unit_assignment =
            UnitAssigment::new([length.into(), angle.into(), time.into()], &mut me.ifc);

        // create world root coordinate
        let world_root = Axis3D::new(Point3D::from(DVec3::new(0.0, 0.0, 0.0)), &mut me.ifc);

        // create geometry context
        let context =
            GeometricRepresentationContext::new(DimensionCount::Three, world_root, &mut me.ifc);
        let context_id = me.ifc.data.insert_new(context);

        let sub_context = GeometricRepresentationSubContext::derive(
            context_id,
            GeometricProjection::ModelView,
            &mut me.ifc,
        );

        let project = Project::new(project_name)
            .owner_history(owner_history_id, &mut me.ifc)
            .unit_assignment(unit_assignment, &mut me.ifc)
            .add_context(context_id, &mut me.ifc);

        me.ifc.data.insert_new(project);
        me.ifc.data.insert_new(sub_context);

        me
    }

    pub fn new_building<'a>(&'a mut self, name: &str) -> IfcBuildingBuilder<'a> {
        let owner_history = self.ifc.data.id_of::<OwnerHistory>().last().unwrap();
        let building = Building::new(name).owner_history(owner_history, &mut self.ifc);
        let building_id = self.ifc.data.insert_new(building);

        self.buildings.insert(building_id);

        IfcBuildingBuilder::new(&mut self.ifc, building_id, owner_history)
    }

    pub fn build(mut self) -> String {
        // create relation between project and buildings
        let project_id = self.ifc.data.id_of::<Project>().last().unwrap();

        let project_building_relation = RelAggregates::new("ProjectBuildingsLink")
            .relate_project_with_buildings(
                project_id,
                self.buildings
                    .into_iter()
                    .map(|building_id| building_id.into()),
                &mut self.ifc,
            );
        self.ifc.data.insert_new(project_building_relation);

        self.ifc.to_string()
    }
}
