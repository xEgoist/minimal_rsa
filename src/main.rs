use minimal_rsa::RSA;
use minimal_rsa::{denumbify, numbify};

fn main() {
    let _rsa = RSA::init();
    let numbered = numbify("HELLO WORLD");
    let t = _rsa.encrypt(numbered);

    println!("Encrypted Text: {t}");
    let decrypted = _rsa.decrypt(t);
    println!("Decrypted Text: {}", denumbify(decrypted));
}
