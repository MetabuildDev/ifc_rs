use std::str::FromStr;

use chrono::{DateTime, Utc};
use strum::VariantNames;
use winnow::combinator::{alt, delimited, separated};
use winnow::prelude::*;

use super::description::{FileDescription, ImplementationLevel, ViewDefinition};
use super::details::{
    Author, Authorization, Organization, OriginatingSystem, PreprocessorVersion, TimeStamp,
};
use super::details::{FileDetails, FileName};
use super::schema::{FileSchema, FileSchemas};
use super::version::Version;
use super::Header;
use crate::parser::comma::Comma;
use crate::parser::*;

fn p_header<'a>() -> impl IFCParser<'a, Header> {
    winnow::seq! {
        Header {
            _: p_space_or_comment(),
            version: p_header_version(),
            _: p_space_or_comment_surrounded("HEADER;"),
            description: p_header_description(),
            _: p_space_or_comment(),
            name: p_header_name(),
            _: p_space_or_comment(),
            schema: p_header_schema(),
            _: p_space_or_comment_surrounded("ENDSEC;"),
        }
    }
}

fn p_header_version<'a>() -> impl IFCParser<'a, Version> {
    let variants: [&str; Version::VARIANTS.len()] =
        Version::VARIANTS.try_into().expect("statically known");

    delimited(
        p_space_or_comment(),
        alt(variants
            .map(|v| (v, Version::from_str(v).expect("valid version")))
            .map(|(k, v)| k.map(move |_| v))),
        (";", p_space_or_comment()),
    )
}

fn p_header_description<'a>() -> impl IFCParser<'a, FileDescription> {
    winnow::seq! {
        FileDescription {
            _: p_space_or_comment_surrounded(("FILE_DESCRIPTION", p_space_or_comment(), "(")),
            descriptions: p_header_desc_desc(),
            _: Comma::parse(),
            implementation_level: delimited("'", p_header_desc_level(), "'"),
            _: p_space_or_comment_surrounded((")", p_space_or_comment(), ";"))
        }
    }
}

fn p_header_desc_desc<'a>() -> impl IFCParser<'a, Vec<ViewDefinition>> {
    let p_view_def = p_view_definition();
    let p_item = p_space_or_comment_surrounded(delimited("'", p_view_def, "'"));
    delimited("(", separated(.., p_item, ","), ")")
}

fn p_view_definition<'a>() -> impl IFCParser<'a, ViewDefinition> {
    let mut p_items = delimited(
        '[',
        separated(.., p_space_or_comment_surrounded(p_ident()), ','),
        ']',
    );
    winnow::seq! {
        ViewDefinition {
            _: p_space_or_comment(),
            name: p_ident(),
            _: p_space_or_comment(),
            items: p_items,
            _: p_space_or_comment(),
        }
    }
}

fn p_header_desc_level<'a>() -> impl IFCParser<'a, ImplementationLevel> {
    let variants: [&str; ImplementationLevel::VARIANTS.len()] = ImplementationLevel::VARIANTS
        .try_into()
        .expect("statically known");

    p_space_or_comment_surrounded(alt(variants
        .map(|v| (v, ImplementationLevel::from_str(v).expect("valid version")))
        .map(|(k, v)| k.map(move |_| v))))
}

fn p_header_name<'a>() -> impl IFCParser<'a, FileDetails> {
    // TODO: Better ideas for failing here
    fn time_from_string(s: String) -> DateTime<Utc> {
        let date_res = DateTime::parse_from_rfc3339(s.as_str())
            .or_else(|_| DateTime::parse_from_rfc3339(format!("{s}Z").as_str()));
        date_res.unwrap().into()
    }
    let mut p_name = p_quote_word().map(FileName);
    let mut p_time = p_quote_word().map(time_from_string).map(TimeStamp);
    let mut p_author = delimited(
        "(",
        separated(
            ..,
            p_space_or_comment_surrounded(p_quote_word()).map(Author),
            ",",
        ),
        ")",
    );
    let mut p_org = delimited(
        "(",
        separated(
            ..,
            p_space_or_comment_surrounded(p_quote_word()).map(Organization),
            ",",
        ),
        ")",
    );
    winnow::seq! {
        FileDetails {
            _: p_space_or_comment_surrounded(("FILE_NAME", p_space_or_comment(), "(")),
            name: p_name,
            _: Comma::parse(),
            timestamp: p_time,
            _: Comma::parse(),
            author: p_author,
            _: Comma::parse(),
            organization: p_org,
            _: Comma::parse(),
            preprocessor_version: p_quote_word().map(PreprocessorVersion),
            _: Comma::parse(),
            originating_system: p_quote_word().map(OriginatingSystem),
            _: Comma::parse(),
            authorization: p_quote_word().map(Authorization),
            _: p_space_or_comment_surrounded((")", p_space_or_comment(), ";"))
        }
    }
}

