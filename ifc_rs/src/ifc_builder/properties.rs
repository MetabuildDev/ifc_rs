use std::marker::PhantomData;

use crate::prelude::*;

/// Builder to prepare an arbitrary [`PropertySet`] which is automatically assigned to the wall
/// which is currently build via a [`Drop`] implementation. This builder is created with
/// [`IfcWallBuilder::add_properties`]
pub struct IfcWallPropertiesBuilder<'a, ObjectT, BuilderT>
where
    ObjectT: IfcType,
    BuilderT: IfcObjectBuilder<ObjectT>,
{
    pub(crate) object: &'a mut BuilderT,
    pub(crate) name: String,
    pub(crate) properties: Vec<TypedId<PropertySingleValue>>,

    pd: PhantomData<ObjectT>,
}

impl<ObjectT, BuilderT> Drop for IfcWallPropertiesBuilder<'_, ObjectT, BuilderT>
where
    ObjectT: IfcType,
    BuilderT: IfcObjectBuilder<ObjectT>,
{
    fn drop(&mut self) {
        // here we create the set AND link it to the wall
        let set = PropertySet::new(self.name.as_str(), self.properties.clone());
        let set_id = self.object.get_ifc().data.insert_new(set);

        let rel = RelDefinesByProperties::new("PropertiesLink", set_id, self.object.get_ifc())
            .relate_obj(self.object.get_id(), self.object.get_ifc());
        self.object.get_ifc().data.insert_new(rel);
    }
}

impl<'a, ObjectT, BuilderT> IfcWallPropertiesBuilder<'a, ObjectT, BuilderT>
where
    ObjectT: IfcType,
    BuilderT: IfcObjectBuilder<ObjectT>,
{
    /// Add a [`PropertySingleValue`] to the [`PropertySet`] of a wall
    #[must_use]
    pub fn single_property(
        &mut self,
        name: &str,
        value: IfcValue,
        unit: Option<Id>,
    ) -> TypedId<PropertySingleValue> {
        let prop = PropertySingleValue::new(name, value, unit);
        let id = self.object.get_ifc().data.insert_new(prop);
        self.properties.push(id);
        id
    }
}

impl IfcWallBuilder<'_, '_> {
    /// Add a [`PropertySet`] to the [`Wall`] which is currently build.
    #[must_use]
    pub fn add_properties(&mut self, name: &str) -> IfcWallPropertiesBuilder<Wall, Self> {
        IfcWallPropertiesBuilder {
            object: self,
            name: name.to_string(),
            properties: Default::default(),

            pd: PhantomData,
        }
    }
}

/// Builder to prepare an arbitrary [`PropertySet`] which can later be assigned with
/// [`IfcStoreyBuilder::relate_properties_object`]. This builder is created with
/// [`IfcStoreyBuilder::add_properties`]
pub struct IfcPropertySetBuilder<'a> {
    pub(crate) ifc: &'a mut IFC,
    pub(crate) name: String,
    pub(crate) properties: Vec<TypedId<PropertySingleValue>>,
}

impl IfcStoreyBuilder<'_> {
    /// creates an [`IfcPropertySetBuilder`] to prepare an arbitrary [`PropertySet`]
    #[must_use]
    pub fn add_properties(&mut self, name: &str) -> IfcPropertySetBuilder {
        IfcPropertySetBuilder {
            ifc: &mut self.project.ifc,
            name: name.to_string(),
            properties: Default::default(),
        }
    }

    /// relates an arbitrary [`PropertySet`] with an arbitrary IFC object (check possibilities via
    /// implementors of [`IfcType`])
    pub fn relate_properties_object<OBJ: IfcType>(
        &mut self,
        property_set_id: TypedId<PropertySet>,
        object_id: TypedId<OBJ>,
    ) {
        let rel =
            RelDefinesByProperties::new("PropertiesLink", property_set_id, &mut self.project.ifc)
                .relate_obj(object_id, &mut self.project.ifc);
        self.project.ifc.data.insert_new(rel);
    }
}

impl<'a> IfcPropertySetBuilder<'a> {
    /// Add a [`PropertySingleValue`] to this [`PropertySet`]
    #[must_use]
    pub fn single_property(
        &mut self,
        name: &str,
        value: IfcValue,
        unit: Option<Id>,
    ) -> TypedId<PropertySingleValue> {
        let prop = PropertySingleValue::new(name, value, unit);
        let id = self.ifc.data.insert_new(prop);
        self.properties.push(id);
        id
    }

    /// Finish and build the [`PropertySet`]
    #[must_use]
    pub fn finish(self) -> TypedId<PropertySet> {
        let set = PropertySet::new(self.name.as_str(), self.properties.clone());
        self.ifc.data.insert_new(set)
    }
}
