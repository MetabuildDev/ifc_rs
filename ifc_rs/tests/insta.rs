#[test]
fn archicad_results() -> anyhow::Result<()> {
    let ifc = ifc_rs::IFC::from_file("resources/AC20-FZK-Haus.ifc")?;

    let output = ifc.to_string();

    let uuid_re = regex::Regex::new(
        r"([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12})",
    )?;

    let output = uuid_re.replace_all(&output, "[redacted uuid]");

    insta::assert_snapshot!(output);

    Ok(())
}
