use std::fmt::Display;

use crate::{
    id::{Id, IdOr},
    ifc_type::IfcType,
    parser::{
        comma::Comma, optional::OptionalParameter, p_space_or_comment_surrounded, IFCParse,
        IFCParser,
    },
    IFC,
};

use super::axis::AxisPlacement;

/// An IfcLocalPlacement defines the relative placement of a product in
/// relation to the placement of another product or the absolute placement
/// of a product within the geometric representation context of the project.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifclocalplacement.htm
pub struct LocalPlacement {
    /// Reference to object placement that provides the relative placement with
    /// its placement in a grid, local coordinate system or linear referenced placement.
    /// If it is omitted, then in the case of linear placement it is established
    /// by the origin of horizontal alignment of the referenced IfcAlignment Axis.
    /// In the case of local placement it is established by the geometric representation
    /// context.
    pub placement_rel_to: OptionalParameter<Id>,

    /// Geometric placement that defines the transformation from the related
    /// coordinate system into the relating. The placement can be either 2D or 3D,
    /// depending on the dimension count of the coordinate system.
    pub relative_placement: Id,
}

impl LocalPlacement {
    pub fn new<A: AxisPlacement>(relative_placement: impl Into<IdOr<A>>, ifc: &mut IFC) -> Self {
        Self {
            placement_rel_to: OptionalParameter::omitted(),
            relative_placement: relative_placement.into().into_id(ifc).id(),
        }
    }

    pub fn relative_to<A: AxisPlacement>(
        mut self,
        placement_rel_to: impl Into<IdOr<A>>,
        ifc: &mut IFC,
    ) -> Self {
        self.placement_rel_to = placement_rel_to.into().into_id(ifc).id().into();
        self
    }
}

impl IFCParse for LocalPlacement {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCLOCALPLACEMENT("),

                placement_rel_to: OptionalParameter::parse(),
                _: Comma::parse(),
                relative_placement: Id::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for LocalPlacement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCLOCALPLACEMENT({},{});",
            self.placement_rel_to, self.relative_placement
        )
    }
}

impl IfcType for LocalPlacement {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::LocalPlacement;
    use crate::parser::IFCParse;

    #[test]
    fn rel_aggregates_round_trip() {
        let example = "IFCLOCALPLACEMENT($,#36);";

        let local_placement: LocalPlacement = LocalPlacement::parse().parse(example).unwrap();
        let str_local_placement = local_placement.to_string();

        assert_eq!(example, str_local_placement);
    }
}
