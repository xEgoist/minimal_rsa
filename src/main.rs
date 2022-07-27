use async_std::io;
use futures::join;
use minimal_rsa::RSA;
use minimal_rsa::{denumbify, numbify};
#[async_std::main]
async fn main() {
    println!("Please Enter The Secret Text That You Wish To Encrypt");
    let mut input = String::new();
    let inpu = io::stdin();
    let _rsa = join!(RSA::init(), inpu.read_line(&mut input));
    input = input.trim().trim_matches('\0').to_owned();
    let numbered = numbify(&input);
    let t = _rsa.0.encrypt(numbered);

    println!("Encrypted Text: {t}");
    let decrypted = _rsa.0.decrypt(t);
    println!("Decrypted Text: {}", denumbify(decrypted));
}
