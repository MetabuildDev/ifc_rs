use winnow::{combinator::delimited, Parser};

use crate::{
    geometry::indexed_poly_curve::IndexedPolyCurve,
    id::Id,
    parser::{comma::Comma, optional::OptionalParameter, IFCParse, IFCParser},
};

impl IFCParse for IndexedPolyCurve {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        delimited(
            "IFCINDEXEDPOLYCURVE(",
            (
                Id::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
            ),
            ");",
        )
        .map(|(points, _, segments, _, self_intersect)| Self {
            points,
            segments,
            self_intersect,
        })
    }
}

#[test]
fn parse_indexed_poly_curve_works() {
    let data = "IFCINDEXEDPOLYCURVE(#28,$,.FALSE.);";
    let parsed = IndexedPolyCurve::parse().parse(data).unwrap();
    assert_eq!(data, parsed.to_string());

    // TODO: When IfcLineIndex/IfcArcIndex is implemented
    //
    // let data = "IFCINDEXEDPOLYCURVE(#28,(IfcLineIndex((1,2)),IfcArcIndex((2,3,4)),IfcLineIndex((4,5)),IfcArcIndex((5,6,1))),.FALSE.);";
    // let parsed = IndexedPolyCurve::parse().parse(data).unwrap();
    // assert_eq!(data, parsed.to_string());
}
