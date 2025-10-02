use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
use std::str::FromStr;
use std::num::ParseIntError;

fn gcd(a: i64, b:i64) -> i64 {
    let mut a = a.abs();
    let mut b = b.abs();
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fraction {
    numerator: i64,
    denominator: i64,
}

impl Fraction {
    pub fn new(numerator: i64, denominator: i64) -> Self {
        assert!(denominator != 0, "Denominator cannot be zero");
        let mut frac = Fraction { numerator, denominator };
        frac.simplify();
        frac
    }

    fn simplify(&mut self) {
        let gcd_val = gcd(self.numerator, self.denominator);
        
        self.numerator /= gcd_val;
        self.denominator /= gcd_val;

        if self.denominator < 0 {
            self.denominator *= -1;
            self.numerator *= -1;
        }
    }  
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.denominator == 1 {
            write!(f, "{}", self.numerator)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

impl FromStr for Fraction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let s = s.trim();

        if let Some((num_str, denom_str)) = s.split_once('/') {
            let num = num_str.trim().parse::<i64>()?;
            let denom = denom_str.trim().parse::<i64>()?;
            return Ok(Fraction::new(num, denom));
        }


        let num = s.parse::<i64>()?;
        Ok(Fraction::new(num, 1))
    }
}

impl Add for Fraction {
    type Output = Fraction;

    fn add(self, rhs: Self) -> Fraction {
        let numerator = self.numerator * rhs.denominator + rhs.numerator * self.denominator;
        let denominator = self.denominator * rhs.denominator;
        Fraction::new(numerator, denominator)
    }
}

impl Add<i64> for Fraction {
    type Output = Fraction;

    fn add(self, rhs: i64) -> Fraction {
        let rhs_frac = Fraction::new(rhs, 1);
        self + rhs_frac
    }
}

impl Add<Fraction> for i64 {
    type Output = Fraction;

    fn add(self, rhs: Fraction) -> Fraction {
        Fraction::new(self, 1) + rhs
    }
}

impl Sub for Fraction {
    type Output = Fraction;

    fn sub(self, rhs: Self) -> Fraction {
        let numerator = self.numerator * rhs.denominator - rhs.numerator * self.denominator;
        let denominator = self.denominator * rhs.denominator;
        Fraction::new(numerator, denominator)
    }
}

impl Sub<i64> for Fraction {
    type Output = Fraction;

    fn sub(self, rhs: i64) -> Fraction {
        self - Fraction::new(rhs, 1)
    }
}

impl Sub<Fraction> for i64 {
    type Output = Fraction;

    fn sub(self, rhs: Fraction) -> Fraction {
        Fraction::new(self, 1) - rhs
    }
}

impl Mul for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: Self) -> Fraction {
        Fraction::new(self.numerator * rhs.numerator, self.denominator * rhs.denominator)
    }
}

impl Mul<i64> for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: i64) -> Fraction {
        Fraction::new(self.numerator * rhs, self.denominator)
    }
}

impl Mul<Fraction> for i64 {
    type Output = Fraction;

    fn mul(self, rhs: Fraction) -> Fraction {
        Fraction::new(rhs.numerator * self, rhs.denominator)
    }
}

impl Div for Fraction {
    type Output = Fraction;

    fn div(self, rhs: Self) -> Fraction {
        Fraction::new(self.numerator * rhs.denominator, self.denominator * rhs.numerator)
    }
}

impl Div<i64> for Fraction {
    type Output = Fraction;

    fn div(self, rhs: i64) -> Fraction {
        assert!(rhs != 0, "Cannot divide by zero");
        Fraction::new(self.numerator, self.denominator * rhs)
    }
}

impl Div<Fraction> for i64 {
    type Output = Fraction;

    fn div(self, rhs: Fraction) -> Fraction {
        assert!(rhs.numerator != 0, "Cannot divide by zero");
        Fraction::new(self * rhs.denominator, rhs.numerator)
    }
}