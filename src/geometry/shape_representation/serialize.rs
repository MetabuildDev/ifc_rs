use std::fmt::Display;

use itertools::Itertools;

use super::ShapeRepresentation;

impl Display for ShapeRepresentation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCSHAPEREPRESENTATION({id},{name},{_type},({items}));",
            id = self.context_of_items,
            name = self.representation_identifier,
            _type = self.representation_type,
            items = self.items.iter().map(|id| format!("{id}")).join(",")
        )
    }
}
