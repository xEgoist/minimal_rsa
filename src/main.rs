use std::io::{BufRead, stdin};
use minimal_rsa::RSA;
use minimal_rsa::{denumbify, numbify};

fn main() {
    println!("Please Enter The Secret Text That You Wish To Encrypt");
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input = input.trim().trim_matches('\0').to_owned();
    let _rsa = RSA::init();
    let numbered =
        numbify(&input);
    let t = _rsa.encrypt(numbered);

    println!("Encrypted Text: {t}");
    let decrypted = _rsa.decrypt(t);
    println!("Decrypted Text: {}", denumbify(decrypted));
}