fn p_header_schema<'a>() -> impl IFCParser<'a, FileSchemas> {
    winnow::seq! {
        FileSchemas (
            _: p_space_or_comment_surrounded(("FILE_SCHEMA", p_space_or_comment(), "(")),
            p_schema_outer(),
            _: p_space_or_comment_surrounded((")", p_space_or_comment(), ";"))
        )
    }
}

fn p_schema_outer<'a>() -> impl IFCParser<'a, Vec<FileSchema>> {
    delimited(
        "(",
        separated(
            ..,
            p_space_or_comment_surrounded(delimited("'", p_schema(), "'")),
            ",",
        ),
        ")",
    )
}

fn p_schema<'a>() -> impl IFCParser<'a, FileSchema> {
    let variants: [&str; FileSchema::VARIANTS.len()] =
        FileSchema::VARIANTS.try_into().expect("statically known");

    p_space_or_comment_surrounded(alt(variants
        .map(|v| (v, FileSchema::from_str(v).expect("valid version")))
        .map(|(k, v)| k.map(move |_| v))))
}

#[test]
fn parse_header_works() {
    let data_with_comment = r#"ISO-10303-21;
HEADER;

/******************************************************************************************
* STEP Physical File produced by: The EXPRESS Data Manager Version 5.02.0100.07 : 28 Aug 2013
* Module:                         EDMstepFileFactory/EDMstandAlone
* Creation date:                  Mon Jun 29 16:13:56 2020
* Host:                           WS-033
* Database:                       C:\Users\SchmitzH\AppData\Local\Temp\{CB542990-BA47-47C9-AFAF-CE952D10F79A}\ifc
* Database version:               5507
* Database creation date:         Mon Jun 29 15:15:06 2020
* Schema:                         IFC2X3
* Model:                          DataRepository.ifc
* Model creation date:            Mon Jun 29 15:15:07 2020
* Header model:                   DataRepository.ifc_HeaderModel
* Header model creation date:     Mon Jun 29 15:15:07 2020
* EDMuser:                        sdai-user
* EDMgroup:                       sdai-group
* License ID and type:            5605 : Permanent license. Expiry date: 
* EDMstepFileFactory options:     020000
******************************************************************************************/
FILE_DESCRIPTION(('ViewDefinition [CoordinationView_V2.0, QuantityTakeOffAddOnView]'),'2;1');
FILE_NAME('23022','2020-06-29T16:13:56',(''),('RKW Architektur +'),'The EXPRESS Data Manager Version 5.02.0100.07 : 28 Aug 2013','20190510_1515(x64) - Exporter 18.3.3.18 - Alternativ-UI 18.3.3.18','');
FILE_SCHEMA(('IFC2X3'));
ENDSEC;
    "#;
    let data_without_comment = r#"ISO-10303-21;
HEADER;

FILE_DESCRIPTION(('ViewDefinition [CoordinationView_V2.0, QuantityTakeOffAddOnView]'),'2;1');
FILE_NAME('23022','2020-06-29T16:13:56',(''),('RKW Architektur +'),'The EXPRESS Data Manager Version 5.02.0100.07 : 28 Aug 2013','20190510_1515(x64) - Exporter 18.3.3.18 - Alternativ-UI 18.3.3.18','');
FILE_SCHEMA(('IFC2X3'));
ENDSEC;
    "#;

    let header_1 = p_header().parse(data_with_comment).unwrap();
    let header_2 = p_header().parse(data_without_comment).unwrap();

    assert_eq!(header_1, header_2);

    println!("{header_1:#?}");

    let back_to_string = header_1.to_string();
    println!("{back_to_string}");
}
