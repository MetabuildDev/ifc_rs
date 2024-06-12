use glam::{DVec2, DVec3};
use std::collections::{HashMap, HashSet};

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

pub struct VerticalWallParameter {
    pub height: f64,
    pub length: f64,
    pub placement: DVec3,
}

pub struct IfcBuildingBuilder<'a> {
    ifc: &'a mut IFC,

    owner_history: TypedId<OwnerHistory>,
    sub_context: TypedId<GeometricRepresentationSubContext>,

    building: TypedId<Building>,

    walls: HashSet<TypedId<Wall>>,

    wall_type_to_wall: HashMap<TypedId<WallType>, HashSet<TypedId<Wall>>>,
    material_to_wall: HashMap<TypedId<MaterialLayerSetUsage>, HashSet<TypedId<Wall>>>,
    material_to_wall_type: HashMap<TypedId<MaterialLayerSet>, HashSet<TypedId<WallType>>>,
}

impl<'a> IfcBuildingBuilder<'a> {
    fn new(
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

            walls: HashSet::new(),

            wall_type_to_wall: HashMap::new(),
            material_to_wall: HashMap::new(),
            material_to_wall_type: HashMap::new(),
        }
    }

    pub fn vertical_wall(
        &mut self,
        material: TypedId<MaterialLayerSetUsage>,
        wall_type: TypedId<WallType>,
        name: &str,
        wall_information: VerticalWallParameter,
    ) {
        let position = Axis3D::new(Point3D::from(wall_information.placement), self.ifc);
        let wall_thickness = self.calculate_material_layer_set_thickness(material);

        let shape_repr = ShapeRepresentation::new(self.sub_context, self.ifc)
            .add_item(
                PolyLine::from_3d(
                    [
                        DVec3::new(0.0, 0.0, 0.0).into(),
                        DVec3::new(wall_information.length, 0.0, 0.0).into(),
                    ]
                    .into_iter(),
                    self.ifc,
                ),
                self.ifc,
            )
            .add_item(
                ExtrudedAreaSolid::new(
                    RectangleProfileDef::new(
                        ProfileType::Area,
                        wall_information.length,
                        wall_thickness,
                    )
                    // center of the rectangle
                    .position(
                        Axis2D::new(
                            Point2D::from(DVec2::new(
                                wall_information.length * 0.5,
                                wall_thickness * 0.5,
                            )),
                            self.ifc,
                        ),
                        self.ifc,
                    ),
                    // vertical wall (z-up)
                    Direction3D::from(DVec3::new(0.0, 0.0, 1.0)),
                    wall_information.height,
                    self.ifc,
                ),
                self.ifc,
            );

        let product_shape = ProductDefinitionShape::new().add_representation(shape_repr, self.ifc);
        let local_placement = LocalPlacement::new(position, self.ifc);

        let wall = Wall::new(name)
            .owner_history(self.owner_history, self.ifc)
            .object_placement(local_placement, self.ifc)
            .representation(product_shape, self.ifc);

        self.wall(material, wall_type, wall);
    }

    fn calculate_material_layer_set_thickness(
        &self,
        material: TypedId<MaterialLayerSetUsage>,
    ) -> f64 {
        let layer_set_usage = self.ifc.data.get::<MaterialLayerSetUsage>(material.id());
        let layer_set = self
            .ifc
            .data
            .get::<MaterialLayerSet>(layer_set_usage.spatial_element_structure);

        layer_set
            .material_layers
            .0
            .iter()
            .map(|layer_id| {
                let layer = self.ifc.data.get::<MaterialLayer>(*layer_id);
                layer.layer_thickness.0
            })
            .sum()
    }

    fn wall(
        &mut self,
        material: TypedId<MaterialLayerSetUsage>,
        wall_type: TypedId<WallType>,
        wall: Wall,
    ) {
        let wall_id = self.ifc.data.insert_new(wall);

        self.walls.insert(wall_id);
        self.wall_type_to_wall
            .get_mut(&wall_type)
            .unwrap()
            .insert(wall_id);
        self.material_to_wall
            .get_mut(&material)
            .unwrap()
            .insert(wall_id);
    }

    pub fn wall_type(
        &mut self,
        material: TypedId<MaterialLayerSet>,
        name: &str,
        wall_type: WallTypeEnum,
    ) -> TypedId<WallType> {
        let wall_type = WallType::new(name, wall_type)
            .owner_history(self.owner_history, self.ifc)
            .name(name);

        let wall_type_id = self.ifc.data.insert_new(wall_type);

        self.wall_type_to_wall.insert(wall_type_id, HashSet::new());
        self.material_to_wall_type
            .get_mut(&material)
            .unwrap()
            .insert(wall_type_id);

        wall_type_id
    }

    pub fn material_layer(
        &mut self,
        name: &str,
        thickness: f64,
        is_ventilated: bool,
    ) -> TypedId<MaterialLayer> {
        let material = self.ifc.data.insert_new(Material::new(name));

        let material_layer =
            MaterialLayer::new(thickness, is_ventilated).material(material, self.ifc);

        self.ifc.data.insert_new(material_layer)
    }

    pub fn material_layer_set(
        &mut self,
        layer: impl IntoIterator<Item = TypedId<MaterialLayer>>,
    ) -> TypedId<MaterialLayerSet> {
        let mut material_layer_set = MaterialLayerSet::new();

        for layer in layer {
            material_layer_set = material_layer_set.add_layer(layer, self.ifc);
        }

        let id = self.ifc.data.insert_new(material_layer_set);
        self.material_to_wall_type.insert(id, HashSet::new());

        id
    }

    pub fn material_layer_set_usage(
        &mut self,
        material_layer_set: TypedId<MaterialLayerSet>,
        direction: LayerSetDirectionEnum,
        sense: DirectionSenseEnum,
        offset: f64,
    ) -> TypedId<MaterialLayerSetUsage> {
        let material_layer_set_usage =
            MaterialLayerSetUsage::new(material_layer_set, direction, sense, offset, self.ifc);

        let id = self.ifc.data.insert_new(material_layer_set_usage);
        self.material_to_wall.insert(id, HashSet::new());

        id
    }
}

impl<'a> Drop for IfcBuildingBuilder<'a> {
    fn drop(&mut self) {
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

        // relate material to wall
        for (index, (material, walls)) in self.material_to_wall.iter().enumerate() {
            let mut material_wall_association =
                RelAssociatesMaterial::new(format!("MaterialToWall{index}"), *material, self.ifc)
                    .owner_history(self.owner_history, self.ifc);

            for wall in walls {
                material_wall_association = material_wall_association.relate_wall(*wall, self.ifc);
            }

            self.ifc.data.insert_new(material_wall_association);
        }

        // relate material to wall type
        for (index, (material, wall_types)) in self.material_to_wall_type.iter().enumerate() {
            let mut wall_type_material_association = RelAssociatesMaterial::new(
                format!("MaterialToWallType{index}"),
                *material,
                self.ifc,
            )
            .owner_history(self.owner_history, self.ifc);

            for wall_type in wall_types {
                wall_type_material_association =
                    wall_type_material_association.relate_wall_type(*wall_type, self.ifc);
            }

            self.ifc.data.insert_new(wall_type_material_association);
        }

        // relate building to walls
        let mut spatial_relation: RelContainedInSpatialStructure =
            RelContainedInSpatialStructure::new("BuildingToWall", self.building, self.ifc)
                .owner_history(self.owner_history, self.ifc);

        for wall in self.walls.iter() {
            spatial_relation = spatial_relation.relate_structure(*wall, self.ifc);
        }

        self.ifc.data.insert_new(spatial_relation);
    }
}
