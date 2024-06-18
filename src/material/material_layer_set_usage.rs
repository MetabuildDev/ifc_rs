use std::fmt::Display;

use ifc_verify_derive::IfcVerify;

use crate::{
    id::{Id, IdOr, TypedId},
    ifc_type::{IfcType, IfcVerify},
    parser::{
        comma::Comma, ifc_float::IfcFloat, optional::OptionalParameter,
        p_space_or_comment_surrounded, IFCParse, IFCParser,
    },
    prelude::MaterialLayerSet,
    relations::rel_associates_material::RelatableMaterial,
    IFC,
};

use super::{
    direction_sense_enum::DirectionSenseEnum, layer_set_direction_enum::LayerSetDirectionEnum,
};

/// The IfcMaterialLayerSetUsage determines the usage of IfcMaterialLayerSet
/// in terms of its location and orientation relative to the associated element
/// geometry. The location of material layer set shall be compatible with the
/// building element geometry (that is, material layers shall fit inside the
/// element geometry). The rules to ensure the compatibility depend on the
/// type of the building element.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcmateriallayersetusage.htm
#[derive(IfcVerify)]
pub struct MaterialLayerSetUsage {
    /// The IfcMaterialLayerSet set to which the usage is applied.
    pub spatial_element_structure: TypedId<MaterialLayerSet>,

    /// Orientation of the material layer set relative to element reference
    /// geometry. The meaning of the value of this attribute shall be
    /// specified in the geometry use section for each element. For
    /// extruded shape representation, direction can be given along
    /// the extrusion path (e.g. for slabs) or perpendicular to
    /// it (e.g. for walls).
    pub layer_set_direction: LayerSetDirectionEnum,

    /// Denotes whether the material layer set is oriented in positive
    /// or negative sense along the specified axis (defined by LayerSetDirection).
    /// "Positive" means that the consecutive layers (the IfcMaterialLayer
    /// instances in the list of IfcMaterialLayerSet.MaterialLayers) are
    /// placed face-by-face in the direction of the positive axis as established
    /// by LayerSetDirection: for AXIS2 it would be in +y, for AXIS3 it would
    /// be +z. "Negative" means that the layers are placed face-by-face in the
    /// direction of the negative LayerSetDirection. In both cases, starting
    /// at the material layer set base line.
    pub direction_sense: DirectionSenseEnum,

    /// Offset of the material layer set base line (MlsBase) from reference
    /// geometry (line or plane) of element. The offset can be positive or
    /// negative, unless restricted for a particular building element type
    /// in its use definition or by implementer agreement. A positive value
    /// means, that the MlsBase is placed on the positive side of the reference
    /// line or plane, on the axis established by LayerSetDirection (in case
    /// of AXIS2 into the direction of +y, or in case of AXIS2 into the direction
    /// of +z). A negative value means that the MlsBase is placed on the negative
    /// side, as established by LayerSetDirection (in case of AXIS2 into the
    /// direction of -y).
    pub offset_from_reference_line: IfcFloat,

    ///	Extent of the extrusion of the elements body shape representation to
    /// which the IfcMaterialLayerSetUsage applies. It is used as the
    /// reference value for the upper OffsetValues[2] provided by the
    /// IfcMaterialLayerSetWithOffsets subtype for included material layers.
    pub reference_extent: OptionalParameter<IfcFloat>,
}

impl MaterialLayerSetUsage {
    pub fn new(
        spatial_element_structure: impl Into<IdOr<MaterialLayerSet>>,
        layer_set_direction: LayerSetDirectionEnum,
        direction_sense: DirectionSenseEnum,
        offset_from_reference_line: f64,
        ifc: &mut IFC,
    ) -> Self {
        Self {
            spatial_element_structure: spatial_element_structure.into().or_insert(ifc),
            layer_set_direction,
            direction_sense,
            offset_from_reference_line: offset_from_reference_line.into(),
            reference_extent: OptionalParameter::omitted(),
        }
    }

    pub fn reference_extent(mut self, extent: f64) -> Self {
        self.reference_extent = IfcFloat(extent).into();
        self
    }
}

impl IFCParse for MaterialLayerSetUsage {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCMATERIALLAYERSETUSAGE("),

                spatial_element_structure: Id::parse().map(|id| TypedId::new(id)),
                _: Comma::parse(),
                layer_set_direction: LayerSetDirectionEnum::parse(),
                _: Comma::parse(),
                direction_sense: DirectionSenseEnum::parse(),
                _: Comma::parse(),
                offset_from_reference_line: IfcFloat::parse(),
                _: Comma::parse(),
                reference_extent: OptionalParameter::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for MaterialLayerSetUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCMATERIALLAYERSETUSAGE({},{},{},{},{});",
            self.spatial_element_structure,
            self.layer_set_direction,
            self.direction_sense,
            self.offset_from_reference_line,
            self.reference_extent,
        )
    }
}

impl IfcType for MaterialLayerSetUsage {}
impl RelatableMaterial for MaterialLayerSetUsage {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::MaterialLayerSetUsage;
    use crate::parser::IFCParse;

    #[test]
    fn material_layer_set_usage_round_trip() {
        let example = "IFCMATERIALLAYERSETUSAGE(#39,.AXIS2.,.POSITIVE.,0.,$);";

        let parsed: MaterialLayerSetUsage = MaterialLayerSetUsage::parse().parse(example).unwrap();
        let str = parsed.to_string();

        assert_eq!(example, str);
    }
}
