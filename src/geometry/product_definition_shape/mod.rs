pub mod deserialize;
pub mod serialize;

use ifc_verify_derive::IfcVerify;

use crate::{
    id::{IdOr, TypedId},
    ifc_type::{IfcType, IfcVerify},
    parser::{label::Label, list::IfcList, optional::OptionalParameter},
    prelude::*,
};

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
#[derive(IfcVerify)]
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
    pub representations: IfcList<TypedId<ShapeRepresentation>>,
}

impl ProductDefinitionShape {
    pub fn new() -> Self {
        Self {
            name: OptionalParameter::omitted(),
            description: OptionalParameter::omitted(),
            representations: IfcList::empty(),
        }
    }

    pub fn name(mut self, name: impl Into<Label>) -> Self {
        self.name = name.into().into();
        self
    }

    pub fn description(mut self, description: impl Into<Label>) -> Self {
        self.description = description.into().into();
        self
    }

    pub fn add_representation(
        mut self,
        representation: impl Into<IdOr<ShapeRepresentation>>,
        ifc: &mut IFC,
    ) -> Self {
        self.representations
            .0
            .push(representation.into().or_insert(ifc));

        self
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
        let context =
            GeometricRepresentationContext::new(DimensionCount::Three, world_coord_system, ifc);

        let sub_context =
            GeometricRepresentationSubContext::derive(context, GeometricProjection::ModelView, ifc);

        let shape = ShapeRepresentation::new(sub_context, ifc).add_item(
            PolyLine::from(
                [
                    Point3D::from(DVec3::new(0.0, 0.0, 0.0)),
                    Point3D::from(DVec3::new(1.0, 0.0, 0.0)),
                    Point3D::from(DVec3::new(1.0, 1.0, 0.0)),
                    Point3D::from(DVec3::new(0.0, 1.0, 0.0)),
                ]
                .into_iter(),
                ifc,
            ),
            ifc,
        );

        ProductDefinitionShape::new().add_representation(shape, ifc)
    }

    #[test]
    fn create_product_definition_shape() {
        let mut ifc = IFC::default();

        let axis = Axis3D::new(Point3D::from(DVec3::new(0.0, 0.0, 0.0)), &mut ifc);
        let axis_id = ifc.data.insert_new(axis);

        let shape = new_product_definition_shape(&mut ifc, axis_id.into());
        ifc.data.insert_new(shape);

        println!("{}", ifc.data);
    }
}
