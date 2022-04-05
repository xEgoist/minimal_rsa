mod lib;

use rug::Integer;
use lib::RSA;


fn main() {
    let _rsa = RSA::init();
    let t = _rsa.encrypt(Integer::from(500));

    println!("Encrypted Text: {t}");

    println!("Decrypted Text: {}", _rsa.decrypt(t));
}
