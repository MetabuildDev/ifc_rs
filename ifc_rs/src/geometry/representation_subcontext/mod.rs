mod deserialize;
mod serialize;

use ifc_rs_verify_derive::IfcVerify;

use crate::geometry::dimension_count::DimensionCount;
use crate::geometry::geometric_projection::GeometricProjection;
use crate::id::{Id, IdOr, TypedId};
use crate::parser::optional::OptionalParameter;
use crate::parser::real::RealPrimitive;
use crate::parser::string::StringPrimitive;
use crate::prelude::*;

use super::representation_context::GeometricRepresentationContext;

/// The IfcGeometricRepresentationSubContext defines the context that applies
/// to several shape representations of a product being a sub context, sharing
/// the WorldCoordinateSystem, CoordinateSpaceDimension, Precision and TrueNorth
/// attributes with the parent IfcGeometricRepresentationContext.
///
/// The IfcGeometricRepresentationSubContext is used to define semantically distinguished
/// representation types for different information content, dependent on the
/// representation view and the target scale. It can be used to control the
/// level of detail of the shape representation that is most applicable to this
/// geometric representation context. addition the sub context is used to
/// control the later appearance of the IfcShapeRepresentation within a plot view.
///
/// Each IfcProduct can then have several instances of subtypes of IfcRepresentation,
/// each being assigned to a different geometric representation context
/// (IfcGeometricRepresentationContext or IfcGeometricRepresentationSubContext).
/// The application can then choose the most appropriate representation for
/// showing the geometric shape of the product, depending on the target view and scale.
///
/// https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcrepresentationresource/lexical/ifcgeometricrepresentationsubcontext.htm
#[derive(Debug, Clone, IfcVerify)]
pub struct GeometricRepresentationSubContext {
    // first six fields inherited from IfcGeometricRepresentationContext
    //
    /// The optional identifier of the representation context as used within a project.
    pub context_identifier: OptionalParameter<RepresentationIdentifier>,
    /// The description of the type of a representation context. The supported values for context
    /// type are to be specified by implementers agreements.
    ///
    /// implementer note: https://validate.buildingsmart.org treats it as an error if this isn't a
    /// string so I'm going to remove the optionality
    pub context_type: OptionalParameter<StringPrimitive>,
    /// The integer dimension count of the coordinate space modeled in a geometric representation
    /// context.
    pub coord_space_dimension: OptionalParameter<DimensionCount>,
    /// Value of the model precision for geometric models. It is a double value (REAL), typically
    /// in 1E-5 to 1E-8 range, that indicates the tolerance under which two given points are still
    /// assumed to be identical. The value can be used e.g. to sets the maximum distance from an
    /// edge curve to the underlying face surface in brep models.
    pub precision: OptionalParameter<RealPrimitive>,
    /// Establishment of the engineering coordinate system (often referred to as the world
    /// coordinate system in CAD) for all representation contexts used by the project.
    ///
    /// NOTE  It can be used to provide better numeric stability if the placement of the
    /// building(s) is far away from the origin. In most cases however it would be set to origin:
    /// (0.,0.,0.) and directions x(1.,0.,0.), y(0.,1.,0.), z(0.,0.,1.).
    ///
    /// If an geographic placement is provided using IfcMapConversion then the
    /// WorldCoordinateSystem atttibute is used to define the offset between the zero point of the
    /// local engineering coordinate system and the geographic reference point to which the
    /// IfcMapConversion offset relates. In preferred practise both points (also called "project
    /// base point" and "survey point") should be coincidental. However it is possible to offset
    /// the geographic reference point from the local zero point.
    #[ifc_types(Axis2D, Axis3D)]
    pub world_coord_system: OptionalParameter<Id>,
    /// Direction of the true north, or geographic northing direction, relative to the underlying
    /// project coordinate system. It is given by a 2 dimensional direction within the xy-plane of
    /// the project coordinate system. If not present, it defaults to 0. 1., meaning that the
    /// positive Y axis of the project coordinate system equals the geographic northing direction.
    ///
    /// NOTE  If a geographic placement is provided using IfcMapConversion then the true north is for information only. In case of inconsistency, the value provided with IfcMapConversion shall take precedence.
    #[ifc_types(Axis2D, Axis3D)]
    pub true_north: OptionalParameter<Id>,

    /// Parent context from which the sub context derives its world coordinate
    /// system, precision, space coordinate dimension and true north.
    pub parent_context: TypedId<GeometricRepresentationContext>,
    /// The target plot scale of the representation to which this representation context applies.
    ///
    /// Scale indicates the target plot scale for the representation sub context,
    /// all annotation styles are given in plot dimensions according to this target plot scale.
    /// If multiple instances of IfcGeometricRepresentationSubContext are given
    /// having the same TargetView value, the target plot scale applies up to the
    /// next smaller scale, or up to unlimited small scale.
    ///
    /// *Note: Scale 1:100 (given as 0.01 within TargetScale) is bigger then 1:200
    /// (given as 0.005 within TargetScale).*
    pub target_scale: OptionalParameter<RealPrimitive>, // TODO: IfcPositiveRatioMeasure
    /// Target view of the representation to which this representation context applies.
    pub target_view: GeometricProjection,
    /// User defined target view, this attribute value shall be given,
    /// if the TargetView attribute is set to USERDEFINED.
    pub user_defined_target_view: OptionalParameter<StringPrimitive>,
}

impl GeometricRepresentationSubContext {
    pub fn derive(
        context: impl Into<IdOr<GeometricRepresentationContext>>,
        target_view: GeometricProjection,
        ifc: &mut IFC,
    ) -> Self {
        Self {
            context_identifier: RepresentationIdentifier::Body.into(),
            context_type: StringPrimitive::from("Model").into(),
            coord_space_dimension: OptionalParameter::inherited(),
            precision: OptionalParameter::inherited(),
            world_coord_system: OptionalParameter::inherited(),
            true_north: OptionalParameter::inherited(),

            parent_context: context.into().or_insert(ifc),
            target_scale: OptionalParameter::omitted(),
            target_view,
            user_defined_target_view: OptionalParameter::omitted(),
        }
    }

    pub fn target_scale(mut self, scale: f64) -> Self {
        self.target_scale = RealPrimitive(scale).into();
        self
    }

    pub fn user_defined_target_view(
        mut self,
        user_defined_target_view: impl Into<StringPrimitive>,
    ) -> Self {
        self.user_defined_target_view = user_defined_target_view.into().into();
        self
    }
}

impl IfcType for GeometricRepresentationSubContext {}
