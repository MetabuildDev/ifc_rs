use std::fmt::Display;

use crate::objects::application::Application;

impl Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCAPPLICATION({application_developer},{version},{application_full_name},{application_identifier});",
            application_developer = self.application_developer,
            version = self.version,
            application_full_name = self.application_full_name,
            application_identifier = self.application_identifier,
        )
    }
}
