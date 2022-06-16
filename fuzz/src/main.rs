#[macro_use]
extern crate afl;
use minimal_rsa::*;
fn main() {
    fuzz!(|data: &[u8]| {
if let Ok(data) = std::str::from_utf8(data) {
        if data.len() > 10 {
            let _rsa = RSA::init();
            let data = data.trim_matches('\0');
            let numbered = numbify(data);
            let t = _rsa.encrypt(numbered);
            println!("DATA !!!!! {data}");
            assert_eq!(data, denumbify(_rsa.decrypt(t)));
        }
    }
    });
}
