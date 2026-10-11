use std::ops::{Add, Mul, Neg, Sub};
use std::str::FromStr;

/// A 106-bit double-double precision floating-point number represented as the unevaluated
/// sum of two IEEE 754 double precision numbers: value = hi + lo, where |lo| <= 0.5 ulp(hi).
/// Provides ~31-32 decimal digits of precision for deep fractal zooming up to 10^30x.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DoubleDouble {
    pub hi: f64,
    pub lo: f64,
}

impl DoubleDouble {
    pub const ZERO: Self = Self { hi: 0.0, lo: 0.0 };
    pub const ONE: Self = Self { hi: 1.0, lo: 0.0 };
    pub const TWO: Self = Self { hi: 2.0, lo: 0.0 };

    #[inline(always)]
    pub const fn new(hi: f64, lo: f64) -> Self {
        Self { hi, lo }
    }

    #[inline(always)]
    pub const fn from_f64(v: f64) -> Self {
        Self { hi: v, lo: 0.0 }
    }

    /// Exact sum of two f64 values: a + b = s + e
    #[inline(always)]
    pub fn two_sum(a: f64, b: f64) -> (f64, f64) {
        let s = a + b;
        let v = s - a;
        let e = (a - (s - v)) + (b - v);
        (s, e)
    }

    /// Fast sum when |a| >= |b|: a + b = s + e
    #[inline(always)]
    pub fn quick_two_sum(a: f64, b: f64) -> (f64, f64) {
        let s = a + b;
        let e = b - (s - a);
        (s, e)
    }

    /// Exact product using hardware FMA: a * b = p + e
    #[inline(always)]
    pub fn two_prod(a: f64, b: f64) -> (f64, f64) {
        let p = a * b;
        let e = a.mul_add(b, -p);
        (p, e)
    }

    #[inline(always)]
    pub fn add_f64(self, rhs: f64) -> Self {
        let (s1, s2) = Self::two_sum(self.hi, rhs);
        let (hi, lo) = Self::quick_two_sum(s1, s2 + self.lo);
        Self { hi, lo }
    }

    #[inline(always)]
    pub fn mul_f64(self, rhs: f64) -> Self {
        let (p1, p2) = Self::two_prod(self.hi, rhs);
        let p2 = p2 + self.lo * rhs;
        let (hi, lo) = Self::quick_two_sum(p1, p2);
        Self { hi, lo }
    }

    #[inline(always)]
    pub fn sqr(self) -> Self {
        let (p1, p2) = Self::two_prod(self.hi, self.hi);
        let p2 = p2 + (2.0 * self.hi * self.lo);
        let (hi, lo) = Self::quick_two_sum(p1, p2);
        Self { hi, lo }
    }

    #[inline(always)]
    pub fn abs(self) -> Self {
        if self.hi < 0.0 {
            -self
        } else {
            self
        }
    }

    #[inline(always)]
    pub fn to_f64(self) -> f64 {
        self.hi + self.lo
    }
}

impl Add for DoubleDouble {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        let (s1, s2) = Self::two_sum(self.hi, rhs.hi);
        let (t1, t2) = Self::two_sum(self.lo, rhs.lo);
        let c = s2 + t1;
        let (v1, v2) = Self::quick_two_sum(s1, c);
        let w = t2 + v2;
        let (hi, lo) = Self::quick_two_sum(v1, w);
        Self { hi, lo }
    }
}

impl Sub for DoubleDouble {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        self + (-rhs)
    }
}

impl Neg for DoubleDouble {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self {
            hi: -self.hi,
            lo: -self.lo,
        }
    }
}

impl Mul for DoubleDouble {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        let (p1, p2) = Self::two_prod(self.hi, rhs.hi);
        let p2 = p2 + (self.hi * rhs.lo + self.lo * rhs.hi);
        let (hi, lo) = Self::quick_two_sum(p1, p2);
        Self { hi, lo }
    }
}

impl From<f64> for DoubleDouble {
    #[inline(always)]
    fn from(v: f64) -> Self {
        Self::from_f64(v)
    }
}

impl FromStr for DoubleDouble {
    type Err = std::num::ParseFloatError;

    /// Parse a high-precision decimal string into a DoubleDouble with up to 32 digits of accuracy
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Ok(Self::ZERO);
        }

        let is_neg = s.starts_with('-');
        let s_abs = s.trim_start_matches('-').trim_start_matches('+');

        // Split into integer, fractional, and exponent parts
        let (mantissa, exp_val) = match s_abs.split_once(['e', 'E']) {
            Some((m, e)) => (m, e.parse::<i32>().unwrap_or(0)),
            None => (s_abs, 0),
        };

        let mut result = Self::ZERO;
        let mut seen_dot = false;
        let mut frac_scale = Self::ONE;
        let ten = Self::from_f64(10.0);
        let one_tenth = Self::from_f64(0.1);

        for ch in mantissa.chars() {
            if ch == '.' {
                seen_dot = true;
                continue;
            }
            if let Some(digit) = ch.to_digit(10) {
                let d = Self::from_f64(digit as f64);
                if !seen_dot {
                    result = result * ten + d;
                } else {
                    frac_scale = frac_scale * one_tenth;
                    result = result + d * frac_scale;
                }
            }
        }

        // Apply exponent if present
        if exp_val > 0 {
            for _ in 0..exp_val {
                result = result * ten;
            }
        } else if exp_val < 0 {
            for _ in 0..(-exp_val) {
                result = result * one_tenth;
            }
        }

        Ok(if is_neg { -result } else { result })
    }
}

/// A complex number with DoubleDouble components for 106-bit high-precision fractal computation.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ComplexDD {
    pub re: DoubleDouble,
    pub im: DoubleDouble,
}

impl ComplexDD {
    pub const ZERO: Self = Self {
        re: DoubleDouble::ZERO,
        im: DoubleDouble::ZERO,
    };

    #[inline(always)]
    pub const fn new(re: DoubleDouble, im: DoubleDouble) -> Self {
        Self { re, im }
    }

    #[inline(always)]
    pub fn from_f64(re: f64, im: f64) -> Self {
        Self {
            re: DoubleDouble::from_f64(re),
            im: DoubleDouble::from_f64(im),
        }
    }

    /// Complex squaring: (re + i*im)^2 = (re^2 - im^2) + i*(2*re*im)
    #[inline(always)]
    pub fn sqr(self) -> Self {
        let re_sqr = self.re.sqr();
        let im_sqr = self.im.sqr();
        let new_re = re_sqr - im_sqr;
        let new_im = (self.re * self.im).mul_f64(2.0);
        Self {
            re: new_re,
            im: new_im,
        }
    }

    #[inline(always)]
    pub fn add(self, rhs: Self) -> Self {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }

    /// Fast norm squared approximation using the high 53-bit part for escape radius checks
    #[inline(always)]
    pub fn norm_sqr_f64(self) -> f64 {
        self.re.hi * self.re.hi + self.im.hi * self.im.hi
    }
}
