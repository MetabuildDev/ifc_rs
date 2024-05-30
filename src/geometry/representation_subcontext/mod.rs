mod deserialize;
mod serialize;

use crate::id::Id;
use crate::parser::ifc_float::IfcFloat;
use crate::parser::label::Label;
use crate::parser::optional::OptionalParameter;

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
#[derive(Debug, Clone)]
pub struct GeometricRepresentationSubContext {
    // first six fields inherited from IfcGeometricRepresentationContext
    //
    pub context_identifier: OptionalParameter<Label>,
    pub context_type: OptionalParameter<Label>,
    pub coord_space_dimension: usize,
    pub precision: OptionalParameter<IfcFloat>,
    pub world_coord_system: Id,
    pub true_north: OptionalParameter<Id>,

    /// Parent context from which the sub context derives its world coordinate
    /// system, precision, space coordinate dimension and true north.
    pub parent_context: Id,
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
    pub target_scale: OptionalParameter<IfcFloat>, // TODO: IfcPositiveRatioMeasure
    /// Target view of the representation to which this representation context applies.
    pub target_view: GeometricProjection,
    /// User defined target view, this attribute value shall be given,
    /// if the TargetView attribute is set to USERDEFINED.
    pub user_defined_target_view: OptionalParameter<Label>,
}
