use async_std::io;

use minimal_rsa::RSA;
use minimal_rsa::{denumbify, numbify};
#[async_std::main]
async fn main() {
    println!("Please Enter The Secret Text That You Wish To Encrypt");
    let _rsa = RSA::init().await;
    let mut input = String::new();
    io::stdin().read_line(&mut input).await.unwrap();
    input = input.trim().trim_matches('\0').to_owned();
    let numbered = numbify(&input);
    let t = _rsa.encrypt(numbered);

    println!("Encrypted Text: {t}");
    let decrypted = _rsa.decrypt(t);
    println!("Decrypted Text: {}", denumbify(decrypted));
}
