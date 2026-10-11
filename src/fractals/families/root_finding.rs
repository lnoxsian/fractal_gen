use crate::core::bounds::InterestingBounds;
use crate::core::complex::Complex64;
use crate::core::config::FractalConfig;
use crate::fractals::Fractal;

const ROOTS_3: [Complex64; 3] = [
    Complex64::new(1.0, 0.0),
    Complex64::new(-0.5, 0.8660254037844386),
    Complex64::new(-0.5, -0.8660254037844386),
];

// 45. Newton Fractal
pub struct Newton;
impl Fractal for Newton {
    fn id(&self) -> &'static str { "newton" }
    fn name(&self) -> &'static str { "Newton Fractal" }
    fn family(&self) -> &'static str { "root-finding-family" }
    fn formula(&self) -> &'static str { "z_{n+1} = z_n - f(z_n)/f'(z_n) for f(z) = z^3 - 1" }
    fn description(&self) -> &'static str { "Newton-Raphson basins of attraction for polynomial roots." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(2.0, 1.2) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut z = Complex64::new(x, y);
        let max_iter = config.iterations;

        for i in 0..max_iter {
            let z_sqr = z.sqr();
            let denom = z_sqr * 3.0;
            if denom.norm_sqr() < 1e-12 { break; }
            let f = z * z_sqr - Complex64::new(1.0, 0.0);
            z = z - (f / denom);

            for (root_idx, root) in ROOTS_3.iter().enumerate() {
                if (z - *root).norm_sqr() < 1e-6 {
                    let base_hue = root_idx as f64 / 3.0;
                    let iter_factor = (i as f64 / max_iter as f64) * 0.3;
                    return Some((base_hue + iter_factor).fract());
                }
            }
        }
        None
    }
}

// 46. Halley Fractal
pub struct Halley;
impl Fractal for Halley {
    fn id(&self) -> &'static str { "halley" }
    fn name(&self) -> &'static str { "Halley Fractal" }
    fn family(&self) -> &'static str { "root-finding-family" }
    fn formula(&self) -> &'static str { "z_{n+1} = z_n - 2ff' / (2(f')^2 - ff'') for f(z) = z^3 - 1" }
    fn description(&self) -> &'static str { "Higher-order Halley root-finding method converging with cubic rapidity." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(2.0, 1.2) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut z = Complex64::new(x, y);
        let max_iter = config.iterations;

        for i in 0..max_iter {
            let z2 = z.sqr();
            let f = z * z2 - Complex64::new(1.0, 0.0);
            let fp = z2 * 3.0;
            let fpp = z * 6.0;

            let num = f * fp * 2.0;
            let den = (fp.sqr() * 2.0) - (f * fpp);
            if den.norm_sqr() < 1e-12 { break; }
            z = z - (num / den);

            for (root_idx, root) in ROOTS_3.iter().enumerate() {
                if (z - *root).norm_sqr() < 1e-6 {
                    let base_hue = root_idx as f64 / 3.0;
                    let iter_factor = (i as f64 / max_iter as f64) * 0.3;
                    return Some((base_hue + iter_factor).fract());
                }
            }
        }
        None
    }
}

// 47. Nova Fractal
pub struct Nova;
impl Fractal for Nova {
    fn id(&self) -> &'static str { "nova" }
    fn name(&self) -> &'static str { "Nova Fractal" }
    fn family(&self) -> &'static str { "root-finding-family" }
    fn formula(&self) -> &'static str { "z_{n+1} = z_n - α*f(z)/f'(z) + c + β*(z_n - z_{n-1})" }
    fn description(&self) -> &'static str { "Modified Newton iteration with relaxation parameter and memory term." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(2.0, 1.2) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let c = Complex64::new(x, y);
        let mut z_prev = Complex64::zero();
        let mut z = Complex64::new(1.0, 0.0);
        let max_iter = config.iterations;
        let alpha = Complex64::new(1.0, 0.0);
        let beta = Complex64::new(0.5, 0.0);

        for i in 0..max_iter {
            let z2 = z.sqr();
            let denom = z2 * 3.0;
            if denom.norm_sqr() < 1e-12 { break; }
            let f = z * z2 - Complex64::new(1.0, 0.0);
            let z_next = z - (alpha * (f / denom)) + c + (beta * (z - z_prev));
            z_prev = z;
            z = z_next;

            if z.norm_sqr() > 100.0 {
                return Some((i as f64 / max_iter as f64).clamp(0.0, 1.0));
            }
        }
        None
    }
}
