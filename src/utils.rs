use ibig::ibig;
use rand::{thread_rng, Rng};

#[derive(PartialEq, Debug)]
pub enum IsPrime {
    Probably,
    NotPrime,
}

impl Miller for ibig::IBig {
    fn probably_prime(&self, rounds: u32) -> IsPrime {
        if self == &ibig!(2) {
            return IsPrime::Probably;
        }
        if self % ibig!(2) == ibig!(0) {
            return IsPrime::NotPrime;
        }

        let (mut r, mut s) = (0, self - ibig!(1));
        while &s % ibig!(2) == ibig!(0) {
            r += 1;
            s /= ibig!(2);
        }
        let mut ret = IsPrime::NotPrime;
        let mut rng = thread_rng();
        for _ in 0..=rounds {
            let a = rng.gen_range(ibig!(2)..=self - ibig!(1));
            let mut x = pow_mod(a, s.clone(), self);
            if x == ibig!(1) || x == (self - ibig!(1)) || &x % self == ibig!(0) {
                continue;
            }
            for _ in 0..r - 1 {
                x = pow_mod(x, ibig!(2), self);
                if x == (self - ibig!(1)) {
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

pub fn pow_mod(mut b: ibig::IBig, mut e: ibig::IBig, m: &ibig::IBig) -> ibig::IBig {
    let mut num = ibig!(1);
    b = &b % m;

    while e > ibig!(0) {
        if &e & ibig!(1) != ibig!(0) {
            num = (num * &b) % m;
        }
        e >>= 1;
        b = &b * &b % m;
    }
    num
}

#[cfg(test)]
mod tests {
    use super::*;
    use ibig::{ibig, IBig};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        let t = ibig!(3);
        assert_eq!(IsPrime::Probably, t.probably_prime(40));
    }

    #[test]
    fn test_pow_mod_1() {
        assert_eq!(ibig!(2), pow_mod(ibig!(5), ibig!(3), &ibig!(3)));
    }

    #[test]
    fn test_pow_mod_2() {
        assert_eq!(ibig!(25), pow_mod(ibig!(5), ibig!(150), &ibig!(60)));
    }

    #[test]
    fn is_prime_25() {
        let t = ibig!(25);
        assert_eq!(IsPrime::NotPrime, t.probably_prime(40))
    }

    #[test]
    fn is_prime_31() {
        let t = ibig!(31);
        assert_eq!(IsPrime::Probably, t.probably_prime(40))
    }

    #[test]
    fn is_prime_101() {
        let t = "191".parse::<IBig>().unwrap();
        assert_eq!(IsPrime::Probably, t.probably_prime(40));
    }

    #[test]
    fn very_large_prime() {
        let t = "1936738294519690982211090334402079885308248998113910860490043561062398610429619904537193501740559101".parse::<IBig>().unwrap();
        assert_eq!(IsPrime::Probably, t.probably_prime(40));
    }

    #[test]
    fn is_prime_zero() {
        let t = ibig!(0);
        assert_eq!(IsPrime::NotPrime, t.probably_prime(40));
    }
}
