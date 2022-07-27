pub mod utils;
use std::fs::File;
use crate::utils::{pow_mod, IsPrime, Miller};
use async_recursion::async_recursion;
use ibig::{ubig, UBig};
use std::mem::{self, MaybeUninit};
use std::sync::{Arc, Mutex};
use std::io::Read;
#[cfg(target_os = "windows")]
use core::ffi::{c_long, c_ulong, c_void};
#[cfg(target_os = "windows")]
use std::ptr;



#[cfg(target_os = "windows")]
type NTSTATUS = c_long;
#[cfg(target_os = "windows")]
type LPVOID = *mut c_void;
#[cfg(target_os = "windows")]
type ULONG = c_ulong;
#[link(name = "bcrypt")]
#[cfg(target_os = "windows")]
extern "system" {
	pub fn BCryptGenRandom(
        hAlgorithm: LPVOID,
        pBuffer: *mut u8,
        cbBuffer: ULONG,
        dwFlags: ULONG,
    ) -> NTSTATUS;
}



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

#[derive(Debug, PartialEq, Eq)]
pub enum RSAError {
    StandardEuclidean(String),
    EGenError(String),
}

type Result<T> = std::result::Result<T, RSAError>;

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
    use rayon::prelude::*;
    use std::thread;
    //eprint!("\rGenerating Prime.");
    let init: [UBig; 100] = {
        let mut data: [MaybeUninit<UBig>; 100] = unsafe { MaybeUninit::uninit().assume_init() };
        // Safety: UBig does not implement Copy, so we cannot do [ubig!(0);100]
        // Therefore, we are using ptr to initialize the array with 0s
        // Also, the size of the array is the exact same as the size of the space we are iterating through.
        // Thus, we can safely initialize it as it would break the for loop before the safe code.
        for elem in &mut data[..] {
            elem.write(ubig!(0));
        }
        unsafe { mem::transmute::<_, [UBig; 100]>(data) }
    };

    let veccer = Arc::new(Mutex::new(init));
    let mut handles = vec![];

    for i in 0..100 {
        let cloned = Arc::clone(&veccer);
        let handle = thread::spawn(move || {
            let mut buf: [u8;256] = [0;256];
            #[cfg(target_os = "windows")]
            let _ = BCryptGenRandom(ptr::null_mut(),buf.as_mut_ptr(), buf.len() as u64, 0x00000002 );
            #[cfg(not(target_os = "windows"))]
            {
              let mut fd = File::open("/dev/urandom").unwrap();
              fd.read_exact(&mut buf).unwrap();
            }
            let mut num = cloned.lock().unwrap();
            let mut candy = UBig::from_le_bytes(&buf);
            if &candy % ubig!(2) == ubig!(0) {
              candy += ubig!(1);
            }
            (*num)[i] = candy;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
      {
    let t = veccer.lock().unwrap();
    let q = t
        .par_iter()
        .find_any(|&x| x.probably_prime(40) == IsPrime::Probably);

    if let Some(ret) = q {
       return ret.clone();
    }
   }
      generate_prime().await
}

#[inline(always)]
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
    std::str::from_utf8(&input.to_le_bytes())
        .unwrap()
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ibig::ubig;

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
        assert_eq!(denum, t);
    }
    #[cfg(target_os = "windows")]
    #[test]
    fn bcrypt_gen() {
      let mut arr: [u8;5] = [0;5];
      let _ = BCryptGenRandom(ptr::null_mut(),arr.to_mut_ptr(), arr.len(), 0x00000002 );
      eprintln!("{arr:?}");
    }
}
