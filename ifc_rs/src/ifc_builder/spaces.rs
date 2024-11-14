use std::collections::HashSet;

use bevy_math::{DVec2, DVec3};

use crate::prelude::*;

pub struct SpaceParameter {
    pub coords: Vec<DVec2>,
    pub height: f64,
    pub placement: DVec3,
}

impl<'a> IfcStoreyBuilder<'a> {
    #[must_use]
    pub fn space(
        &mut self,
        space_type: TypedId<SpaceType>,
        name: &str,
        space_information: SpaceParameter,
    ) -> TypedId<Space> {
        // TODO: add the footprint curve as an additional shaperepresentation to the space's
        // `ProductDefinitionShape.representations` vec
        let product_shape = ProductDefinitionShape::new_horizontal_arbitrary_shape(
            space_information.coords.into_iter(),
            space_information.height,
            self.sub_context,
            &mut self.project.ifc,
        );

        let position = Axis3D::new(
            Point3D::from(space_information.placement),
            &mut self.project.ifc,
        );
        let local_placement =
            LocalPlacement::new_relative(position, self.storey, &mut self.project.ifc);

        let space = Space::new(name)
            .owner_history(self.owner_history, &mut self.project.ifc)
            .object_placement(local_placement, &mut self.project.ifc)
            .representation(product_shape, &mut self.project.ifc);

        let space_id = self.project.ifc.data.insert_new(space);

        self.spaces.insert(space_id);
        self.space_type_to_space
            .entry(space_type)
            .or_default()
            .insert(space_id);

        space_id
    }

    #[must_use]
    pub fn space_type(&mut self, name: &str, space_type: SpaceTypeEnum) -> TypedId<SpaceType> {
        let space_type = SpaceType::new(name, space_type)
            .owner_history(self.owner_history, &mut self.project.ifc)
            .name(name);

        let space_type_id = self.project.ifc.data.insert_new(space_type);

        self.space_type_to_space
            .insert(space_type_id, HashSet::new());

        space_type_id
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use bevy_math::{DVec2, DVec3};

    use crate::ifc_builder::spaces::SpaceParameter;
    use crate::prelude::*;

    use super::super::test::create_builder;

    #[test]
    fn builder_spaces() {
        let mut builder = create_builder();

        {
            let mut site_builder = builder.new_site("test", DVec3::ZERO);
            let mut building_builder = site_builder.new_building("test", DVec3::ZERO);
            let mut storey_builder = building_builder.new_storey("test", 0.0);

            let story_footprint = vec![
                DVec2::ZERO,
                DVec2::new(0.0, 4.0),
                DVec2::new(2.0, 6.0),
                DVec2::new(4.0, 4.0),
                DVec2::new(4.0, 0.0),
                DVec2::ZERO,
            ];

            let space_type = storey_builder.space_type("ExampleWallType", SpaceTypeEnum::Space);
            let _space = storey_builder.space(
                space_type,
                "ExampleSpaceDefault",
                SpaceParameter {
                    coords: story_footprint.clone(),
                    height: 4.0,
                    placement: DVec3::new(0.0, 0.0, 0.0),
                },
            );
        }

        let s = builder.build();
        let ifc = IFC::from_str(&s).unwrap();

        assert_eq!(s, ifc.to_string());
    }
}
