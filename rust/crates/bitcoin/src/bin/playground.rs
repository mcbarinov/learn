use bitcoin::secp256k1::Secp256k1;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let (secret_key, public_key) = Secp256k1::new().generate_keypair(&mut rand::thread_rng());
    dbg!(secret_key.display_secret());
    dbg!(public_key);
    Ok(())
}
