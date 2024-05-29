use std::fmt::Display;

use super::Footer;

impl Display for Footer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "END-{version};", version = self.version)?;
        Ok(())
    }
}
