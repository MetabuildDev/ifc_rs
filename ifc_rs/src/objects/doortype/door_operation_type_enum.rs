use std::str::FromStr;

use strum::{Display, EnumString, VariantNames};
use winnow::combinator::{alt, delimited};
use winnow::Parser;

use crate::parser::*;

/// This enumeration defines the basic ways to describe how doors operate, as
/// shown in Figure 189. It combines the partitioning of the door into a single
/// or multiple door panels and the operation types of that panels.
///
/// In the most common case of swinging doors the IfcDoorTypeOperationEnum
/// defined the hinge side (left hing or right hung) and the opening direction
/// (opening to the left, opening to the right). Whether the door opens inwards
/// or outwards is determined by the local coordinate system of the IfcDoor, or
/// IfcDoorStandardCase.
///
/// https://standards.buildingsmart.org/IFC/RELEASE/IFC4/ADD2_TC1/HTML/schema/ifcsharedbldgelements/lexical/ifcdoortypeoperationenum.htm
#[derive(EnumString, VariantNames, Display, Clone, Copy)]
pub enum DoorOperationTypeEnum {
    /// Door with one panel that opens (swings) to the left. The hinges are on
    /// the left side as viewed in the direction of the positive y-axis.
    #[strum(to_string = ".SINGLE_SWING_LEFT.")]
    SingleSwingLeft,

    /// Door with one panel that opens (swings) to the right. The hinges are on
    ///the right side as viewed in the direction of the positive y-axis.
    #[strum(to_string = ".SINGLE_SWING_RIGHT.")]
    SingleSwingRight,

    /// Door with two panels, one opens (swings) to the left the other opens
    /// (swings) to the right.
    #[strum(to_string = ".DOUBLE_DOOR_SINGLE_SWING.")]
    DoubleDoorSingleSwing,

    /// Door with one panel that swings in both directions and to the left in
    /// the main trafic direction. Also called double acting door.
    #[strum(to_string = ".DOUBLE_SWING_LEFT.")]
    DoubleSwingLeft,

    /// Door with one panel that swings in both directions and to the right in
    /// the main trafic direction. Also called double acting door.
    #[strum(to_string = ".DOUBLE_SWING_RIGHT.")]
    DoubleSwingRight,

    /// Door with two panels, one swings in both directions and to the right in
    /// the main trafic direction the other swings also in both directions and
    /// to the left in the main trafic direction.
    #[strum(to_string = ".DOUBLE_DOOR_DOUBLE_SWING.")]
    DoubleDoorDoubleSwing,

    /// Door with two panels that both open to the left, one panel swings in
    /// one direction and the other panel swings in the opposite direction.
    #[strum(to_string = ".DOUBLE_DOOR_SINGLE_SWING_OPPOSITE_LEFT.")]
    DoubleDoorSingleSwingOppositeLeft,

    /// Door with two panels that both open to the right, one panel swings in
    /// one direction and the other panel swings in the opposite direction.
    #[strum(to_string = ".DOUBLE_DOOR_SINGLE_SWING_OPPOSITE_RIGHT.")]
    DoubleDoorSingleSwingOppositeRight,

    /// Door with one panel that is sliding to the left.
    #[strum(to_string = ".SLIDING_TO_LEFT.")]
    SlidingToLeft,

    /// Door with one panel that is sliding to the right.
    #[strum(to_string = ".SLIDING_TO_RIGHT.")]
    SlidingToRight,

    /// Door with two panels, one is sliding to the left the other is sliding
    /// to the right.
    #[strum(to_string = ".DOUBLE_DOOR_SLIDING.")]
    DoubleDoorSliding,

    /// Door with one panel that is folding to the left.
    #[strum(to_string = ".FOLDING_TO_LEFT.")]
    FoldingToLeft,

    /// Door with one panel that is folding to the right.
    #[strum(to_string = ".FOLDING_TO_RIGHT.")]
    FoldingToRight,

    /// Door with two panels, one is folding to the left the other is folding to the right.
    #[strum(to_string = ".DOUBLE_DOOR_FOLDING.")]
    DoubleDoorFolding,

    /// An entrance door consisting of four leaves set in a form of a cross and
    /// revolving around a central vertical axis (the four panels are described
    /// by a single IfcDoor panel property).
    #[strum(to_string = ".REVOLVING.")]
    Revolving,

    /// Door that opens by rolling up.
    #[strum(to_string = ".ROLLINGUP.")]
    RollingUp,

    /// Door with one panel that opens (swings) to the left and one fixed panel.
    /// The hinges of the swinging panel are on the left side as viewed in the
    /// direction of the positive y-axis.
    #[strum(to_string = ".SWING_FIXED_LEFT.")]
    SwingFixedLeft,

    /// Door with one panel that opens (swings) to the right and one fixed panel.
    /// The hinges of the swinging panel are on the right side as viewed in the
    /// direction of the positive y-axis.
    #[strum(to_string = ".SWING_FIXED_RIGHT.")]
    SwingFixedRight,

    /// User-defined operation type
    #[strum(to_string = ".USERDEFINED.")]
    UserDefined,

    /// Undefined operation type
    #[strum(to_string = ".NOTDEFINED.")]
    NotDefined,
}

impl IFCParse for DoorOperationTypeEnum {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        let variants: [&str; Self::VARIANTS.len()] =
            Self::VARIANTS.try_into().expect("statically known");

        delimited(
            p_space_or_comment(),
            alt(variants
                .map(|v| (v, Self::from_str(v).expect("valid DoorOperationTypeEnum")))
                .map(|(k, v)| k.map(move |_| v))),
            p_space_or_comment(),
        )
    }
}
