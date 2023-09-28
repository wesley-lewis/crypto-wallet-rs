use anyhow::Result;
use dotenv;
use std::env;
mod eth_wallet;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    
    // let (secret_key, pub_key) = eth_wallet::generate_keypair();
    //
    // println!("Secret key: {}", &secret_key.to_string());
    // println!("Public Key: {}", &pub_key.to_string());
    //
    // let pub_address = eth_wallet::public_key_address(&pub_key);
    // println!("Address: {}", pub_address.to_string());
    //
    // let crypto_wallet = eth_wallet::Wallet::new(&secret_key, &pub_key);
    // println!("Crypto wallet: {:?}", &crypto_wallet);
    //
    // crypto_wallet.save_to_file("crypto_wallet.json")?;
    //
    let wallet_file_path = "crypto_wallet.json";

    let loaded_wallet: eth_wallet::Wallet = eth_wallet::Wallet::from_file(wallet_file_path)?;
    println!("Loaded wallet: {:?}", &loaded_wallet);

    let endpoint = env::var("INFURA_GOERLI_WS")?;
    let web3_con = eth_wallet::establish_web3_connection(&endpoint).await?;

    let block_number = web3_con.eth().block_number().await?;
    println!("Block number: {}", &block_number);

    let balance = loaded_wallet.get_balance(&web3_con).await?;
    println!("Wallet balance: {}", &balance);
    
    Ok(())
}
