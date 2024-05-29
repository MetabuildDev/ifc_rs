use glam::{DVec2, DVec3};
use winnow::{ascii::float, Parser};

use super::IFCParser;
use crate::parser::p_space_or_comment_surrounded;

pub fn p_vec2<'a>() -> impl IFCParser<'a, DVec2> {
    winnow::seq! {
        (
            _: p_space_or_comment_surrounded("("),
            float,
            _: p_space_or_comment_surrounded(","),
            float,
            _: p_space_or_comment_surrounded(")")
        )
    }
    .map(DVec2::from)
}

pub fn p_vec3<'a>() -> impl IFCParser<'a, DVec3> {
    winnow::seq! {
        (
            _: p_space_or_comment_surrounded("("),
            float,
            _: p_space_or_comment_surrounded(","),
            float,
            _: p_space_or_comment_surrounded(","),
            float,
            _: p_space_or_comment_surrounded(")")
        )
    }
    .map(DVec3::from)
}
