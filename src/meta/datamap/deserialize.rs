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
    parser::{optional::IFCParse, p_space_or_comment_surrounded, IFCParser},
};

impl IFCParse for DataMap {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        let p_obj = repeat_till(.., any, newline).map(|(s, _): (String, _)| DataValue::Any { s });
        let p_line = separated_pair(Id::parse(), p_space_or_comment_surrounded("="), p_obj);
        let p_line_spaced = p_space_or_comment_surrounded(p_line);
        let p_lines =
            repeat_till(.., p_line_spaced, "ENDSEC;").map(|(v, _): (BTreeMap<Id, DataValue>, _)| v);
        let p_data_section = p_space_or_comment_surrounded(preceded("DATA;", p_lines));
        p_data_section.map(DataMap)
    }
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
    let map = DataMap::parse().parse(data).unwrap();

    println!("{map}");

    assert_eq!(format!("{map}").trim(), data.trim());
}

#[test]
fn parse_from_example_file() {
    let data = r#"DATA;
#1= IFCBUILDING('39t4Pu3nTC4ekXYRIHJB9W',#2,'IfcBuilding',$,$,$,$,$,$,$,$,$);
#2= IFCOWNERHISTORY(#5,#6,$,.ADDED.,1454575675,$,$,1454575675);
#5= IFCPERSONANDORGANIZATION(#7,#8,$);
#6= IFCAPPLICATION(#9,'0.0.1.0','ggRhinoIFC - Geometry Gym Plug-in for Rhino3d','ggRhinoIFC');
#7= IFCPERSON('Jon','Jon',$,$,$,$,$,$);
#8= IFCORGANIZATION($,'Geometry Gym Pty Ltd',$,$,$);
#9= IFCORGANIZATION($,'Geometry Gym Pty Ltd',$,$,$);
#3= IFCRELAGGREGATES('091a6ewbvCMQ2Vyiqspa7a',#2,'Project Container','Project Container for Buildings',#10,(#1));
#4= IFCRELCONTAINEDINSPATIALSTRUCTURE('3Sa3dTJGn0H8TQIGiuGQd5',#2,'Building','Building Container for Elements',(#11),#1);
#10= IFCPROJECT('0$WU4A9R19$vKWO$AdOnKA',#2,'IfcProject',$,$,$,$,(#12),#13);
#12= IFCGEOMETRICREPRESENTATIONCONTEXT($,'Model',3,0.0001,#15,$);
#13= IFCUNITASSIGNMENT((#18,#19,#20));
#15= IFCAXIS2PLACEMENT3D(#21,$,$);
#16= IFCGEOMETRICREPRESENTATIONSUBCONTEXT('Axis','Model',0,$,$,$,#12,$,.MODEL_VIEW.,$);
#17= IFCGEOMETRICREPRESENTATIONSUBCONTEXT('Body','Model',0,$,$,$,#12,$,.MODEL_VIEW.,$);
#18= IFCSIUNIT($,.LENGTHUNIT.,.MILLI.,.METRE.);
#19= IFCSIUNIT($,.PLANEANGLEUNIT.,$,.RADIAN.);
#20= IFCSIUNIT($,.TIMEUNIT.,$,.SECOND.);
#21= IFCCARTESIANPOINT((0.,0.,0.));
#22= IFCSHAPEREPRESENTATION(#17,'Axis','Curve2D',(#24));
#23= IFCSHAPEREPRESENTATION(#17,'Body','SweptSolid',(#26));
#24= IFCPOLYLINE((#27,#28));
#25= IFCPRODUCTDEFINITIONSHAPE($,$,(#22,#23));
#26= IFCEXTRUDEDAREASOLID(#29,$,#30,2000.);
#27= IFCCARTESIANPOINT((0.,0.));
#28= IFCCARTESIANPOINT((5000.,0.));
#29= IFCRECTANGLEPROFILEDEF(.AREA.,'Wall Perim',#31,5000.,270.);
#30= IFCDIRECTION((0.,0.,1.));
#31= IFCAXIS2PLACEMENT2D(#32,$);
#32= IFCCARTESIANPOINT((2500.,135.));
#11= IFCWALL('0DWgwt6o1FOx7466fPk$jl',#2,$,$,$,#33,#25,$,$);
#33= IFCLOCALPLACEMENT($,#36);
#36= IFCAXIS2PLACEMENT3D(#21,$,$);
#14= IFCRELDECLARES('1lEof85zvB$O57GEVffll1',#2,$,$,#10,(#37));
#34= IFCRELASSOCIATESMATERIAL('1BYoVhjtLADPUZYzipA826',#2,'MatAssoc','Material Associates',(#11),#38);
#38= IFCMATERIALLAYERSETUSAGE(#39,.AXIS2.,.POSITIVE.,0.,$);
#39= IFCMATERIALLAYERSET((#40,#41,#42),'Double Brick - 270',$);
#40= IFCMATERIALLAYER(#44,110.,.F.,'Finish',$,$,$);
#41= IFCMATERIALLAYER($,50.,.T.,'Air Infiltration Barrier',$,$,$);
#42= IFCMATERIALLAYER(#45,110.,.F.,'Core',$,$,$);
#44= IFCMATERIAL('Masonry - Brick - Brown',$,$);
#45= IFCMATERIAL('Masonry',$,$);
#35= IFCRELDEFINESBYTYPE('1$EkFElNT8TB_VUVG1FtMe',#2,$,$,(#11),#37);
#37= IFCWALLTYPE('2aG1gZj7PD2PztLOx2$IVX',#2,'Double Brick - 270',$,$,$,$,$,$,.NOTDEFINED.);
#43= IFCRELASSOCIATESMATERIAL('36U74BIPDD89cYkx9bkV$Y',#2,'MatAssoc','Material Associates',(#37),#39);
ENDSEC;"#;

    let map = DataMap::parse().parse(data).unwrap();
    let str_map = map.to_string();

    assert_eq!(data, str_map);
}
