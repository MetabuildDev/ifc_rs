use ifc::meta::footer::Footer;

use crate::common::example_version;

fn example_footer() -> Footer {
    Footer {
        version: example_version(),
    }
}

#[test]
fn serde_roundtrips_backwards() {
    let footer = example_footer();
    let footer_str = serde_json::to_string(&footer).unwrap();
    let footer_again: Footer = serde_json::from_str(&footer_str).unwrap();
    assert_eq!(footer, footer_again);
}
