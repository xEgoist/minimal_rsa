pub mod utils;

use crate::utils::{pow_mod, IsPrime, Miller};
use ibig::{UBig, ubig};
use rand::prelude::*;
use std::alloc::System;
use std::sync::{Arc, Mutex};

#[global_allocator]
static A: System = System;

#[allow(clippy::upper_case_acronyms)]
#[derive(Default, Clone)]
pub struct RSA {
    pub phi: UBig,
    pub p: UBig,
    pub q: UBig,
    pub pq: UBig,
    pub e: UBig,
    pub d: UBig,
}

impl RSA {
    //TODO: Fix me later
    pub fn init() -> Self {
        let mut rsa = RSA::default();
        rsa.p = generate_prime();
        rsa.q = rsa.p.clone();
        while rsa.q == rsa.p && &rsa.q % &rsa.p == ubig!(0) {
            rsa.q = generate_prime();
        }
        rsa.pq = &rsa.p * &rsa.q.clone();
        rsa.phi = (&rsa.p - ubig!(1)) * (&rsa.q - ubig!(1));
        // Fermat Prime (Used in OpenSSL)
        rsa.e = ubig!(65537);
        println!(
            "\np = {}\nq = {}\nphi={}\ne= {}\n",
            rsa.p, rsa.q, rsa.phi, rsa.e
        );
        rsa.d = modinv(&rsa.e, &rsa.phi).unwrap();
        println!("D is {}", rsa.d);
        rsa
    }

    pub fn generate_prime_between_phi(&self) -> UBig {
        let mut rng = thread_rng();

        loop {
            let ret = rng.gen_range(ubig!(3)..self.phi.clone());
            if ret.probably_prime(40) != IsPrime::Probably {
                continue;
            }
            if &self.phi % &ret == ubig!(0) {
                continue;
            }
            return ret;
        }
    }

    pub fn encrypt(&self, input: UBig) -> UBig {
        pow_mod(input, self.e.clone(), &self.pq)
    }
    pub fn decrypt(&self, input: UBig) -> UBig {
        //let dp = &self.d % (&self.p - ubig!(1));
        //let dq = &self.d % (&self.q - ubig!(1));

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

fn egcd(a: &UBig, b: &UBig) -> (UBig, UBig, UBig) {
    if a == &ubig!(0) {
        return (b.clone(), ubig!(0), ubig!(1));
    }
    let (g, y, x) = egcd(&(b % a), a);
    (g, x + (b / a) * &y, y)
}

pub fn modinv(a: &UBig, m: &UBig) -> Result<UBig> {
    let mut iter = 1;
    let mut u1 = ubig!(1);
    let mut u3 = a.clone();
    let mut v1 = ubig!(0);
    let mut v3 = m.clone();
    while v3 != ubig!(0) {
        let q = &u3 / &v3;
        let t3 = &u3 % &v3;
        let t1 = &u1 + q * &v1;
        u1 = v1.clone();
        v1 = t1;
        u3 = v3.clone();
        v3 = t3;
        iter = -iter;
    }
    if u3 != ubig!(1) {
        return Err(RSAError::StandardEuclidean(
            "Something went wrong, you probably didn't supply a prime".to_owned(),
        ));
    }
    if iter < 0 {
        let inv = m - u1;
        Ok(inv)
    } else {
        let inv = u1;
        Ok(inv)
    }
}

pub fn crt(dq: UBig, dp: UBig, p: &UBig, q: &UBig, c: UBig) -> UBig {
    let m1 = pow_mod(c.clone(), dp, p);
    let m2 = pow_mod(c, dq, q);
    let qinv = modinv(q, p).unwrap();

    ((((m1 - &m2) * qinv) % p) * q) + &m2
}

pub fn generate_prime() -> UBig {
    use rayon::prelude::*;
    eprint!("\rGenerating Prime.");
    loop {
        let ret = ubig!(0);
        let mutexed = Arc::new(Mutex::new(ret));
        (0..2048).into_par_iter().for_each(|x| {
            let boo: bool = random();
            let mut t = mutexed.lock().unwrap();
            if boo {
                t.set_bit(x);
            }
        });

        if mutexed.lock().unwrap().probably_prime(40) == IsPrime::Probably {
            eprint!("\x1b[2K\r\n");
            return (*mutexed.lock().unwrap()).clone();
        }
        eprint!(".");
    }
}

pub fn numbify(input: &str) -> UBig {
    let mut num = ubig!(0);
    for c in input.chars() {
        num = (num * 0x110000) + c as u8;
    }
    num
}

pub fn denumbify(input: UBig) -> String {
    let mut v = vec![];
    let mut copy = input;
    while copy != ubig!(0) {
        v.push((&copy % (0x110000_u32)) as u8 as char);
        copy /= ubig!(0x110000);
    }
    v.reverse();
    String::from_iter(v)
}

#[cfg(test)]
mod tests {
    use ibig::{UBig, ubig};
    use super::*;

    #[test]
    fn test_pow_mod_neg() {
        assert_eq!(modinv(&ubig!(38), &ubig!(97)), Ok(ubig!(23)));
    }

    #[test]
    fn it_works() {
        assert_eq!(modinv(&ubig!(7), &ubig!(20)).unwrap(), ubig!(3));
    }

    #[test]
    fn numbification() {
        let t = "Hello World";
        assert_eq!(
            numbify(t),
            "212139510922239649191555332064962889369514977791303932739059812"
                .parse::<UBig>()
                .unwrap()
        );
    }

    #[test]
    fn denumbification() {
        assert_eq!(
            denumbify(
                "212139510922239649191555332064962889369514977791303932739059812"
                    .parse::<UBig>()
                    .unwrap()
            ),
            "Hello World".to_owned()
        )
    }
}
