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
    let veccer  = Arc::new(Mutex::new(Vec::new()));
    (0..1000).into_par_iter().for_each(|_y: usize| {
        let ret = ubig!(0);
        let mutexed = Arc::new(Mutex::new(ret));
        (0..1024).into_par_iter().for_each(|x| {
            let boo: bool = random();
            let mut t = mutexed.lock().unwrap();
            if boo {
                t.set_bit(x);
            }
        });
       veccer.lock().unwrap().push( mutexed.lock().unwrap().clone());
       });
      let t =   veccer.lock().unwrap();
        let q = t.par_iter().find_any(|&x| x.probably_prime(40) == IsPrime::Probably);
      if let Some(ret) = q {
        ret.clone()
        }
else {
generate_prime()
}
}

pub fn numbify(input: &str) -> UBig {
    UBig::from_str_radix(&input.as_bytes().iter().map(|x| format!("{:02x}", x)).collect::<String>(),16).unwrap()
}

pub fn denumbify(input: UBig) -> String {
		let s =  format!("{:x}",input);
    let t: Vec<u8> = (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect();
		std::str::from_utf8(&t).unwrap().to_owned()
 // std::str::from_utf8(&hex::decode(format!("{:x}", input)).unwrap()).unwrap().to_owned()
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
        let num = numbify(t);
        let denum = denumbify(num);
        assert_eq!(denum,t);
    }
}
