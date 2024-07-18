use alloy::primitives::utils::format_units;

fn main() -> eyre::Result<()> {
    let gwei = format_units(10_000_000, "gwei")?.parse::<f64>()?;
    dbg!(gwei); //  0.01

    Ok(())
}
