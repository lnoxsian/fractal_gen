use super::mandelbrot::smooth_escape_time;
use crate::core::bounds::InterestingBounds;
use crate::core::complex::Complex64;
use crate::core::config::FractalConfig;
use crate::fractals::Fractal;

// 10. Julia Set
pub struct Julia;
impl Fractal for Julia {
    fn id(&self) -> &'static str { "julia" }
    fn name(&self) -> &'static str { "Julia Set" }
    fn family(&self) -> &'static str { "julia-family" }
    fn formula(&self) -> &'static str { "z_{n+1} = z_n^2 + c, c fixed" }
    fn description(&self) -> &'static str { "Classic Julia set where z0 varies across coordinates and c is fixed." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::julia() }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut z = Complex64::new(x, y);
        let c = Complex64::new(config.julia_cx, config.julia_cy);
        let max_iter = config.iterations;
        let escape_sqr = config.escape_radius * config.escape_radius;

        for i in 0..max_iter {
            z = z.sqr() + c;
            let n_sqr = z.norm_sqr();
            if n_sqr > escape_sqr {
                return Some(smooth_escape_time(i, n_sqr, max_iter));
            }
        }
        None
    }
}

// 11. Julia Snakes
pub struct JuliaSnakes;
impl Fractal for JuliaSnakes {
    fn id(&self) -> &'static str { "julia-snakes" }
    fn name(&self) -> &'static str { "Julia Snakes" }
    fn family(&self) -> &'static str { "julia-family" }
    fn formula(&self) -> &'static str { "z_{n+1} = z_n^2 + c with winding filament parameter" }
    fn description(&self) -> &'static str { "Julia parameter selection producing winding serpentine filaments." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::julia() }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut z = Complex64::new(x, y);
        let c = Complex64::new(-0.8, 0.156);
        let max_iter = config.iterations;
        let escape_sqr = config.escape_radius * config.escape_radius;

        for i in 0..max_iter {
            z = z.sqr() + c;
            let n_sqr = z.norm_sqr();
            if n_sqr > escape_sqr {
                return Some(smooth_escape_time(i, n_sqr, max_iter));
            }
        }
        None
    }
}

// 12. Multibrot Julia
pub struct MultibrotJulia;
impl Fractal for MultibrotJulia {
    fn id(&self) -> &'static str { "multibrot-julia" }
    fn name(&self) -> &'static str { "Multibrot Julia" }
    fn family(&self) -> &'static str { "julia-family" }
    fn formula(&self) -> &'static str { "z_{n+1} = z_n^d + c, c fixed" }
    fn description(&self) -> &'static str { "Higher-order power Julia sets producing rotational symmetries." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::julia() }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut z = Complex64::new(x, y);
        let c = Complex64::new(config.julia_cx, config.julia_cy);
        let d = if config.power != 2.0 { config.power } else { 3.0 };
        let max_iter = config.iterations;
        let escape_sqr = config.escape_radius * config.escape_radius;

        for i in 0..max_iter {
            z = z.powf(d) + c;
            let n_sqr = z.norm_sqr();
            if n_sqr > escape_sqr {
                return Some(smooth_escape_time(i, n_sqr, max_iter));
            }
        }
        None
    }
}

// 13. Burning Ship Julia
pub struct BurningShipJulia;
impl Fractal for BurningShipJulia {
    fn id(&self) -> &'static str { "burning-ship-julia" }
    fn name(&self) -> &'static str { "Burning Ship Julia" }
    fn family(&self) -> &'static str { "julia-family" }
    fn formula(&self) -> &'static str { "z_{n+1} = (|Re(z)| + i|Im(z)|)^2 + c" }
    fn description(&self) -> &'static str { "Julia equivalent of Burning Ship with absolute-value folds." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::julia() }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut z = Complex64::new(x, y);
        let c = Complex64::new(config.julia_cx, config.julia_cy);
        let max_iter = config.iterations;
        let escape_sqr = config.escape_radius * config.escape_radius;

        for i in 0..max_iter {
            let re = z.re.abs();
            let im = z.im.abs();
            z = Complex64::new(re * re - im * im + c.re, 2.0 * re * im + c.im);
            let n_sqr = z.norm_sqr();
            if n_sqr > escape_sqr {
                return Some(smooth_escape_time(i, n_sqr, max_iter));
            }
        }
        None
    }
}

// 14. Tricorn Julia
pub struct TricornJulia;
impl Fractal for TricornJulia {
    fn id(&self) -> &'static str { "tricorn-julia" }
    fn name(&self) -> &'static str { "Tricorn Julia" }
    fn family(&self) -> &'static str { "julia-family" }
    fn formula(&self) -> &'static str { "z_{n+1} = conj(z_n)^2 + c" }
    fn description(&self) -> &'static str { "Conjugate Julia dynamics generating triangular symmetry features." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::julia() }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut z = Complex64::new(x, y);
        let c = Complex64::new(config.julia_cx, config.julia_cy);
        let max_iter = config.iterations;
        let escape_sqr = config.escape_radius * config.escape_radius;

        for i in 0..max_iter {
            z = z.conj().sqr() + c;
            let n_sqr = z.norm_sqr();
            if n_sqr > escape_sqr {
                return Some(smooth_escape_time(i, n_sqr, max_iter));
            }
        }
        None
    }
}

