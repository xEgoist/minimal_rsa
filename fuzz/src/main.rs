use honggfuzz::fuzz;
use minimal_rsa::*;
fn main() {
    // Here you can parse `std::env::args and
    // setup / initialize your project

    // You have full control over the loop but
    // you're supposed to call `fuzz` ad vitam aeternam
    loop {
        // The fuzz macro gives an arbitrary object (see `arbitrary crate`)
        // to a closure-like block of code.
        // For performance reasons, it is recommended that you use the native type
        // `&[u8]` when possible.
        // Here, this slice will contain a "random" quantity of "random" data.
        fuzz!(|data: &[u8]| {
				if let Ok(data) = std::str::from_utf8(data) {
            let _rsa = RSA::init();
            let data = data.trim_matches('\0');
            let numbered = numbify(data);
            let t = _rsa.encrypt(numbered);
            println!("DATA !!!!! {data}");
            assert_eq!(data, denumbify(_rsa.decrypt(t)));
					}
       });
    }
}
