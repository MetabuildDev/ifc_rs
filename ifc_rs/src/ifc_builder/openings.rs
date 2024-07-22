use glam::{DVec2, DVec3};

use crate::prelude::*;

pub struct CustomDirectionOpeningParameter {
    pub height: f64,
    pub length: f64,
    /// Local to the attached parent
    pub placement: DVec3,
    pub direction: DVec3,
}

pub struct OpeningParameter {
    pub height: f64,
    pub length: f64,
    /// Local to the attached parent
    pub placement: DVec3,
}

pub struct HorizontalArbitraryOpeningParameter {
    pub coords: Vec<DVec2>,
}

pub struct ArbitraryOpeningParameter {
    pub coords: Vec<DVec3>,
}

impl<'a, 'b> IfcWallBuilder<'a, 'b> {
    pub fn vertical_opening(
        &mut self,
        name: &str,
        opening_information: OpeningParameter,
    ) -> TypedId<OpeningElement> {
        self.opening(
            name,
            CustomDirectionOpeningParameter {
                height: opening_information.height,
                length: opening_information.length,
                placement: opening_information.placement,
                direction: DVec3::Z,
            },
        )
    }

    pub fn opening(
        &mut self,
        name: &str,
        opening_information: CustomDirectionOpeningParameter,
    ) -> TypedId<OpeningElement> {
        let position = Axis3D::new(
            Point3D::from(opening_information.placement),
            &mut self.storey.project.ifc,
        );

        let opening_thickness = self.opening_thickness();

        let product_shape = ProductDefinitionShape::new_rectangular_shape(
            opening_information.length,
            opening_information.height,
            opening_thickness,
            Direction3D::from(opening_information.direction),
            self.storey.sub_context,
            &mut self.storey.project.ifc,
        );

        let local_placement =
            LocalPlacement::new_relative(position, self.wall_id, &mut self.storey.project.ifc);
        let opening_element = OpeningElement::new(name)
            .owner_history(self.storey.owner_history, &mut self.storey.project.ifc)
            .representation(product_shape, &mut self.storey.project.ifc)
            .object_placement(local_placement, &mut self.storey.project.ifc);

        let opening_element_id = self.storey.project.ifc.data.insert_new(opening_element);

        self.storey.opening_elements.insert(opening_element_id);
        self.storey
            .opening_elements_to_wall
            .insert(opening_element_id, self.wall_id);

        opening_element_id
    }

    fn opening_thickness(&self) -> f64 {
        let wall_material_set_usage = self
            .storey
            .project
            .material_to_wall
            .iter()
            .find_map(|(mat, associates)| associates.is_related_to(self.wall_id).then_some(mat))
            .copied()
            .unwrap();

        self.storey
            .calculate_material_layer_set_thickness(wall_material_set_usage)
    }
}

impl<'a, 'b> IfcSlabBuilder<'a, 'b> {
    pub fn rect_opening(
        &mut self,
        name: &str,
        opening_information: OpeningParameter,
    ) -> TypedId<OpeningElement> {
        let opening_thickness = self.opening_thickness();

        let slab_direction = self
            .storey
            .slab_direction(self.slab_id)
            .expect("could not find slab extrude direction");

        let product_shape = ProductDefinitionShape::new_rectangular_shape(
            opening_information.length,
            opening_thickness,
            opening_information.height,
            slab_direction,
            self.storey.sub_context,
            &mut self.storey.project.ifc,
        );

        self.opening(name, product_shape, opening_information.placement)
    }

    pub fn horizontal_arbitrary_opening(
        &mut self,
        name: &str,
        opening_information: HorizontalArbitraryOpeningParameter,
    ) -> TypedId<OpeningElement> {
        let opening_thickness = self.opening_thickness();

        let product_shape = ProductDefinitionShape::new_horizontal_arbitrary_shape(
            opening_information.coords.into_iter(),
            opening_thickness,
            self.storey.sub_context,
            &mut self.storey.project.ifc,
        );

        self.opening(name, product_shape, DVec3::new(0.0, 0.0, 0.0))
    }

    pub fn arbitrary_opening(
        &mut self,
        name: &str,
        opening_information: ArbitraryOpeningParameter,
    ) -> TypedId<OpeningElement> {
        let opening_thickness = self.opening_thickness();

        let slab_direction = self
            .storey
            .slab_direction(self.slab_id)
            .expect("could not find slab extrude direction");

        let product_shape = ProductDefinitionShape::new_arbitrary_shape(
            opening_information.coords.into_iter(),
            opening_thickness,
            slab_direction.0 .0,
            self.storey.sub_context,
            &mut self.storey.project.ifc,
        );

        self.opening(name, product_shape, DVec3::new(0.0, 0.0, 0.0))
    }

    fn opening(
        &mut self,
        name: &str,
        product_shape: ProductDefinitionShape,
        placement: DVec3,
    ) -> TypedId<OpeningElement> {
        let position = Axis3D::new(Point3D::from(placement), &mut self.storey.project.ifc);

        let local_placement =
            LocalPlacement::new_relative(position, self.slab_id, &mut self.storey.project.ifc);
        let opening_element = OpeningElement::new(name)
            .owner_history(self.storey.owner_history, &mut self.storey.project.ifc)
            .representation(product_shape, &mut self.storey.project.ifc)
            .object_placement(local_placement, &mut self.storey.project.ifc);

        let opening_element_id = self.storey.project.ifc.data.insert_new(opening_element);

        self.storey.opening_elements.insert(opening_element_id);
        self.storey
            .opening_elements_to_slab
            .insert(opening_element_id, self.slab_id);

        opening_element_id
    }

    fn opening_thickness(&self) -> f64 {
        let slab_material_set_usage = self
            .storey
            .project
            .material_to_slab
            .iter()
            .find_map(|(mat, associates)| associates.is_related_to(self.slab_id).then_some(mat))
            .copied()
            .unwrap();

        self.storey
            .calculate_material_layer_set_thickness(slab_material_set_usage)
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use glam::DVec3;

    use crate::prelude::*;

    use super::super::test::create_builder;

    #[test]
    fn builder_openings() {
        let mut builder = create_builder();

        {
            let mut site_builder = builder.new_site("test", DVec3::ZERO);
            let mut building_builder = site_builder.new_building("test", DVec3::ZERO);
            let mut storey_builder = building_builder.new_storey("test", 0.0);

            let material_layer = storey_builder.material_layer("ExampleMaterial", 0.02, false);
            let material_layer_set = storey_builder.material_layer_set([material_layer]);
            let material_layer_set_usage = storey_builder.material_layer_set_usage(
                material_layer_set,
                LayerSetDirectionEnum::Axis2,
                DirectionSenseEnum::Positive,
                0.0,
            );

            let wall_type = storey_builder.wall_type(
                material_layer_set,
                "ExampleWallType",
                WallTypeEnum::NotDefined,
            );

            let mut wall = storey_builder.vertical_wall(
                material_layer_set_usage,
                wall_type,
                "ExampleWallDefault",
                VerticalWallParameter {
                    height: 2.0,
                    length: 4.0,
                    placement: DVec3::new(0.0, 0.0, 0.0),
                },
            );

            wall.vertical_opening(
                "ExampleOpeningElement",
                OpeningParameter {
                    height: 0.5,
                    length: 0.5,
                    placement: DVec3::new(2.0, 0.0, 0.5),
                },
            );
        }

        let s = builder.build();
        let ifc = IFC::from_str(&s).unwrap();

        assert_eq!(s, ifc.to_string());
    }
}
