use std::fmt::Display;

use super::DataMap;

impl Display for DataMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "DATA;")?;

        for (id_str, obj_str) in self.0.iter() {
            writeln!(f, "{id}= {obj}", id = id_str, obj = obj_str.to_string())?;
        }

        writeln!(f, "ENDSEC;")?;
        Ok(())
    }
}
