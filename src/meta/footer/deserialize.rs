use std::str::FromStr;

use optional::IFCParse;
use strum::VariantNames;
use winnow::combinator::{alt, delimited, preceded};
use winnow::prelude::*;

use crate::meta::footer::Footer;
use crate::meta::version::Version;
use crate::parser::*;

impl IFCParse for Footer {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment(),
                version: p_footer_version(),
                _: p_space_or_comment(),
            }
        }
    }
}

fn p_footer_version<'a>() -> impl IFCParser<'a, Version> {
    let variants: [&str; Version::VARIANTS.len()] =
        Version::VARIANTS.try_into().expect("statically known");

    delimited(
        p_space_or_comment(),
        preceded(
            "END-",
            alt(variants
                .map(|v| (v, Version::from_str(v).expect("valid version")))
                .map(|(k, v)| k.map(move |_| v))),
        ),
        (";", p_space_or_comment()),
    )
}

#[test]
fn parse_footer_works() {
    let data_with_comment = r#"
    /* THE FOLLOWING IS A COMMENT OVER THE FOOTER */
        END-ISO-10303-21;
    /* THE FOLLOWING IS A COMMENT AFTER THE FOOTER */
    "#;

    let data_without_comment = r#"END-ISO-10303-21;"#;

    let footer_1 = Footer::parse().parse(data_with_comment).unwrap();
    let footer_2 = Footer::parse().parse(data_without_comment).unwrap();

    assert_eq!(footer_1, footer_2);

    println!("{footer_1:#?}");

    let back_to_string = footer_1.to_string();
    println!("{back_to_string}");
}
