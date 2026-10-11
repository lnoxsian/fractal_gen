use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

/// A lightweight 2D complex number with operations needed for fractal equations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex64 {
    pub re: f64,
    pub im: f64,
}

impl Complex64 {
    #[inline(always)]
    pub const fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    #[inline(always)]
    pub const fn zero() -> Self {
        Self { re: 0.0, im: 0.0 }
    }

    #[inline(always)]
    pub fn norm_sqr(self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    #[inline(always)]
    pub fn norm(self) -> f64 {
        self.norm_sqr().sqrt()
    }

    #[inline(always)]
    pub fn arg(self) -> f64 {
        self.im.atan2(self.re)
    }

    #[inline(always)]
    pub fn conj(self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    /// z^2
    #[inline(always)]
    pub fn sqr(self) -> Self {
        Self {
            re: self.re * self.re - self.im * self.im,
            im: 2.0 * self.re * self.im,
        }
    }

    /// z^d for integer power d
    pub fn powi(self, d: i32) -> Self {
        if d == 0 {
            return Self::new(1.0, 0.0);
        }
        if d == 2 {
            return self.sqr();
        }
        if d == 3 {
            return self * self.sqr();
        }
        if d == 4 {
            let s = self.sqr();
            return s.sqr();
        }
        let r = self.norm();
        if r == 0.0 {
            return Self::zero();
        }
        let theta = self.arg();
        let r_d = r.powi(d);
        let theta_d = theta * (d as f64);
        Self::new(r_d * theta_d.cos(), r_d * theta_d.sin())
    }

    /// z^p for floating point power p
    pub fn powf(self, p: f64) -> Self {
        let r = self.norm();
        if r == 0.0 {
            return Self::zero();
        }
        let theta = self.arg();
        let r_p = r.powf(p);
        let theta_p = theta * p;
        Self::new(r_p * theta_p.cos(), r_p * theta_p.sin())
    }

    /// 1 / z
    #[inline(always)]
    pub fn inv(self) -> Self {
        let denom = self.norm_sqr();
        if denom == 0.0 {
            Self::zero()
        } else {
            Self::new(self.re / denom, -self.im / denom)
        }
    }

    /// exp(z)
    pub fn exp(self) -> Self {
        let exp_re = self.re.exp();
        Self::new(exp_re * self.im.cos(), exp_re * self.im.sin())
    }

    /// sin(z) = (exp(iz) - exp(-iz)) / (2i)
    pub fn sin(self) -> Self {
        Self::new(self.re.sin() * self.im.cosh(), self.re.cos() * self.im.sinh())
    }

    /// cos(z)
    pub fn cos(self) -> Self {
        Self::new(self.re.cos() * self.im.cosh(), -self.re.sin() * self.im.sinh())
    }

    /// tan(z)
    pub fn tan(self) -> Self {
        let s = self.sin();
        let c = self.cos();
        s / c
    }
}

impl Add for Complex64 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl AddAssign for Complex64 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl Sub for Complex64 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl SubAssign for Complex64 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        self.re -= rhs.re;
        self.im -= rhs.im;
    }
}

impl Mul for Complex64 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl Mul<f64> for Complex64 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: f64) -> Self {
        Self {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}

impl MulAssign for Complex64 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl std::ops::Div for Complex64 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        let denom = rhs.re * rhs.re + rhs.im * rhs.im;
        if denom == 0.0 {
            Self::zero()
        } else {
            Self {
                re: (self.re * rhs.re + self.im * rhs.im) / denom,
                im: (self.im * rhs.re - self.re * rhs.im) / denom,
            }
        }
    }
}

impl std::ops::Div<f64> for Complex64 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: f64) -> Self {
        Self {
            re: self.re / rhs,
            im: self.im / rhs,
        }
    }
}
