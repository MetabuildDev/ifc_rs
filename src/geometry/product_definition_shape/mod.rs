pub mod deserialize;
pub mod serialize;

use crate::{
    id::Id,
    ifc_type::IfcType,
    parser::{label::Label, list::IfcList, optional::OptionalParameter},
    IFC,
};

use super::shape_representation::ShapeRepresentation;

/// The IfcProductDefinitionShape defines all shape relevant information about an IfcProduct. It
/// allows for multiple geometric shape representations of the same product. The shape relevant
/// information includes:
///
/// - the shape representation including geometric representation items (for 3D solids, 2D
///   annotations, etc.) and:
///   - associated presentation information (line color, line type, surface rendering properties)
///   - assignment to presentation layers (CAD layers for visibility control)
/// - or the topological representation items for connectivity systems (vertex, edge, face
///   representations) that may include geometric representation items (vertex points, edge curves,
///   face surfaces)
pub struct ProductDefinitionShape {
    // from IfcProductRepresentation https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcproductrepresentation.htm
    //
    /// The word or group of words by which the product representation is known.
    pub name: OptionalParameter<Label>,
    // TODO: This should be TEXT instead
    /// The word or group of words that characterize the product representation. It can be used to
    /// add additional meaning to the name of the product representation.
    pub description: OptionalParameter<Label>,
    /// Contained list of representations (including shape representations). Each member defines a
    /// valid representation of a particular type within a particular representation context.
    pub representations: IfcList<Id>,
}

impl ProductDefinitionShape {
    pub fn new<NAME: Into<Label>, DESC: Into<Label>>(
        name: impl Into<Option<NAME>>,
        description: impl Into<Option<DESC>>,
        represenations: impl IntoIterator<Item = ShapeRepresentation>,
        ifc: &mut IFC,
    ) -> Self {
        let id = represenations
            .into_iter()
            .map(|reprs| ifc.data.insert_new(reprs).id())
            .collect();

        Self {
            name: name.into().map(|s| s.into()).into(),
            description: description.into().map(|s| s.into()).into(),
            representations: IfcList(id),
        }
    }
}

impl IfcType for ProductDefinitionShape {}

#[cfg(test)]
pub mod test {
    use glam::DVec3;

    use crate::{
        geometry::{
            axis::Axis3D, dimension_count::DimensionCount,
            geometric_projection::GeometricProjection, point::Point3D, polyline::PolyLine,
            representation_context::GeometricRepresentationContext,
            representation_subcontext::GeometricRepresentationSubContext,
            shape_representation::ShapeRepresentation,
        },
        id::IdOr,
        IFC,
    };

    use super::ProductDefinitionShape;

    pub fn new_product_definition_shape(
        ifc: &mut IFC,
        world_coord_system: IdOr<Axis3D>,
    ) -> ProductDefinitionShape {
        let context = GeometricRepresentationContext::new(
            "Model",
            DimensionCount::Three,
            0.01,
            world_coord_system,
            ifc,
        );

        let sub_context = GeometricRepresentationSubContext::derive(
            context,
            None,
            GeometricProjection::ModelView,
            None,
            ifc,
        );

        let shapes = vec![{
            let mut s = ShapeRepresentation::new(sub_context, "first_shape", "", ifc);
            s.add_item(
                PolyLine::from_3d(
                    [
                        DVec3::new(0.0, 0.0, 0.0).into(),
                        DVec3::new(1.0, 0.0, 0.0).into(),
                        DVec3::new(1.0, 1.0, 0.0).into(),
                        DVec3::new(0.0, 1.0, 0.0).into(),
                    ]
                    .into_iter(),
                    ifc,
                ),
                ifc,
            );

            s
        }];

        ProductDefinitionShape::new::<&str, &str>(None, None, shapes, ifc)
    }

    #[test]
    fn create_product_definition_shape() {
        let mut ifc = IFC::default();

        let axis = Axis3D::new(Point3D::from(DVec3::new(0.0, 0.0, 0.0)), &mut ifc);
        let axis_id = ifc.data.insert_new(axis);

        let shape = new_product_definition_shape(&mut ifc, axis_id);
        ifc.data.insert_new(shape);

        println!("{}", ifc.data);
    }
}
