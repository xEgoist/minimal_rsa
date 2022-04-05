mod lib;

use minimal_rsa::RSA;
use minimal_rsa::{denumbify, numbify};
use rug::Integer;

fn main() {
    let _rsa = RSA::init();
    let numbered = numbify("HE");
    let t = _rsa.encrypt(numbered);

    println!("Encrypted Text: {t}");
    let decrypted = _rsa.decrypt(t);
    println!("Decrypted Text: {}", denumbify(decrypted));
}
