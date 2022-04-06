use std::ops::{Mul, Rem, Sub};
use rand::prelude::*;


use rug::ops::DivRounding;
use rug::{Assign, Integer};
use rug::integer::IsPrime;
use rug::rand::RandState;


#[allow(clippy::upper_case_acronyms)]
#[derive(Default)]
pub struct RSA {
    pub phi: Integer,
    pub p: Integer,
    pub q: Integer,
    pub pq: Integer,
    pub e: Integer,
    pub d: Integer,
}

impl RSA {
    //TODO: Fix me later
    pub fn init() -> Self {
        let mut rsa = RSA::default();
        rsa.p = generate_prime();
        rsa.q = rsa.p.clone();
        while rsa.q == rsa.p && rsa.q.clone() % rsa.p.clone() == 0 {
            rsa.q.assign(generate_prime());
        }
        rsa.pq.assign(rsa.p.clone().mul(rsa.q.clone()));
        rsa.phi = (rsa.p.clone().sub(1_i32)).mul(rsa.q.clone().sub(1_i32));
        rsa.e = rsa.generate_prime_between_phi();
        println!(
            "p = {}, q = {}\nphi={}, e= {}",
            rsa.p, rsa.q, rsa.phi, rsa.e
        );
        rsa.d = rsa.e.clone().pow_mod(&Integer::from(-1), &rsa.phi).unwrap();
        println!("D is {}", rsa.d);
        rsa
    }
    //pub fn find_e(&mut self) -> Result<()> {
    //    let mut temp = self.phi;
    //    if self.phi > u32::MAX as usize {
    //        temp = u32::MAX as usize;
    //    }
    //    self.e = self.generate_prime_between(3, temp as u32)?;
    //    Ok(())
    //}
    fn generate_prime_between_phi(&self) -> Integer {
        let mut rand = RandState::new();
        rand.seed(&Integer::from(rand::thread_rng().gen::<u32>()));

        loop {
            let i = Integer::random_below(self.phi.clone(), &mut rand);
            if i.is_probably_prime(30) == IsPrime::No {
                continue;
            }
            if self.phi.clone() % i.clone() == 0 {
                continue;
            }
            return i;
        }
    }

    pub fn encrypt(&self, input: Integer) -> Integer {
        input
            .pow_mod(&self.e.clone(), &self.pq.clone())
            .unwrap()
    }
    pub fn decrypt(&self, input: Integer) -> Integer {
        let output = input.pow_mod(&self.d.clone(), &self.pq.clone());
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

pub fn euclidean(mut lhs: Integer, rhs: Integer) -> Result<Integer> {
    let (mut a, mut b, mut u) = (Integer::from(0_u32), rhs.clone(), Integer::from(1_u32));
    let cloned = lhs.clone();
    while lhs.clone() > 0 {
        let q = b.clone().div_floor(lhs.clone());
        println!("{b} {rhs}");
        lhs.assign(b.clone().rem(lhs.clone()));
        a.assign(u.clone());
        b.assign(lhs.clone());
        u.assign(a.clone() - q.clone() * u.clone());
    }
    if b == 1 || b == cloned {
        return Ok(a % rhs);
    }
    Err(RSAError::StandardEuclidean(
        format!("ERROR, You Probably Didn't Supply a CoPrime Remember\
    to call with small,big {b}")
        ,
    ))
}

pub fn generate_prime() -> Integer {
    let mut rand = RandState::new();
    rand.seed(&Integer::from(rand::thread_rng().gen::<u32>()));
    let mut i = Integer::from(Integer::random_bits(2048, &mut rand));

    while i.is_probably_prime(30) == IsPrime::No {
        i.assign(Integer::random_bits(2048, &mut rand));
    }
    i
}

pub fn numbify(input: &str) -> Integer {
    let mut num = Integer::from(0_u32);
    for c in input.chars() {
        num = (num * 0x110000) + c as u8;
    }
    num
}

pub fn denumbify(input: Integer) -> String {
    let mut v = vec![];
    let mut copy = input;
    while copy != 0 {
        v.push((copy.mod_u(0x110000_u32)) as u8 as char);
        copy = copy.div_floor(0x110000_i32);
    }
    v.reverse();
    String::from_iter(v)
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
