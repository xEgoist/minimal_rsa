use ibig::ubig;
use rand::{thread_rng, Rng};

#[derive(PartialEq, Debug)]
pub enum IsPrime {
    Probably,
    NotPrime,
}

impl Miller for ibig::UBig {
    fn probably_prime(&self, rounds: u32) -> IsPrime {
        if self == &ubig!(0) {
            return IsPrime::NotPrime;
        }
        if self <= &ubig!(5) {
            return IsPrime::Probably;
        }
        if self % ubig!(2) == ubig!(0) {
            return IsPrime::NotPrime;
        }

        let (mut r, mut s) = (0, self - ubig!(1));
        while &s & ubig!(1) != ubig!(0) {
            r += 1;
            s /= ubig!(2);
        }
        let mut ret = IsPrime::NotPrime;
        let mut rng = thread_rng();
        for _ in 0..rounds {
            let a = rng.gen_range(ubig!(2)..self - ubig!(1));
            let mut x = pow_mod(a, s.clone(), self);
            if x == ubig!(1) || x == (self - ubig!(1)) || &x % self == ubig!(0) {
                continue;
            }
            for _ in 0..r - 1 {
                x = pow_mod(x, ubig!(2), self);
                if x == (self - ubig!(1)) {
                    ret = IsPrime::Probably;
                    break;
                }
            }
            if ret == IsPrime::NotPrime {
                return IsPrime::NotPrime;
            }
        }

        IsPrime::Probably
    }
}

pub trait Miller {
    fn probably_prime(&self, rounds: u32) -> IsPrime;
}

pub fn pow_mod(b: ibig::UBig, e: ibig::UBig, m: &ibig::UBig) -> ibig::UBig {
    if e == ubig!(0) {
      return ubig!(1);
    }
    let mut ret = b.clone();
    let bits = e.bit_len() - 1;

    for bit in (0..bits).rev() {
     ret = ret.pow(2);
      ret %= m;
      if e.bit(bit) {
        ret *= &b;
        ret %= m;
      }
    }
    ret
}

pub fn recursive_mod_pow(b: &ibig::UBig, e: &ibig::UBig, m: &ibig::UBig) -> ibig::UBig {
    if *e == ubig!(0){
      return ubig!(1);
    }
    let tmp = recursive_mod_pow(b, &(e >>1), m) % m;
    if e % ubig!(2) == ubig!(0) {
      (&tmp * &tmp) % m
    }
    else if *e > ubig!(0) {
       (b * &tmp * &tmp) % m
     }
     else {
    ((&tmp * &tmp) / b) % m
     }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ibig::UBig;

    #[test]
    fn it_works() {
        let t = ubig!(3);
        assert_eq!(IsPrime::Probably, t.probably_prime(40));
    }

    #[test]
    fn test_pow_mod_1() {
        assert_eq!(ubig!(2), recursive_mod_pow(&ubig!(5), &ubig!(3), &ubig!(3)));
    }

    #[test]
    fn test_pow_mod_2() {
        let b = ubig!(5);
        let e =  ubig!(150);
        let m = ubig!(60);
        assert_eq!(ubig!(25), pow_mod(b.clone(), e.clone(), &m));
        assert_eq!(ubig!(25), recursive_mod_pow(&b, &e, &m));
    }

    #[test]
    fn test_pow_mod_e0() {
        assert_eq!(ubig!(1), pow_mod(ubig!(5), ubig!(0), &ubig!(60)));
        assert_eq!(ubig!(1), recursive_mod_pow(&ubig!(5), &ubig!(0), &ubig!(60)));
    }

    #[test]
    fn is_prime_25() {
        let t = ubig!(25);
        assert_eq!(IsPrime::NotPrime, t.probably_prime(40))
    }

    #[test]
    fn is_prime_31() {
        let t = ubig!(31);
        assert_eq!(IsPrime::Probably, t.probably_prime(40))
    }

    #[test]
    fn is_prime_101() {
        let t = "191".parse::<UBig>().unwrap();
        assert_eq!(IsPrime::Probably, t.probably_prime(40));
    }

    #[test]
    fn very_large_prime() {
        let t = "1936738294519690982211090334402079885308248998113910860490043561062398610429619904537193501740559101".parse::<UBig>().unwrap();
        assert_eq!(IsPrime::Probably, t.probably_prime(40));
    }

    #[test]
    fn is_prime_zero() {
        let t = ubig!(0);
        assert_eq!(IsPrime::NotPrime, t.probably_prime(40));
    }
}
