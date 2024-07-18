use alloy::hex::ToHexExt;
use alloy::signers::local::{MnemonicBuilder, PrivateKeySigner};
use alloy::signers::local::coins_bip39::{English, Mnemonic};
use eyre::Result;
use rand::thread_rng;

fn main() -> Result<()> {
    let mut rng = thread_rng();

    // Generate a new mnemonic
    let mnemonic = Mnemonic::<English>::new_with_count(&mut rng, 24)?;
    println!("{}", mnemonic.to_phrase());
    println!("{}", mnemonic.to_seed(Some("my-passphrase")).unwrap().encode_hex());
    dbg!(mnemonic.to_phrase());
    // allow evidence glass broken mask again ship tower head asthma walnut siren burst express mobile pioneer leisure pen absent input receive need help maze
    // 1bc26ec91cdec72f56c0b717d2d9dd76a8c32911b6035ff3107e6bf13fcc073f718a5b1bc89973f9d7bd53277b5877e77767788dfc44e0a8c6c1adc1a968134a

    // Get an account using a mnemonic, index and passphrase.
    let signer = MnemonicBuilder::<English>::default()
        .phrase("crop index worth fashion midnight toss message ignore near alter only sand symbol chuckle soldier problem park major cheap dismiss become cage aim health")
        .password("my-passphrase")
        .index(2)      .unwrap()        .build()?;
    dbg!(signer);
    // address=0xb3b06e771d96c1a99e22f9f587c421d2ed603e61

    // Get an account using a mnemonic, derivation path and passphrase.
    let signer = MnemonicBuilder::<English>::default()
        .phrase("crop index worth fashion midnight toss message ignore near alter only sand symbol chuckle soldier problem park major cheap dismiss become cage aim health")
        .password("my-passphrase")
        .derivation_path("m/44'/60'/0'/0/2").unwrap().build()?;
    dbg!(signer);
    // address=0xb3b06e771d96c1a99e22f9f587c421d2ed603e61

    // Generate a new key
    let signer = PrivateKeySigner::random();
    let address_str = signer.address().to_string();
    let private_key_str = signer.to_bytes().to_string();
    println!("address: {address_str}, private_key: {private_key_str}");
    // address: 0x6634F22aa2b3aC17588f4bE99c9bce022DE2b047, private_key: 0x3ede14f2ed0e276c6adec08811ce65f19b40ebc08645cb7469da829844f64998

    Ok(())
}
