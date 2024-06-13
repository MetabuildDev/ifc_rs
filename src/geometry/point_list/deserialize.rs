use winnow::{combinator::delimited, Parser};

use crate::{
    geometry::point_list::{PointList2D, PointList3D},
    parser::{comma::Comma, list::IfcList, optional::OptionalParameter, IFCParse, IFCParser},
};

impl IFCParse for PointList2D {
    fn parse<'a>() -> impl IFCParser<'a, Self>
    where
        Self: Sized,
    {
        delimited(
            "IFCCARTESIANPOINTLIST2D(",
            (IfcList::parse(), Comma::parse(), OptionalParameter::parse()),
            ");",
        )
        .map(|(coord_list, _, tag_list)| Self {
            coord_list,
            tag_list,
        })
    }
}

impl IFCParse for PointList3D {
    fn parse<'a>() -> impl IFCParser<'a, Self>
    where
        Self: Sized,
    {
        delimited(
            "IFCCARTESIANPOINTLIST3D(",
            (IfcList::parse(), Comma::parse(), OptionalParameter::parse()),
            ");",
        )
        .map(|(coord_list, _, tag_list)| Self {
            coord_list,
            tag_list,
        })
    }
}

#[test]
fn parse_point_list_2d_works() {
    let data = "IFCCARTESIANPOINTLIST2D(((0.,0.),(1000.,0.),(1399.,2000.),(1000.,4000.),(0.,4000.),(-400.,2000.)),$);";
    let p = PointList2D::parse().parse(data).unwrap();
    println!("{p:?}");
    assert_eq!(p.to_string(), data);
}

#[test]
fn parse_point_list_3d_works() {
    let data = "IFCCARTESIANPOINTLIST3D(((0.,0.,0.),(1000.,0.,3.),(1.,1399.,2000.),(1.,1000.,4000.),(1.,0.,4000.),(-3.,-400.,2000.)),$);";
    let p = PointList3D::parse().parse(data).unwrap();
    println!("{p:?}");
    assert_eq!(p.to_string(), data);
}
