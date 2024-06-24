use std::fmt::Display;

use crate::geometry::representation_context::GeometricRepresentationContext;

impl Display for GeometricRepresentationContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCGEOMETRICREPRESENTATIONCONTEXT({context_id},{context_type},{coord_dims},{precision},{world_coord_system},{true_north});",
            context_id = self.context_identifier,
            context_type = self.context_type,
            coord_dims = self.coord_space_dimension,
            precision = self.precision,
            world_coord_system = self.world_coord_system,
            true_north = self.true_north,
        )
    }
}
