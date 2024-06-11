use std::fmt::Display;

use itertools::Itertools;

use super::{
    description::{FileDescription, ViewDefinition},
    details::FileDetails,
    schema::FileSchemas,
    Header,
};

impl Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{version};", version = self.version)?;
        writeln!(f, "HEADER;")?;
        writeln!(f, "FILE_DESCRIPTION({desc});", desc = self.description)?;
        writeln!(f, "FILE_NAME({name});", name = self.name)?;
        writeln!(f, "FILE_SCHEMA({schema});", schema = self.schema)?;
        writeln!(f, "ENDSEC;")?;
        Ok(())
    }
}

impl Display for FileDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({descs})",
            descs = if self.descriptions.is_empty() {
                "''".to_string()
            } else {
                self.descriptions
                    .iter()
                    .map(|desc| format!("'{desc}'"))
                    .join(",")
            }
        )?;
        write!(f, ",")?;
        write!(f, "'{level}'", level = self.implementation_level)?;
        Ok(())
    }
}

impl Display for ViewDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{name} [{items}]",
            name = self.name,
            items = self.items.join(", ")
        )
    }
}

impl Display for FileDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{name}'", name = self.name.0)?;
        write!(f, ",")?;
        write!(
            f,
            "'{time}'",
            time = self.timestamp.0.format("%Y-%m-%dT%H:%M:%S")
        )?;
        write!(f, ",")?;
        write!(
            f,
            "({authors})",
            authors = self
                .author
                .iter()
                .map(|author| format!("'{a}'", a = author.0))
                .join(",")
        )?;
        write!(f, ",")?;
        write!(
            f,
            "({orgs})",
            orgs = self
                .organization
                .iter()
                .map(|org| format!("'{o}'", o = org.0))
                .join(",")
        )?;
        write!(f, ",")?;
        write!(f, "'{prep_v}'", prep_v = self.preprocessor_version.0)?;
        write!(f, ",")?;
        write!(f, "'{sys}'", sys = self.originating_system.0)?;
        write!(f, ",")?;
        write!(f, "'{auth}'", auth = self.authorization.0)?;
        Ok(())
    }
}

impl Display for FileSchemas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({schemas})",
            schemas = self.0.iter().map(|schema| format!("'{schema}'")).join(",")
        )
    }
}
