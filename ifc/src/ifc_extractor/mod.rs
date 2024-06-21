pub mod prelude;

use std::ops::Deref;

use crate::{ifc_type::IfcType, prelude::*};

pub struct IfcExtractor {
    ifc: IFC,
}

impl IfcExtractor {
    pub fn projects(&self) -> Vec<(TypedId<Project>, &Project)> {
        self.ifc.data.find_all_of_type::<Project>().collect()
    }

    pub fn relations_of<RELATING, RELATED>(
        &self,
        id: TypedId<RELATING>,
    ) -> Vec<(TypedId<RELATED>, &RELATED)>
    where
        RELATING: IfcType,
        RELATED: IfcType,
    {
        self.ifc
            .data
            .find_all_of_type::<RelAggregates>()
            .filter(|&(_, rel_aggregate)| (rel_aggregate.relating_object == id.id()))
            .flat_map(|(_, rel_aggregate)| {
                rel_aggregate.related_objects.0.iter().filter_map(|id| {
                    self.ifc
                        .data
                        .get_untyped(*id)
                        .downcast_ref::<RELATED>()
                        .map(|s| (TypedId::<RELATED>::new(*id), s))
                })
            })
            .collect()
    }

    pub fn contained_structures<S>(&self, id: TypedId<S>) -> Vec<Id>
    where
        S: Structure,
    {
        self.ifc
            .data
            .find_all_of_type::<RelContainedInSpatialStructure>()
            .filter(|&(_, rel_structure)| (rel_structure.relating_structure == id.id()))
            .flat_map(|(_, rel_structure)| rel_structure.related_elements.0.clone())
            .collect()
    }

    pub fn related_type<S>(&self, id: TypedId<S>) -> &dyn IfcType
    where
        S: Structure,
    {
        self.ifc
            .data
            .find_all_of_type::<RelDefinesByType>()
            .find(|(_, rel_types)| {
                rel_types
                    .related_objects
                    .0
                    .iter()
                    .any(|rel_id| *rel_id == id.id())
            })
            .map(|(_, rel_types)| self.ifc.data.get_untyped(rel_types.relating_type))
            .unwrap()
    }

    pub fn related_materials<S>(&self, id: TypedId<S>) -> Vec<&MaterialLayer>
    where
        S: Structure,
    {
        self.ifc
            .data
            .find_all_of_type::<RelAssociatesMaterial>()
            .filter_map(|(_, rel_material)| {
                rel_material
                    .related_objects
                    .0
                    .iter()
                    .any(|obj_id| *obj_id == id.id())
                    .then(|| {
                        self.ifc
                            .data
                            .get_untyped(rel_material.relating_material)
                            .downcast_ref::<MaterialLayerSetUsage>()
                            .map(|set_usage| {
                                self.ifc
                                    .data
                                    .get(set_usage.spatial_element_structure)
                                    .material_layers
                                    .0
                                    .iter()
                                    .map(|material_layer_id| self.ifc.data.get(*material_layer_id))
                            })
                    })
                    .flatten()
            })
            .flatten()
            .collect()
    }

    pub fn related_constiuents<S>(&self, id: TypedId<S>) -> Vec<&MaterialConstituent>
    where
        S: Structure,
    {
        self.ifc
            .data
            .find_all_of_type::<RelAssociatesMaterial>()
            .filter_map(|(_, rel_material)| {
                rel_material
                    .related_objects
                    .0
                    .iter()
                    .any(|obj_id| *obj_id == id.id())
                    .then(|| {
                        self.ifc
                            .data
                            .get_untyped(rel_material.relating_material)
                            .downcast_ref::<MaterialConstituentSet>()
                            .map(|constituent_set| {
                                constituent_set
                                    .material_constituents
                                    .0
                                    .iter()
                                    .map(|constituent_id| self.ifc.data.get(*constituent_id))
                            })
                    })
                    .flatten()
            })
            .flatten()
            .collect()
    }
}

impl From<IFC> for IfcExtractor {
    fn from(ifc: IFC) -> Self {
        Self { ifc }
    }
}

impl Deref for IfcExtractor {
    type Target = IFC;

    fn deref(&self) -> &Self::Target {
        &self.ifc
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use anyhow::Result;

    #[test]
    fn archicad_file_extractor() -> Result<()> {
        let ifc = IfcExtractor::from(IFC::from_file("resources/AC20-FZK-Haus.ifc")?);

        let projects = ifc.projects();

        println!("project count: {}", projects.len());

        let sites = projects
            .iter()
            .flat_map(|(id, _)| ifc.relations_of::<Project, Site>(*id))
            .collect::<Vec<_>>();

        println!("site count: {}", sites.len());

        let buildings = sites
            .iter()
            .flat_map(|(id, _)| ifc.relations_of::<Site, Building>(*id))
            .collect::<Vec<_>>();

        println!("building count: {}", buildings.len());

        let storeys = buildings
            .iter()
            .flat_map(|(id, _)| ifc.relations_of::<Building, Storey>(*id))
            .collect::<Vec<_>>();

        println!("storey count: {}", storeys.len());

        for (storey_index, (storey_id, _storey)) in storeys.into_iter().enumerate() {
            let spaces = ifc.relations_of::<Storey, Space>(storey_id);
            let storey_structures = ifc.contained_structures(storey_id);

            println!(
                "\tstorey {} has {} space(s) and {} related structures",
                storey_index + 1,
                spaces.len(),
                storey_structures.len()
            );

            storey_structures.iter().for_each(|id| {
                let structure_ifc_type = ifc.data.get_untyped(*id);
                println!("\t\t\tstructure name: {}", structure_ifc_type.type_name());

                if let Some(structure) = structure_ifc_type.to_structure() {
                    if let Some(structure_type) = structure.structure_type() {
                        match structure_type {
                            StructureType::Wall(wall) => {
                                let wall_id = TypedId::<Wall>::new(*id);

                                let wall_type = ifc
                                    .related_type(wall_id)
                                    .downcast_ref::<WallType>()
                                    .unwrap();
                                let materials = ifc.related_materials(wall_id);
                                let shapes = wall.shapes(&ifc);

                                println!(
                                    "\t\t\t\twall with wall type {:?}, materials {} and shapes {}",
                                    wall_type.predefined_type,
                                    materials.len(),
                                    shapes.len()
                                );

                                for (shape_index, shape) in shapes.iter().enumerate() {
                                    for (item_index, item) in shape.items(&ifc).enumerate() {
                                        println!(
                                            "\t\t\t\t\titem {} of shape {} is {}",
                                            item_index, shape_index, item
                                        );
                                    }
                                }
                            }
                            StructureType::Slab(_slab) => (),
                            StructureType::Roof(_roof) => (),
                            StructureType::Window(_window) => (),
                        }
                    }
                }

                if let Some(dummy) = structure_ifc_type.downcast_ref::<Dummy>() {
                    println!("\t\t\t\t{}", dummy.s);
                }
            });

            for (space_index, (space_id, _space)) in spaces.into_iter().enumerate() {
                let space_structures = ifc.contained_structures(space_id);

                println!(
                    "\t\tspace {} of storey {} has {} related structures",
                    space_index + 1,
                    storey_index + 1,
                    space_structures.len()
                );
            }
        }

        Ok(())
    }
}
