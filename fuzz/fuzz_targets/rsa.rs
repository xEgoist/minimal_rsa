#![no_main]

use libfuzzer_sys::fuzz_target;
use minimal_rsa::{denumbify, numbify};
use minimal_rsa::{Integer, RSA};
fuzz_target!(|data: &[u8]| {
    let _rsa = RSA::init();
    if let Ok(data) = std::str::from_utf8(data) {
        let numbered = numbify(data);
        let t = _rsa.encrypt(Integer::from(numbered));
        println!("DATA !!!!! {data}");
        assert_eq!(data, denumbify(_rsa.decrypt(t)));
    }
});
