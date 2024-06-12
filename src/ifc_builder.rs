use glam::DVec3;
use std::{
    collections::{HashMap, HashSet},
    fs::write,
};

use crate::prelude::*;

pub struct ApplicationInfo<'a> {
    pub developer: Person,
    pub version: &'a str,
    pub name: &'a str,
    pub short_name: &'a str,
}

pub struct OwnerInfo<'a> {
    pub owner: Person,
    pub organization_name: &'a str,
}

pub struct IfcBuilder {
    ifc: IFC,

    buildings: HashSet<TypedId<Building>>,
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
            .owning_application(application_id.id(), &mut me.ifc)
            .last_modified_date(IfcTimestamp::now())
            .last_modifying_user(modifying_user, &mut me.ifc)
            .last_modifying_application(application_id.id(), &mut me.ifc);

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

        let project = Project::new(project_name)
            .owner_history(owner_history_id, &mut me.ifc)
            .unit_assignment(unit_assignment, &mut me.ifc)
            .add_context(context, &mut me.ifc);

        me.ifc.data.insert_new(project);

        me
    }

    pub fn new_building<'a>(&'a mut self, name: &str) -> IfcBuildingBuilder<'a> {
        let owner_history = self.ifc.data.id_of::<OwnerHistory>().last().unwrap();
        let building = Building::new(name).owner_history(owner_history.id(), &mut self.ifc);
        let building_id = self.ifc.data.insert_new(building);

        IfcBuildingBuilder::new(&mut self.ifc, building_id, owner_history)
    }

    pub fn build(mut self) -> String {
        // create relation between project and buildings
        let project_id = self.ifc.data.id_of::<Project>().last().unwrap();

        let project_building_relation = RelAggregates::new("ProjectBuildingsLink")
            .relate_project_with_buildings(
                project_id.id(),
                self.buildings
                    .into_iter()
                    .map(|building_id| building_id.into()),
                &mut self.ifc,
            );
        self.ifc.data.insert_new(project_building_relation);

        self.ifc.to_string()
    }
}

pub struct IfcBuildingBuilder<'a> {
    ifc: &'a mut IFC,

    owner_history: TypedId<OwnerHistory>,

    building: TypedId<Building>,

    walls: HashSet<TypedId<Wall>>,
    wall_types: HashSet<TypedId<WallType>>,

    wall_to_wall_type: Vec<(TypedId<Wall>, TypedId<WallType>)>,
    material_to_wall: HashMap<TypedId<MaterialLayerSetUsage>, HashSet<TypedId<Wall>>>,
    material_to_wall_type: HashMap<TypedId<MaterialLayerSet>, HashSet<TypedId<WallType>>>,
}

impl<'a> IfcBuildingBuilder<'a> {
    fn new(
        ifc: &'a mut IFC,
        building: TypedId<Building>,
        owner_history: TypedId<OwnerHistory>,
    ) -> Self {
        Self {
            ifc,
            building,
            owner_history,

            walls: HashSet::new(),
            wall_types: HashSet::new(),

            wall_to_wall_type: Vec::new(),
            material_to_wall: HashMap::new(),
            material_to_wall_type: HashMap::new(),
        }
    }

    pub fn add_wall(
        mut self,
        material: TypedId<MaterialLayerSetUsage>,
        wall_type: TypedId<WallType>,
        name: &str,
    ) {
        let wall = Wall::new(name).owner_history(self.owner_history.id(), self.ifc);

        let wall_id = self.ifc.data.insert_new(wall);

        self.walls.insert(wall_id);
        self.wall_to_wall_type.push((wall_id, wall_type));
        self.material_to_wall.get(&material).insert(wall_id);
    }

    pub fn add_wall_type(
        mut self,
        material: TypedId<MaterialLayerSet>,
        name: &str,
        wall_type: WallTypeEnum,
    ) -> TypedId<WallType> {
        let wall_type = WallType::new(name, wall_type)
            .owner_history(self.owner_history.id(), self.ifc)
            .name(name);

        let wall_type_id = self.ifc.data.insert_new(wall_type);

        self.wall_types.insert(wall_type_id);
        self.material_to_wall_type
            .get(&material)
            .insert(wall_type_id);

        wall_type_id
    }

    pub fn add_material(mut self) -> TypedId<Material> {}

    pub fn build(mut self) {
        // relate wall type to wall
        for (index, (wall, wall_type)) in self.wall_to_wall_type.into_iter().enumerate() {
            let wall_wall_type_relation =
                RelDefinesByType::new(format!("WallToWallType{index}"), wall_type, self.ifc)
                    .relate_obj(wall, self.ifc)
                    .owner_history(self.owner_history.id(), self.ifc);
            self.ifc.data.insert_new(wall_wall_type_relation);
        }

        // relate material to wall
        for (index, (material, walls)) in self.material_to_wall.into_iter().enumerate() {
            let mut material_wall_association =
                RelAssociatesMaterial::new(format!("MaterialToWall{index}"), material, self.ifc)
                    .owner_history(self.owner_history.id(), self.ifc);

            for wall in walls {
                material_wall_association = material_wall_association.relate_wall(wall, self.ifc);
            }

            self.ifc.data.insert_new(material_wall_association);
        }

        // relate material to wall type
        for (index, (material, wall_types)) in self.material_to_wall_type.into_iter().enumerate() {
            let mut wall_type_material_association = RelAssociatesMaterial::new(
                format!("MaterialToWallType{index}"),
                material,
                self.ifc,
            )
            .owner_history(self.owner_history.id(), self.ifc);

            for wall_type in wall_types {
                wall_type_material_association =
                    wall_type_material_association.relate_wall_type(wall_type, self.ifc);
            }

            self.ifc.data.insert_new(wall_type_material_association);
        }

        // relate building to walls
        let mut spatial_relation: RelContainedInSpatialStructure =
            RelContainedInSpatialStructure::new("BuildingToWall", self.building, self.ifc)
                .owner_history(self.owner_history.id(), self.ifc);

        for wall in self.walls {
            spatial_relation = spatial_relation.relate_structure(wall, self.ifc);
        }

        self.ifc.data.insert_new(spatial_relation);
    }
}
