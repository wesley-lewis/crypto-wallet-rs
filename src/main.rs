mod eth_wallet;

fn main() {
    let (secret_key, pub_key) = eth_wallet::generate_keypair();

    println!("Secret key: {}", secret_key.to_string());
    println!("Public Key: {}", pub_key.to_string());

    let address = eth_wallet::public_key_address(&pub_key);
    println!("Address: {}", address.to_string());
}