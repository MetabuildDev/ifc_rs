use crate::parser::{
    comma::Comma,
    list::IfcList,
    optional::{IFCParse, OptionalParameter},
    p_space_or_comment_surrounded,
};

use super::ProductDefinitionShape;

impl IFCParse for ProductDefinitionShape {
    fn parse<'a>() -> impl crate::parser::IFCParser<'a, Self>
    where
        Self: Sized,
    {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCPRODUCTDEFINITIONSHAPE("),
                name: OptionalParameter::parse(),
                _: Comma::parse(),
                description: OptionalParameter::parse(),
                _: Comma::parse(),
                representations: IfcList::parse(),
                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

#[test]
fn parse_product_definition_shape_works() {
    use winnow::prelude::*;

    let data = "IFCPRODUCTDEFINITIONSHAPE($,$,(#256));";
    let parsed = ProductDefinitionShape::parse().parse(data).unwrap();
    assert_eq!(format!("{data}"), format!("{parsed}"));
}
