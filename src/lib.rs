pub mod utils;

use crate::utils::{pow_mod, IsPrime, Miller};
use ibig::{ibig, IBig};
use rand::prelude::*;
use std::alloc::System;

#[global_allocator]
static A: System = System;

#[allow(clippy::upper_case_acronyms)]
#[derive(Default, Clone)]
pub struct RSA {
    pub phi: IBig,
    pub p: IBig,
    pub q: IBig,
    pub pq: IBig,
    pub e: IBig,
    pub d: IBig,
}

impl RSA {
    //TODO: Fix me later
    pub fn init() -> Self {
        let mut rsa = RSA::default();
        rsa.p = generate_prime();
        rsa.q = rsa.p.clone();
        while rsa.q == rsa.p && &rsa.q % &rsa.p == ibig!(0) {
            rsa.q = generate_prime();
        }
        rsa.pq = &rsa.p * &rsa.q.clone();
        rsa.phi = (&rsa.p - ibig!(1)) * (&rsa.q - ibig!(1));
        // Fermat Prime (Used in OpenSSL)
        rsa.e = ibig!(65537);
        println!(
            "\np = {}\nq = {}\nphi={}\ne= {}\n",
            rsa.p, rsa.q, rsa.phi, rsa.e
        );
        rsa.d = modinv(&rsa.e, &rsa.phi).unwrap();
        println!("D is {}", rsa.d);
        rsa
    }

    pub fn generate_prime_between_phi(&self) -> IBig {
        let mut rng = thread_rng();

        loop {
            let ret = rng.gen_range(ibig!(3)..self.phi.clone());
            if ret.probably_prime(40) != IsPrime::Probably {
                continue;
            }
            if &self.phi % &ret == ibig!(0) {
                continue;
            }
            return ret;
        }
    }

    pub fn encrypt(&self, input: IBig) -> IBig {
        pow_mod(input, self.e.clone(), &self.pq)
    }
    pub fn decrypt(&self, input: IBig) -> IBig {
        //let dp = &self.d % (&self.p - ibig!(1));
        //let dq = &self.d % (&self.q - ibig!(1));

        //crt(dq, dp, &self.p, &self.q, input)
        pow_mod(input, self.d.clone(), &self.pq)
    }
}

#[derive(Debug, PartialEq)]
pub enum RSAError {
    StandardEuclidean(String),
    EGenError(String),
}

type Result<T> = std::result::Result<T, RSAError>;

fn egcd(a: &IBig, b: &IBig) -> (IBig, IBig, IBig) {
    if a == &ibig!(0) {
        return (b.clone(), ibig!(0), ibig!(1));
    }
    let (g, y, x) = egcd(&(b % a), a);
    (g, x - (b / a) * &y, y)
}

pub fn modinv(a: &IBig, m: &IBig) -> Result<IBig> {
    let (g, x, _y) = egcd(a, m);

    if g != ibig!(1) {
        return Err(RSAError::StandardEuclidean(
            "Something went wrong, you probably didn't supply a prime".to_owned(),
        ));
    }
    let mut ret = x % m;
    if ret < ibig!(0) {
        ret += m;
    }
    Ok(ret)
}

pub fn crt(dq: IBig, dp: IBig, p: &IBig, q: &IBig, c: IBig) -> IBig {
    let m1 = pow_mod(c.clone(), dp.clone(), &p);
    let m2 = pow_mod(c, dq, &q);
    let qinv = modinv(&q, &p).unwrap();

    return ((((m1 - &m2) * qinv) % p) * q) + &m2;
}

pub fn generate_prime() -> IBig {
    let mut rng = thread_rng();
    eprint!("\rGenerating Prime.");
    loop {
        let ret: IBig = rng.gen_range(ibig!(5000)..ibig!(2).pow(1024));
        if ret.probably_prime(40) == IsPrime::Probably {
            eprint!("\x1b[2K\r\n");
            return ret;
        }
        eprint!(".");
    }
}

pub fn numbify(input: &str) -> IBig {
    let mut num = ibig!(0);
    for c in input.chars() {
        num = (num * 0x110000) + c as u8;
    }
    num
}

pub fn denumbify(input: IBig) -> String {
    let mut v = vec![];
    let mut copy = input;
    while copy != ibig!(0) {
        v.push((&copy % (0x110000_u32)).to_f32() as u8 as char);
        copy /= ibig!(0x110000);
    }
    v.reverse();
    String::from_iter(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow_mod_neg() {
        assert_eq!(modinv(&ibig!(38), &ibig!(97)), Ok(ibig!(23)));
    }

    #[test]
    fn it_works() {
        assert_eq!(modinv(&ibig!(7), &ibig!(20)).unwrap(), ibig!(3));
    }

    #[test]
    fn numbification() {
        let t = "Hello World";
        assert_eq!(
            numbify(t),
            "212139510922239649191555332064962889369514977791303932739059812"
                .parse::<IBig>()
                .unwrap()
        );
    }

    #[test]
    fn denumbification() {
        assert_eq!(
            denumbify(
                "212139510922239649191555332064962889369514977791303932739059812"
                    .parse::<IBig>()
                    .unwrap()
            ),
            "Hello World".to_owned()
        )
    }
}
