use crate::geometry::arbitrary_closed_profile_def::ArbitraryClosedProfileDef;
use crate::geometry::indexed_poly_curve::Curve;
use crate::geometry::profile_type::ProfileType;
use crate::id::TypedId;
use crate::parser::comma::Comma;
use crate::parser::optional::OptionalParameter;
use crate::parser::*;

impl<C: Curve> IFCParse for ArbitraryClosedProfileDef<C> {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCARBITRARYCLOSEDPROFILEDEF("),

                profile_type: ProfileType::parse(),
                _: Comma::parse(),
                profile_name: OptionalParameter::parse(),
                _: Comma::parse(),
                outer_curve: TypedId::<C>::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}
