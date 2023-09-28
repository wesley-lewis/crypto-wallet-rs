use anyhow::Result;
mod eth_wallet;

fn main() -> Result<()> {
    let (secret_key, pub_key) = eth_wallet::generate_keypair();

    println!("Secret key: {}", &secret_key.to_string());
    println!("Public Key: {}", &pub_key.to_string());

    let pub_address = eth_wallet::public_key_address(&pub_key);
    println!("Address: {}", pub_address.to_string());

    let crypto_wallet = eth_wallet::Wallet::new(&secret_key, &pub_key);
    println!("Crypto wallet: {:?}", &crypto_wallet);

    crypto_wallet.save_to_file("crypto_wallet.json")?;

    let wallet_file_path = "crypto_wallet.json";
    crypto_wallet.save_to_file(wallet_file_path)?;

    let loaded_wallet = eth_wallet::Wallet::from_file(wallet_file_path)?;
    println!("Loaded wallet: {:?}", loaded_wallet);
    
    Ok(())
}