// 15. Phoenix Julia
pub struct PhoenixJulia;
impl Fractal for PhoenixJulia {
    fn id(&self) -> &'static str { "phoenix-julia" }
    fn name(&self) -> &'static str { "Phoenix Julia" }
    fn family(&self) -> &'static str { "julia-family" }
    fn formula(&self) -> &'static str { "z_{n+1} = z_n^2 + c + p*z_{n-1}" }
    fn description(&self) -> &'static str { "Phoenix recurrence in Julia mode creating intricate winged spirals." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::julia() }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut z_prev = Complex64::zero();
        let mut z_curr = Complex64::new(y, x);
        let c = Complex64::new(config.julia_cx, config.julia_cy);
        let p = Complex64::new(-0.5, 0.0);
        let max_iter = config.iterations;
        let escape_sqr = config.escape_radius * config.escape_radius;

        for i in 0..max_iter {
            let z_next = z_curr.sqr() + c + (p * z_prev);
            z_prev = z_curr;
            z_curr = z_next;
            let n_sqr = z_curr.norm_sqr();
            if n_sqr > escape_sqr {
                return Some(smooth_escape_time(i, n_sqr, max_iter));
            }
        }
        None
    }
}

// 16. Lambda Julia
pub struct LambdaJulia;
impl Fractal for LambdaJulia {
    fn id(&self) -> &'static str { "lambda-julia" }
    fn name(&self) -> &'static str { "Lambda Julia" }
    fn family(&self) -> &'static str { "julia-family" }
    fn formula(&self) -> &'static str { "z_{n+1} = λ * z_n * (1 - z_n)" }
    fn description(&self) -> &'static str { "Complex logistic map displaying chaotic bifurcation dynamics." }
    fn bounds(&self) -> InterestingBounds {
        InterestingBounds {
            min_offset_x: -2.0,
            max_offset_x: 2.0,
            min_offset_y: -2.0,
            max_offset_y: 2.0,
            default_offset_x: 0.5,
            default_offset_y: 0.0,
            default_zoom: 1.0,
            ..InterestingBounds::default()
        }
    }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut z = Complex64::new(x, y);
        let lambda = Complex64::new(config.julia_cx, config.julia_cy);
        let max_iter = config.iterations;

        for i in 0..max_iter {
            let one_minus_z = Complex64::new(1.0 - z.re, -z.im);
            z = lambda * z * one_minus_z;
            let n_sqr = z.norm_sqr();
            if n_sqr > 20.0 {
                return Some(smooth_escape_time(i, n_sqr, max_iter));
            }
        }
        None
    }
}

// 17. Hybrid Julia
pub struct HybridJulia;
impl Fractal for HybridJulia {
    fn id(&self) -> &'static str { "hybrid-julia" }
    fn name(&self) -> &'static str { "Hybrid Julia" }
    fn family(&self) -> &'static str { "julia-family" }
    fn formula(&self) -> &'static str { "z_{n+1} = z_n^2 + c + 0.15*z_n^3" }
    fn description(&self) -> &'static str { "Hybrid polynomial combination creating novel organic Julia morphologies." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::julia() }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut z = Complex64::new(x, y);
        let c = Complex64::new(config.julia_cx, config.julia_cy);
        let max_iter = config.iterations;
        let escape_sqr = config.escape_radius * config.escape_radius;

        for i in 0..max_iter {
            z = z.sqr() + c + (z.powi(3) * 0.15);
            let n_sqr = z.norm_sqr();
            if n_sqr > escape_sqr {
                return Some(smooth_escape_time(i, n_sqr, max_iter));
            }
        }
        None
    }
}

// 18. Pickover Stalks
pub struct PickoverStalks;
impl Fractal for PickoverStalks {
    fn id(&self) -> &'static str { "pickover-stalks" }
    fn name(&self) -> &'static str { "Pickover Stalks" }
    fn family(&self) -> &'static str { "julia-family" }
    fn formula(&self) -> &'static str { "Orbit-trap distance to axes under Julia iteration" }
    fn description(&self) -> &'static str { "Visualizes orbit proximity to real and imaginary coordinate axes." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::julia() }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut z = Complex64::new(x, y);
        let c = Complex64::new(config.julia_cx, config.julia_cy);
        let max_iter = config.iterations;
        let mut min_dist = 1e10f64;

        for _ in 0..max_iter {
            z = z.sqr() + c;
            let dist_axes = z.re.abs().min(z.im.abs());
            if dist_axes < min_dist {
                min_dist = dist_axes;
            }
            if z.norm_sqr() > 100.0 {
                break;
            }
        }
        let t = (1.0 - min_dist.min(1.0)).powf(0.5);
        Some(t.clamp(0.0, 1.0))
    }
}

// 19. Biomorphs
pub struct Biomorphs;
impl Fractal for Biomorphs {
    fn id(&self) -> &'static str { "biomorphs" }
    fn name(&self) -> &'static str { "Biomorphs" }
    fn family(&self) -> &'static str { "julia-family" }
    fn formula(&self) -> &'static str { "z_{n+1} = z_n^3 + c with |Re(z)| > R or |Im(z)| > R termination" }
    fn description(&self) -> &'static str { "Clifford Pickover biomorphs producing cellular and organism-like structures." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::julia() }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut z = Complex64::new(x, y);
        let c = Complex64::new(config.julia_cx, config.julia_cy);
        let max_iter = config.iterations;
        let threshold = 10.0;

        for i in 0..max_iter {
            z = z.powi(3) + c;
            if z.re.abs() > threshold || z.im.abs() > threshold {
                return Some((i as f64 / max_iter as f64).clamp(0.0, 1.0));
            }
        }
        None
    }
}
