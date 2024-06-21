use comma::Comma;
use optional::OptionalParameter;

use crate::{objects::shared::element::Element, parser::*};

use super::Roof;

impl IFCParse for Roof {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCROOF("),

                element: Element::parse(),
                _: Comma::parse(),
                predefined_type: OptionalParameter::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use crate::objects::roof::Roof;
    use crate::parser::IFCParse;

    #[test]
    fn roof_round_trip() {
        let example =
            "IFCROOF('3cUkl32yn9qRSPvBJVyWh4',#42,'Basic Roof:Roof_Flat-4Felt-150Ins-50Scr-150Conc-12Plr:286419',$,'Basic Roof:Roof_Flat-4Felt-150Ins-50Scr-150Conc-12Plr:45441',#34171,#35956,'286419',.NOTDEFINED.);";

        let roof = Roof::parse().parse(example).unwrap();
        let str_roof = roof.to_string();

        assert_eq!(example, str_roof);
    }
}
