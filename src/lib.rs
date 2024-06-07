#![allow(dead_code)]

use anyhow::{anyhow, Result};
use parser::IFCParse;
use std::{fs, path::Path};
use winnow::{seq, Parser};

use meta::{datamap::DataMap, footer::Footer, header::Header};

pub mod geometry;
pub mod id;
pub mod ifc_type;
pub mod material;
pub mod meta;
pub mod objects;
pub mod parser;
pub mod relations;
pub mod units;

pub struct IFC {
    pub header: Header,

    pub data: DataMap,

    pub footer: Footer,
}

impl IFC {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let contents = fs::read_to_string(path)?;
        let mut s = contents.as_str();

        let me = seq!(Self {
            header: Header::parse(),
            data: DataMap::parse(),
            footer: Footer::parse(),
        })
        .parse_next(&mut s)
        .map_err(|err| anyhow!("parsing failed: {err:#?}"))?;

        Ok(me)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        geometry::{
            axis::Axis3D, product_definition_shape::ProductDefinitionShape,
            representation_context::GeometricRepresentationContext,
            representation_subcontext::GeometricRepresentationSubContext,
            shape_representation::ShapeRepresentation,
        },
        id::IdOr,
    };

    use super::IFC;
    use anyhow::Result;

    #[test]
    fn load_file() -> Result<()> {
        use crate::objects::wall::Wall;

        let ifc = IFC::from_file("resources/wall-standard-case.ifc")?;

        for wall in ifc.data.find_all_of_type::<Wall>() {
            println!("wall: {wall}");

            if let Some(owner_history) = wall
                .owner_history
                .custom()
                .map(|&id| ifc.data.get_untyped(id))
            {
                println!("\towner_history: {owner_history}");
            }

            if let Some(id_or) = wall.object_placement.custom() {
                match id_or {
                    IdOr::Id(id) => println!("\tpoint3d: {}", ifc.data.get_untyped(*id)),
                    IdOr::Custom(point) => println!("\tpoint3d: {point}"),
                }
            }

            if let Some(representation) = wall
                .representation
                .custom()
                .map(|&id| ifc.data.get_untyped(id))
            {
                println!("\trepresentation: {representation}");

                for repr in representation
                    .downcast_ref::<ProductDefinitionShape>()
                    .unwrap()
                    .representations
                    .iter()
                {
                    let shape = ifc.data.get::<ShapeRepresentation>(*repr);
                    println!("\t\tshape_representation: {shape}");

                    let sub_context = ifc
                        .data
                        .get::<GeometricRepresentationSubContext>(shape.context_of_items);

                    println!("\t\t\tsub context: {sub_context}");

                    let parent_context = ifc
                        .data
                        .get::<GeometricRepresentationContext>(sub_context.parent_context);

                    println!("\t\t\t\tcontext: {parent_context}");
                    println!(
                        "\t\t\t\t\tcoord_dims: {}",
                        parent_context.coord_space_dimension
                    );

                    let world_coord_system =
                        ifc.data.get::<Axis3D>(parent_context.world_coord_system);

                    println!("\t\t\t\t\tworld_coord_system: {world_coord_system}");
                    println!(
                        "\t\t\t\t\t\tcoord_system_point: {}",
                        ifc.data.get_untyped(world_coord_system.location)
                    );

                    for (index, item) in shape.items(&ifc).enumerate() {
                        println!("\t\t\titem {index}: {item}");
                    }
                }
            }

            if let Some(tag) = wall.tag.custom().map(|&id| ifc.data.get_untyped(id)) {
                println!("\ttag: {tag}");
            }

            if let Some(id_or) = wall.predefined_type.custom() {
                match id_or {
                    IdOr::Id(id) => println!("\twall_type: {}", ifc.data.get_untyped(*id)),
                    IdOr::Custom(wall_type) => println!("\twall_type: {}", wall_type),
                }
            }
        }

        Ok(())
    }
}
