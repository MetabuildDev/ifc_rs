use std::fmt::Display;

use crate::geometry::indexed_poly_curve::IndexedPolyCurve;

impl Display for IndexedPolyCurve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCINDEXEDPOLYCURVE({points},{segments},{self_intersect});",
            points = self.points,
            segments = self.segments,
            self_intersect = self.self_intersect,
        )
    }
}
