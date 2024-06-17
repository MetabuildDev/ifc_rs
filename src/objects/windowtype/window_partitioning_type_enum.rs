use std::str::FromStr;

use strum::{Display, EnumString, VariantNames};
use winnow::combinator::{alt, delimited};
use winnow::Parser;

use crate::parser::*;

/// This enumeration defines the basic configuration of the window type in
/// terms of the number of window panels and the subdivision of the total
/// window as shown in Figure 198. The window configurations are given for
/// windows with one, two or three panels (including fixed panels).
///
/// Windows which are subdivided into more than three panels have to be defined
/// by the geometry only. The type of such windows is USERDEFINED.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcsharedbldgelements/lexical/ifcwindowtypepartitioningenum.htm
#[derive(EnumString, VariantNames, Display, Clone, Copy)]
pub enum WindowPartitioningTypeEnum {
    /// Window with one panel.
    #[strum(to_string = ".SINGLE_PANEL.")]
    SinglePanel,

    /// Window with two panels. The configuration of the panels is vertically.
    #[strum(to_string = ".DOUBLE_PANEL_VERTICAL.")]
    DoublePanelVertical,

    /// Window with two panels. The configuration of the panels is horizontally.
    #[strum(to_string = ".DOUBLE_PANEL_HORIZONTAL.")]
    DoublePanelHorizontal,

    /// Window with three panels. The configuration of the panels is vertically.
    #[strum(to_string = ".TRIPLE_PANEL_VERTICAL.")]
    TriplePanelVertical,

    /// Window with three panels. The configuration of the panels is horizontally.
    #[strum(to_string = ".TRIPLE_PANEL_HORIZONTAL.")]
    TriplePanelHorizontal,

    /// Window with three panels. The configuration of two panels is vertically
    /// and the third one is horizontally at the bottom.
    #[strum(to_string = ".TRIPLE_PANEL_BOTTOM.")]
    TriplePanelBottom,

    /// Window with three panels. The configuration of two panels is vertically
    /// and the third one is horizontally at the top.
    #[strum(to_string = ".TRIPLE_PANEL_TOP.")]
    TriplePanelTop,

    /// Window with three panels. The configuration of two panels is horizontally
    /// and the third one is vertically at the left hand side.
    #[strum(to_string = ".TRIPLE_PANEL_LEFT.")]
    TriplePanelLeft,

    /// Window with three panels. The configuration of two panels is horizontally
    /// and the third one is vertically at the right hand side.
    #[strum(to_string = ".TRIPLE_PANEL_RIGHT.")]
    TriplePanelRight,

    /// User-defined parititoning
    #[strum(to_string = ".USERDEFINED.")]
    UserDefined,

    /// Undefined parititoning
    #[strum(to_string = ".NOTDEFINED.")]
    NotDefined,
}

impl IFCParse for WindowPartitioningTypeEnum {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        let variants: [&str; Self::VARIANTS.len()] =
            Self::VARIANTS.try_into().expect("statically known");

        delimited(
            p_space_or_comment(),
            alt(variants
                .map(|v| {
                    (
                        v,
                        Self::from_str(v).expect("valid WindowPartitioningTypeEnum"),
                    )
                })
                .map(|(k, v)| k.map(move |_| v))),
            p_space_or_comment(),
        )
    }
}
