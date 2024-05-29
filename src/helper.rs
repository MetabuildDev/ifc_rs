pub fn format_double(d: f64) -> String {
    // might need tuning 16 decimals allowed
    // +2 for "0." in string
    let is_scientific = d.fract().to_string().len() > 16 + 2;

    if is_scientific {
        format!("{0:.1$E}", d, 14)
    } else {
        format!(
            "{d}{opt_p}",
            opt_p = (d.fract() == 0.0).then_some(".").unwrap_or_default()
        )
    }
}
