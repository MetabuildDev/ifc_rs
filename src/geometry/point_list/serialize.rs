use std::fmt::Display;

use crate::geometry::point_list::{PointList2D, PointList3D};

impl Display for PointList2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCCARTESIANPOINTLIST2D({coord_list},{tag_list});",
            coord_list = self.coord_list,
            tag_list = self.tag_list,
        )
    }
}

impl Display for PointList3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCCARTESIANPOINTLIST3D({coord_list},{tag_list});",
            coord_list = self.coord_list,
            tag_list = self.tag_list,
        )
    }
}
