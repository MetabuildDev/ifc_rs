use crate::{id::Id, parser::list::IfcList};

mod deserialize;
mod serialize;

/// The IfcPolyline is a bounded curve with only linear segments defined by a list of Cartesian
/// points. If the first and the last Cartesian point in the list are identical, then the polyline
/// is a closed curve, otherwise it is an open curve.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcpolyline.htm
pub struct PolyLine {
    /// The points defining the polyline.
    pub points: IfcList<Id>,
}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::PolyLine;
    use crate::parser::optional::IFCParse;

    #[test]
    fn poly_line_round_trip() {
        let example = "IFCPOLYLINE((#395,#397,#399,#401,#403,#405,#407,#409,#411,#395));";

        let poly_line: PolyLine = PolyLine::parse().parse(example).unwrap();
        let str_poly_line = poly_line.to_string();

        assert_eq!(example, str_poly_line);
    }
}
