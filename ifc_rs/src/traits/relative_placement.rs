use crate::prelude::*;

pub trait RelativePlacement: IfcType {
    fn placement_id(&self) -> Option<Id>;
}

impl RelativePlacement for Site {
    fn placement_id(&self) -> Option<Id> {
        self.object_placement.custom().copied()
    }
}
impl RelativePlacement for Building {
    fn placement_id(&self) -> Option<Id> {
        self.object_placement.custom().copied()
    }
}
impl RelativePlacement for Storey {
    fn placement_id(&self) -> Option<Id> {
        self.object_placement.custom().copied()
    }
}
impl RelativePlacement for OpeningElement {
    fn placement_id(&self) -> Option<Id> {
        self.object_placement.custom().copied()
    }
}
impl RelativePlacement for Wall {
    fn placement_id(&self) -> Option<Id> {
        self.object_placement.custom().copied()
    }
}

impl IFC {
    pub fn get_placement_id<T: RelativePlacement>(&self, id: TypedId<T>) -> Option<Id> {
        self.data.get::<T>(id).placement_id()
    }
}

impl LocalPlacement {
    pub fn new_relative<A: AxisPlacement, T: RelativePlacement>(
        placement: impl Into<IdOr<A>>,
        relative: TypedId<T>,
        ifc: &mut IFC,
    ) -> Self {
        let relative = ifc.get_placement_id(relative).unwrap_or_else(|| {
            panic!(
                "Relative position of {} couldn't be found",
                std::any::type_name::<T>()
            )
        });
        Self::new(placement, ifc).relative_to(relative, ifc)
    }
}
