pub mod utils;

use crate::utils::{pow_mod, IsPrime, Miller};
use ibig::{UBig, ubig};
use std::sync::{Arc, Mutex};
use std::mem::{self, MaybeUninit};
use async_recursion::async_recursion;
use rand_chacha::ChaCha20Rng;
use rand::SeedableRng;
use rand::Rng;


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
    pub async fn init() -> Self {
        let mut rsa = RSA::default();
        rsa.p = generate_prime().await;
        rsa.q = rsa.p.clone();
        while rsa.q == rsa.p && &rsa.q % &rsa.p == ubig!(0) {
            rsa.q = generate_prime().await;
        }
        rsa.pq = &rsa.p * &rsa.q.clone();
        rsa.phi = (&rsa.p - ubig!(1)) * (&rsa.q - ubig!(1));
        // Fermat Prime (Used in OpenSSL)
        rsa.e = ubig!(65537);
      //  println!(
      //      "\np = {}\nq = {}\nphi={}\ne= {}\n",
      //      rsa.p, rsa.q, rsa.phi, rsa.e
      //  );
        rsa.d = modinv(&rsa.e, &rsa.phi).unwrap();
      //  println!("D is {}", rsa.d);
        rsa
    }

    pub fn generate_prime_between_phi(&self) -> UBig {
        let mut rng = ChaCha20Rng::from_entropy();

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


#[async_recursion(?Send)]
#[inline(always)]
pub async fn generate_prime() -> UBig {
    use std::thread;
    use rayon::prelude::*;
    //eprint!("\rGenerating Prime.");
    let init: [UBig;1000] = {
    let mut data: [MaybeUninit<UBig>; 1000] = unsafe
        {
              MaybeUninit::uninit().assume_init()
        };
      // Safety: UBig does not implement Copy, so we cannot do [ubig!(0);100]
      // Therefore, we are using ptr to initialize the array with 0s
      // Also, the size of the array is the exact same as the size of the space we are iterating through.
      // Thus, we can safely initialize it as it would break the for loop before the safe code.
    for elem in &mut data[..] {
        elem.write(ubig!(0));
    }
    unsafe { mem::transmute::<_, [UBig; 1000]>(data) }
    };

    let veccer  = Arc::new(Mutex::new(init));
    let mut handles = vec![];
    for i in 0..1000 {
      let cloned = Arc::clone(&veccer);
      let handle = thread::spawn(move || {
        let mut candy = ubig!(0);
        let mut num = cloned.lock().unwrap();
        let mut rng = ChaCha20Rng::from_entropy();
        for b in 0..1024{
            let rand: bool = rng.gen();
            if rand {
              candy.set_bit(b);
            }
        }
      (*num)[i]=candy;
      });
      handles.push(handle);
    }
			for handle in handles {
        handle.join().unwrap();
				}

			let t = veccer.lock().unwrap();
    let q = t.par_iter().find_any(|&x| x.probably_prime(40) == IsPrime::Probably);
      if let Some(ret) = q {
        ret.clone()
        }
else {
      generate_prime().await
}
}

pub fn numbify(input: &str) -> UBig {
    //UBig::from_str_radix(&input.as_bytes().iter().map(|x| format!("{:02x}", x)).collect::<String>(),16).unwrap()
    UBig::from_le_bytes(input.as_bytes())
}

pub fn denumbify(input: UBig) -> String {
//		let s = format!("{:x}",input);
//    let t: Vec<u8> = (0..s.len())
//        .step_by(2)
//        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
//        .collect();
//		std::str::from_utf8(&t).unwrap().to_owned()
 // std::str::from_utf8(&hex::decode(format!("{:x}", input)).unwrap()).unwrap().to_owned()

  // OK IDIOT, let's make this way easier.
  std::str::from_utf8(&input.to_le_bytes()).unwrap().to_owned()

}

#[cfg(test)]
mod tests {
    use ibig::{ubig};
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
