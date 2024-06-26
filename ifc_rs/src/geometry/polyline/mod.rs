use ifc_rs_verify_derive::IfcVerify;

use crate::{id::Id, parser::list::IfcList, prelude::*};

mod deserialize;
mod serialize;

/// The IfcPolyline is a bounded curve with only linear segments defined by a list of Cartesian
/// points. If the first and the last Cartesian point in the list are identical, then the polyline
/// is a closed curve, otherwise it is an open curve.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcpolyline.htm
#[derive(IfcVerify)]
pub struct PolyLine {
    /// The points defining the polyline.
    #[ifc_types(Point2D, Point3D)]
    pub points: IfcList<Id>,
}

impl PolyLine {
    pub fn from<C: CartesianPoint>(points: impl Iterator<Item = C>, ifc: &mut IFC) -> Self {
        Self {
            points: IfcList(
                points
                    .map(|point| ifc.data.insert_new(point).id())
                    .collect(),
            ),
        }
    }

    pub fn points<'a>(&'a self, ifc: &'a IFC) -> Points {
        // use the first point to determine whether it is 2D or 3D
        let point = ifc.data.get_untyped(self.points.0[0]);

        if point.downcast_ref::<Point2D>().is_some() {
            Points::D2(
                self.points
                    .iter()
                    .map(|id| ifc.data.get(TypedId::<Point2D>::new(*id)).0 .0)
                    .collect(),
            )
        } else if point.downcast_ref::<Point3D>().is_some() {
            Points::D3(
                self.points
                    .iter()
                    .map(|id| ifc.data.get(TypedId::<Point3D>::new(*id)).0 .0)
                    .collect(),
            )
        } else {
            unreachable!("checked by type checker")
        }
    }
}

impl IfcType for PolyLine {}
impl Curve for PolyLine {}
impl ShapeItem for PolyLine {}

#[cfg(test)]
mod test {
    use glam::DVec2;
    use winnow::Parser;

    use super::PolyLine;
    use crate::{geometry::point::Point2D, parser::IFCParse, IFC};

    #[test]
    fn poly_line_round_trip() {
        let example = "IFCPOLYLINE((#395,#397,#399,#401,#403,#405,#407,#409,#411,#395));";

        let poly_line: PolyLine = PolyLine::parse().parse(example).unwrap();
        let str_poly_line = poly_line.to_string();

        assert_eq!(example, str_poly_line);
    }

    #[test]
    fn construct_poly_line() {
        let mut ifc = IFC::default();

        let points: Vec<Point2D> = vec![
            DVec2::new(0.0, 0.0).into(),
            DVec2::new(4.0, 0.0).into(),
            DVec2::new(4.0, 4.0).into(),
            DVec2::new(0.0, 4.0).into(),
        ];

        let poly_line = PolyLine::from(points.into_iter(), &mut ifc);
        let _poly_line_id = ifc.data.insert_new(poly_line);

        println!("{}", ifc.data);
    }
}
