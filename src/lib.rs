use rand::prelude::*;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use rug::ops::DivRounding;
use rug::Integer;


#[allow(clippy::upper_case_acronyms)]
#[derive(Default)]
pub struct RSA {
    pub phi: usize,
    pub p: usize,
    pub q: usize,
    pub pq: usize,
    pub e: u32,
    pub d: isize,
}

impl RSA {
    //TODO: Fix me later
    pub fn init() -> Self {
        let mut rsa = RSA::default();
        rsa.p = generate_prime();
        rsa.q = rsa.p;
        while rsa.q == rsa.p {
            rsa.q = generate_prime();
        }
        rsa.pq = rsa.p * rsa.q;
        rsa.phi = (rsa.p - 1) * (rsa.q - 1);
        rsa.find_e().unwrap();
        println!(
            "p = {}, q = {}\nphi={}, e= {}",
            rsa.p, rsa.q, rsa.phi, rsa.e
        );
        rsa.d = euclidean(rsa.e as isize, rsa.phi as isize).unwrap();
        println!("D is {}", rsa.d);
        rsa
    }
    pub fn find_e(&mut self) -> Result<()> {
        let mut temp = self.phi;
        if self.phi > u32::MAX as usize {
            temp = u32::MAX as usize;
        }
        self.e = self.generate_prime_between(3, temp as u32)?;
        Ok(())
    }
    fn generate_prime_between(&self, x: u32, y: u32) -> Result<u32> {
        if x > y {
            return Err(RSAError::EGenError(
                "Prime  Generation error for E ".to_owned(),
            ));
        }

        loop {
            let num: u32 = thread_rng().gen_range(x..1_000_000);
            if num % 2 == 0 {
                continue;
            }
            if self.phi > num as usize && self.phi % num as usize == 0 {
                continue;
            }
            if num as usize % self.phi == 0 {
                continue;
            }

            if (3..num).into_par_iter().all(|d| num as u32 % d != 0) {
                return Ok(num as u32);
            }
        }
    }
    pub fn encrypt(&self, input: Integer) -> Integer {
        input
            .pow_mod(&Integer::from(self.e), &Integer::from(self.pq))
            .unwrap()
    }
    pub fn decrypt(&self, input: Integer) -> Integer {
        let output = input.pow_mod(&Integer::from(self.d), &Integer::from(self.pq));
        //let bytes = &output.to_be_bytes()[..];
        //let string = std::str::from_utf8(bytes).unwrap()
        //    ;
        output.unwrap()
    }
}

#[derive(Debug)]
pub enum RSAError {
    StandardEuclidean(String),
    EGenError(String),
}

type Result<T> = std::result::Result<T, RSAError>;

pub fn euclidean(mut lhs: isize, rhs: isize) -> Result<isize> {
    let (mut a, mut b, mut u) = (0_isize, rhs, 1);
    let cloned = lhs;
    while lhs > 0 {
        let q = (b as f64 / lhs as f64).floor() as isize;
        (lhs, a, b, u) = (b % lhs, u, lhs, a - q * u);
    }
    if b == 1 || b == cloned {
        return Ok(a % rhs);
    }
    Err(RSAError::StandardEuclidean(
        "ERROR, You Probably Didn't Supply a CoPrime Remember\
    to call with small,big"
            .to_owned(),
    ))
}

pub fn generate_prime() -> usize {
    loop {
        let num: u32 = thread_rng().gen_range(5000..u16::MAX as u32);
        if num % 2 == 0 {
            continue;
        }
        if (3..num).into_par_iter().all(|d| num % d != 0) {
            return num as usize;
        }
    }
}

pub fn numbify(input: &str) -> Integer {
    let mut num = Integer::from(0_u32);
    for c in input.chars() {
        num = (num * 0x110000) + c as u8;
    }
    return num;
}

pub fn denumbify(input: Integer) -> String {
    let mut v = vec![];
    let mut copy = input.clone();
    while copy != 0 {
        v.push((copy.mod_u(0x110000_u32)) as u8 as char);
        copy = copy.div_floor(0x110000_i32);
    }
    v.reverse();
    return String::from_iter(v);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rug::Complete;

    #[test]
    fn it_works() {
        assert_eq!(super::euclidean(7 as isize, 20 as isize).unwrap(), 3);
    }

    #[test]
    fn numbification() {
        let t = "Hello World";
        assert_eq!(
            numbify(t),
            rug::Integer::parse("212139510922239649191555332064962889369514977791303932739059812")
                .unwrap()
                .complete()
        );
    }

    #[test]
    fn denumbification() {
        assert_eq!(
            denumbify(
                rug::Integer::parse(
                    "212139510922239649191555332064962889369514977791303932739059812"
                )
                .unwrap()
                .complete()
            ),
            "Hello World".to_owned()
        )
    }
}
