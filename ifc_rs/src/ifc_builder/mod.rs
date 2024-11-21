pub(crate) mod building;
pub(crate) mod materials;
pub(crate) mod openings;
pub(crate) mod prelude;
pub(crate) mod project;
pub(crate) mod properties;
pub(crate) mod roofs;
pub(crate) mod shading_devices;
pub(crate) mod site;
pub(crate) mod slabs;
pub(crate) mod spaces;
pub(crate) mod storey;
pub(crate) mod transforms;
pub(crate) mod walls;
pub(crate) mod windows;

use crate::prelude::*;

pub trait IfcObjectBuilder<T: IfcType> {
    fn get_ifc(&mut self) -> &mut IFC;
    fn get_id(&self) -> TypedId<T>;
}

pub struct ApplicationInfo<'a> {
    pub developer: Organization,
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

    pub fn create_builder() -> IfcProjectBuilder {
        IfcProjectBuilder::new(
            ApplicationInfo {
                developer: Organization::new(None, "ExampleOrganization", None),
                version: "0.0.1",
                name: "IfcBuilderApplication",
                short_name: "builder",
            },
            OwnerInfo {
                owner: Person::empty().given_name("Luigi"),
                organization_name: "Metabuild",
            },
            "IfcBuider Example Project",
        )
    }
}
