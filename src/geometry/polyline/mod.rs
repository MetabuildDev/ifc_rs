use crate::{id::Id, ifc_type::IfcType, parser::list::IfcList, IFC};

use super::{
    point::{Point2D, Point3D, PointType},
    shape_representation::ShapeItem,
};

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

impl PolyLine {
    pub fn from_2d<'a>(points: impl Iterator<Item = Point2D>, ifc: &mut IFC) -> Self {
        Self {
            points: IfcList(
                points
                    .into_iter()
                    .map(|point| ifc.data.insert_new(point).id())
                    .collect(),
            ),
        }
    }

    pub fn from_3d<'a>(points: impl Iterator<Item = Point3D>, ifc: &mut IFC) -> Self {
        Self {
            points: IfcList(
                points
                    .into_iter()
                    .map(|point| ifc.data.insert_new(point).id())
                    .collect(),
            ),
        }
    }

    pub fn points<'a>(&'a self, ifc: &'a IFC) -> IfcList<PointType<'a>> {
        IfcList(
            self.points
                .iter()
                .map(|point_id| {
                    let point = ifc.data.get_untyped(*point_id);

                    if let Some(point_2d) = point.downcast_ref::<Point2D>() {
                        PointType::D2(point_2d)
                    } else if let Some(point_3d) = point.downcast_ref::<Point3D>() {
                        PointType::D3(point_3d)
                    } else {
                        unreachable!()
                    }
                })
                .collect(),
        )
    }
}

impl IfcType for PolyLine {}
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

        let poly_line = PolyLine::from_2d(points.into_iter(), &mut ifc);
        let _poly_line_id = ifc.data.insert_new(poly_line);

        println!("{}", ifc.data);
    }
}
