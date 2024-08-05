use alloy::primitives::Address;
use alloy::primitives::utils::format_units;

fn main() -> eyre::Result<()> {
    let gwei = format_units(10_000_000, "gwei")?.parse::<f64>()?;
    dbg!(gwei); //  0.01

    let address_str = "0x5fBA26d87316fD201834F4ef4636BE937829e84d";
    let address:Address = address_str.parse()?;
    dbg!(address); // 0x5fba26d87316fd201834f4ef4636be937829e84d

    Ok(())
}
