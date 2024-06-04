use std::collections::BTreeMap;

use winnow::{
    ascii::newline,
    combinator::{preceded, repeat_till, separated_pair},
    token::any,
    Parser,
};

use super::{DataMap, DataValue};
use crate::{
    id::Id,
    parser::{p_space_or_comment_surrounded, IFCParse, IFCParser},
};

fn p_index_map<'a>() -> impl IFCParser<'a, DataMap> {
    let p_obj = repeat_till(.., any, newline).map(|(s, _): (String, _)| DataValue::Any { s });
    let p_line = separated_pair(Id::parse(), p_space_or_comment_surrounded("="), p_obj);
    let p_line_spaced = p_space_or_comment_surrounded(p_line);
    let p_lines =
        repeat_till(.., p_line_spaced, "ENDSEC;").map(|(v, _): (BTreeMap<Id, DataValue>, _)| v);
    let p_data_section = p_space_or_comment_surrounded(preceded("DATA;", p_lines));
    p_data_section.map(DataMap)
}

#[test]
fn parse_index_map_works() {
    let data = r#"
DATA;
#1= IFCORGANIZATION($,'Autodesk Revit 2018 (DEU)',$,$,$);
#5= IFCAPPLICATION(#1,'2018','Autodesk Revit 2018 (DEU)','Revit');
#6= IFCCARTESIANPOINT((0.,0.,0.));
#9= IFCCARTESIANPOINT((0.,0.));
#11= IFCDIRECTION((1.,0.,0.));
#13= IFCDIRECTION((-1.,0.,0.));
#15= IFCDIRECTION((0.,1.,0.));
#17= IFCDIRECTION((0.,-1.,0.));
#19= IFCDIRECTION((0.,0.,1.));
#21= IFCDIRECTION((0.,0.,-1.));
#23= IFCDIRECTION((1.,0.));
#25= IFCDIRECTION((-1.,0.));
#27= IFCDIRECTION((0.,1.));
#29= IFCDIRECTION((0.,-1.));
#31= IFCAXIS2PLACEMENT3D(#6,$,$);
#32= IFCLOCALPLACEMENT(#2922282,#31);
#35= IFCTELECOMADDRESS($,$,$,$,$,$,('23022.debeka@rkwmail.de'),$);
#39= IFCPERSON($,'','hannah.schmitz',$,$,$,$,(#35));
#43= IFCORGANIZATION($,'','',$,$);
#44= IFCPERSONANDORGANIZATION(#39,#43,$);
#47= IFCOWNERHISTORY(#44,#5,$,.NOCHANGE.,$,$,$,0);
#48= IFCSIUNIT(*,.LENGTHUNIT.,$,.METRE.);
#49= IFCSIUNIT(*,.AREAUNIT.,$,.SQUARE_METRE.);
#50= IFCSIUNIT(*,.VOLUMEUNIT.,$,.CUBIC_METRE.);
#51= IFCSIUNIT(*,.PLANEANGLEUNIT.,$,.RADIAN.);
#52= IFCDIMENSIONALEXPONENTS(0,0,0,0,0,0,0);
#53= IFCMEASUREWITHUNIT(IFCRATIOMEASURE(0.0174532925199433),#51);
#54= IFCCONVERSIONBASEDUNIT(#52,.PLANEANGLEUNIT.,'DEGREE',#53);
#55= IFCSIUNIT(*,.MASSUNIT.,.KILO.,.GRAM.);
#56= IFCDERIVEDUNITELEMENT(#55,1);
#57= IFCDERIVEDUNITELEMENT(#48,-3);
#58= IFCDERIVEDUNIT((#56,#57),.MASSDENSITYUNIT.,$);
#60= IFCSIUNIT(*,.TIMEUNIT.,$,.SECOND.);
#61= IFCSIUNIT(*,.FREQUENCYUNIT.,$,.HERTZ.);
#62= IFCSIUNIT(*,.THERMODYNAMICTEMPERATUREUNIT.,$,.KELVIN.);
#63= IFCSIUNIT(*,.THERMODYNAMICTEMPERATUREUNIT.,$,.DEGREE_CELSIUS.);
#64= IFCDERIVEDUNITELEMENT(#55,1);
#65= IFCDERIVEDUNITELEMENT(#62,-1);
#66= IFCDERIVEDUNITELEMENT(#60,-3);
#67= IFCDERIVEDUNIT((#64,#65,#66),.THERMALTRANSMITTANCEUNIT.,$);
#69= IFCSIUNIT(*,.LENGTHUNIT.,.DECI.,.METRE.);
#70= IFCDERIVEDUNITELEMENT(#48,3);
#71= IFCDERIVEDUNITELEMENT(#60,-1);
#72= IFCDERIVEDUNIT((#70,#71),.VOLUMETRICFLOWRATEUNIT.,$);
#74= IFCSIUNIT(*,.ELECTRICCURRENTUNIT.,$,.AMPERE.);
#75= IFCSIUNIT(*,.ELECTRICVOLTAGEUNIT.,$,.VOLT.);
#76= IFCSIUNIT(*,.POWERUNIT.,$,.WATT.);
#77= IFCSIUNIT(*,.FORCEUNIT.,.KILO.,.NEWTON.);
#78= IFCSIUNIT(*,.ILLUMINANCEUNIT.,$,.LUX.);
#79= IFCSIUNIT(*,.LUMINOUSFLUXUNIT.,$,.LUMEN.);
#80= IFCSIUNIT(*,.LUMINOUSINTENSITYUNIT.,$,.CANDELA.);
#81= IFCDERIVEDUNITELEMENT(#55,-1);
#82= IFCDERIVEDUNITELEMENT(#48,-2);
#83= IFCDERIVEDUNITELEMENT(#60,3);
#84= IFCDERIVEDUNITELEMENT(#79,1);
#85= IFCDERIVEDUNIT((#81,#82,#83,#84),.USERDEFINED.,'Luminous Efficacy');
#87= IFCDERIVEDUNITELEMENT(#48,1);
#88= IFCDERIVEDUNITELEMENT(#60,-1);
#89= IFCDERIVEDUNIT((#87,#88),.LINEARVELOCITYUNIT.,$);
#91= IFCSIUNIT(*,.PRESSUREUNIT.,$,.PASCAL.);
#92= IFCDERIVEDUNITELEMENT(#48,-2);
#93= IFCDERIVEDUNITELEMENT(#55,1);
#94= IFCDERIVEDUNITELEMENT(#60,-2);
#95= IFCDERIVEDUNIT((#92,#93,#94),.USERDEFINED.,'Friction Loss');
#97= IFCUNITASSIGNMENT((#48,#49,#50,#54,#55,#58,#60,#61,#63,#67,#72,#74,#75,#76,#77,#78,#79,#80,#85,#89,#91,#95));
#99= IFCAXIS2PLACEMENT3D(#6,$,$);
#100= IFCDIRECTION((6.12303176911189E-17,1.));
#102= IFCGEOMETRICREPRESENTATIONCONTEXT($,'Model',3,1.00000000000000E-5,#99,#100);
#105= IFCGEOMETRICREPRESENTATIONSUBCONTEXT('Axis','Model',*,*,*,*,#102,$,.GRAPH_VIEW.,$);
#107= IFCGEOMETRICREPRESENTATIONSUBCONTEXT('Body','Model',*,*,*,*,#102,$,.MODEL_VIEW.,$);
#108= IFCGEOMETRICREPRESENTATIONSUBCONTEXT('Box','Model',*,*,*,*,#102,$,.MODEL_VIEW.,$);
#109= IFCGEOMETRICREPRESENTATIONSUBCONTEXT('FootPrint','Model',*,*,*,*,#102,$,.MODEL_VIEW.,$);
#110= IFCPROJECT('0UQ2T3XlP1QPjq2tNG9N8h',#47,'23022',$,$,'23022 Debeka HV-Erweiterung','',(#102),#97);
ENDSEC;
"#;
    let map = p_index_map().parse(data).unwrap();

    println!("{map}");

    assert_eq!(format!("{map}").trim(), data.trim());
}
