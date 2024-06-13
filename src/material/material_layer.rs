use std::fmt::Display;

use crate::{
    id::{Id, IdOr},
    ifc_type::IfcType,
    parser::{
        bool::IfcBool, comma::Comma, ifc_float::IfcFloat, ifc_integer::IfcInteger, label::Label,
        optional::OptionalParameter, p_space_or_comment_surrounded, IFCParse, IFCParser,
    },
    prelude::Material,
    IFC,
};

/// IfcMaterialLayer is a single and identifiable part of an element which is
/// constructed of a number of layers (one or more). Each IfcMaterialLayer has
/// a constant thickness and is located relative to the referencing
/// IfcMaterialLayerSet along the material layer set base (MlsBase).
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcmateriallayer.htm
pub struct MaterialLayer {
    /// Optional reference to the material from which the layer is constructed.
    /// Note that if this value is not given, it does not denote a layer
    /// with no material (an air gap), it only means that the material
    /// is not specified at that point.
    pub material: OptionalParameter<Id>,

    /// The thickness of the material layer. The meaning of "thickness"
    /// depends on its usage. In case of building elements elements
    /// utilizing IfcMaterialLayerSetUsage, the dimension is measured
    /// along the positive LayerSetDirection as specified in IfcMaterialLayerSetUsage.
    pub layer_thickness: IfcFloat,

    /// Indication of whether the material layer represents an air layer (or cavity).
    ///   * set to TRUE if the material layer is an air gap and provides air
    ///     exchange from the layer to the outside air.
    ///   * set to UNKNOWN if the material layer is an air gap and does not
    ///     provide air exchange (or when this information about air exchange of
    ///     the air gap is not available).
    ///   * set to FALSE if the material layer is a solid material layer (the default).
    pub is_ventilated: IfcBool,

    /// The name by which the material layer is known.
    pub name: OptionalParameter<Label>,

    /// Definition of the material layer in more descriptive terms than
    /// given by attributes Name or Category.
    pub description: OptionalParameter<Label>,

    /// Category of the material layer, e.g. the role it has in the layer set
    /// it belongs to (such as 'load bearing', 'thermal insulation' etc.).
    /// The list of keywords might be extended by model view definitions,
    /// however the following keywords shall apply in general:
    ///   * 'LoadBearing' — for all material layers having a load bearing function.
    ///   * 'Insulation' — for all material layers having an insolating function.
    ///   * 'Inner finish' — for the material layer being the inner finish.
    ///   * 'Outer finish' — for the material layer being the outer finish.
    pub category: OptionalParameter<Label>,

    /// The relative priority of the layer, expressed as normalised integer
    /// range [0..100]. Controls how layers intersect in connections and
    /// corners of building elements: a layer from one element protrudes
    /// into (i.e. displaces) a layer from another element in a joint of
    /// these elements if the former element's layer has higher priority
    /// than the latter. The priority value for a material layer in an
    /// element has to be set and maintained by software applications
    /// in relation to the material layers in connected elements.
    pub priority: OptionalParameter<IfcInteger>,
}

impl MaterialLayer {
    pub fn new(layer_thickness: f64, is_ventilated: bool) -> Self {
        Self {
            material: OptionalParameter::omitted(),
            layer_thickness: IfcFloat(layer_thickness),
            is_ventilated: is_ventilated.into(),
            name: OptionalParameter::omitted(),
            description: OptionalParameter::omitted(),
            category: OptionalParameter::omitted(),
            priority: OptionalParameter::omitted(),
        }
    }

    pub fn material(mut self, material: impl Into<IdOr<Material>>, ifc: &mut IFC) -> Self {
        self.material = material.into().or_insert(ifc).id().into();
        self
    }

    pub fn name(mut self, name: impl Into<Label>) -> Self {
        self.name = name.into().into();
        self
    }

    pub fn description(mut self, description: impl Into<Label>) -> Self {
        self.description = description.into().into();
        self
    }

    pub fn category(mut self, category: impl Into<Label>) -> Self {
        self.category = category.into().into();
        self
    }

    pub fn priority(mut self, priority: i64) -> Self {
        self.priority = IfcInteger(priority).into();
        self
    }
}

impl IFCParse for MaterialLayer {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCMATERIALLAYER("),

                material: OptionalParameter::parse(),
                _: Comma::parse(),
                layer_thickness: IfcFloat::parse(),
                _: Comma::parse(),
                is_ventilated: IfcBool::parse(),
                _: Comma::parse(),
                name: OptionalParameter::parse(),
                _: Comma::parse(),
                description: OptionalParameter::parse(),
                _: Comma::parse(),
                category: OptionalParameter::parse(),
                _: Comma::parse(),
                priority: OptionalParameter::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for MaterialLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCMATERIALLAYER({},{},{},{},{},{},{});",
            self.material,
            self.layer_thickness,
            self.is_ventilated,
            self.name,
            self.description,
            self.category,
            self.priority,
        )
    }
}

impl IfcType for MaterialLayer {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::MaterialLayer;
    use crate::parser::IFCParse;

    #[test]
    fn material_layer_round_trip() {
        let example = "IFCMATERIALLAYER(#44,110.,.FALSE.,'Finish',$,$,$);";

        let parsed: MaterialLayer = MaterialLayer::parse().parse(example).unwrap();
        let str = parsed.to_string();

        assert_eq!(example, str);
    }
}
