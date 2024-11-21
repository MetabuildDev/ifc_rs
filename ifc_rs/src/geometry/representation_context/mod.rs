mod deserialize;
mod serialize;

use ifc_rs_verify_derive::IfcVerify;

use crate::geometry::dimension_count::DimensionCount;
use crate::id::{Id, IdOr};
use crate::parser::optional::OptionalParameter;
use crate::parser::real::RealPrimitive;
use crate::parser::string::StringPrimitive;
use crate::prelude::*;

/// The IfcGeometricRepresentationContext defines the context that applies to several shape
/// representations of products within a project. It defines the type of the context in which the
/// shape representation is defined, and the numeric precision applicable to the geometric representation
/// items defined in this context. In addition it can be used to offset the project coordinate system
/// from a global point of origin, using the WorldCoordinateSystem attribute.
/// The TrueNorth attribute can be given, if the y axis of the WorldCoordinateSystem does not point to the global northing.
///
/// https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcrepresentationresource/lexical/ifcgeometricrepresentationcontext.htm
#[derive(Debug, Clone, IfcVerify)]
pub struct GeometricRepresentationContext {
    // NOTE: from IfcRepresentationContext
    //
    /// The optional identifier of the representation context as used within a project.
    pub context_identifier: OptionalParameter<RepresentationIdentifier>,
    /// The description of the type of a representation context.
    /// The supported values for context type are to be specified by implementers agreements.
    pub context_type: OptionalParameter<StringPrimitive>,

    /// The integer dimension count of the coordinate space modeled in a geometric representation context.
    pub coord_space_dimension: DimensionCount,
    /// Value of the model precision for geometric models. It is a double value (REAL), typically in 1E-5 to 1E-8 range,
    /// that indicates the tolerance under which two given points are still assumed to be identical.
    /// The value can be used e.g. to sets the maximum distance from an edge curve to the underlying face surface in brep models.
    pub precision: OptionalParameter<RealPrimitive>,
    /// Establishment of the engineering coordinate system (often referred to as the world coordinate
    /// system in CAD) for all representation contexts used by the project.
    #[ifc_types(Axis2D, Axis3D)]
    pub world_coord_system: Id,

    /// Direction of the true north, or geographic northing direction, relative to the underlying project coordinate system.
    /// It is given by a 2 dimensional direction within the xy-plane of the project coordinate system.
    /// If not present, it defaults to 0. 1., meaning that the positive Y axis of the project coordinate system equals the
    /// geographic northing direction.
    #[ifc_types(Direction2D, Direction3D)]
    pub true_north: OptionalParameter<Id>,
}

impl GeometricRepresentationContext {
    pub fn new<A: AxisPlacement>(
        coord_space_dimension: DimensionCount,
        world_coord_system: impl Into<IdOr<A>>,
        ifc: &mut IFC,
    ) -> Self {
        let id = world_coord_system.into().or_insert(ifc);

        Self {
            context_identifier: RepresentationIdentifier::Body.into(),
            context_type: StringPrimitive::from("Model").into(),
            coord_space_dimension,
            precision: OptionalParameter::omitted(),
            world_coord_system: id.id(),
            true_north: OptionalParameter::omitted(),
        }
    }

    pub fn precision(mut self, precision: f64) -> Self {
        self.precision = RealPrimitive(precision).into();
        self
    }

    pub fn true_north<A: AxisPlacement>(
        mut self,
        direction: impl Into<IdOr<A>>,
        ifc: &mut IFC,
    ) -> Self {
        self.true_north = direction.into().or_insert(ifc).id().into();
        self
    }
}

impl IfcType for GeometricRepresentationContext {}
