pub mod building;
pub mod materials;
pub mod openings;
pub mod prelude;
pub mod primary;
pub mod roofs;
pub mod slabs;
pub mod storey;
pub mod transforms;
pub mod walls;
pub mod windows;

use crate::prelude::*;

pub struct ApplicationInfo<'a> {
    pub developer: Person,
    pub version: &'a str,
    pub name: &'a str,
    pub short_name: &'a str,
}

pub struct OwnerInfo<'a> {
    pub owner: Person,
    pub organization_name: &'a str,
}

#[cfg(test)]
pub(crate) mod test {
    use crate::prelude::*;

    pub fn create_builder() -> IfcBuilder {
        IfcBuilder::new(
            ApplicationInfo {
                developer: Person::empty().given_name("Mario"),
                version: "0.0.1",
                name: "IfcBuilderApplication",
                short_name: "builder",
            },
            OwnerInfo {
                owner: Person::empty().given_name("Luigi"),
                organization_name: "Metabuild",
            },
            Person::empty().given_name("Bowser"),
            "IfcBuider Example Project",
        )
    }
}
