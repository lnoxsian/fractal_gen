use crate::core::bounds::InterestingBounds;
use crate::core::complex::Complex64;
use crate::core::config::FractalConfig;
use crate::fractals::{Fractal, FractalRenderMode};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

/// Helper for smooth escape-time normalization: nu = n + 1 - log2(ln(|z|))
#[inline(always)]
pub fn smooth_escape_time(iter: u32, z_norm_sqr: f64, max_iter: u32) -> f64 {
    let log_zn = (z_norm_sqr.ln() * 0.5).max(1e-10);
    let nu = iter as f64 + 1.0 - (log_zn.ln() / std::f64::consts::LN_2);
    (nu / max_iter as f64).clamp(0.0, 1.0)
}

// 1. Mandelbrot
pub struct Mandelbrot;
impl Fractal for Mandelbrot {
    fn id(&self) -> &'static str { "mandelbrot" }
    fn name(&self) -> &'static str { "Mandelbrot" }
    fn family(&self) -> &'static str { "mandelbrot-family" }
    fn formula(&self) -> &'static str { "z_{n+1} = z_n^2 + c, z_0 = 0" }
    fn description(&self) -> &'static str { "The classic Mandelbrot set with smooth escape-time coloring." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::mandelbrot() }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let c = Complex64::new(x, y);
        // Cardioid and period-2 bulb fast checks
        let q = (x - 0.25).powi(2) + y.powi(2);
        if q * (q + (x - 0.25)) < 0.25 * y.powi(2) || (x + 1.0).powi(2) + y.powi(2) < 0.0625 {
            return None;
        }

        let mut z = Complex64::zero();
        let max_iter = config.iterations;
        let escape_sqr = config.escape_radius * config.escape_radius;

        for i in 0..max_iter {
            z = z.sqr() + c;
            let n_sqr = z.norm_sqr();
            if n_sqr > escape_sqr {
                return Some(if config.smooth_coloring {
                    smooth_escape_time(i, n_sqr, max_iter)
                } else {
                    (i as f64 / max_iter as f64).clamp(0.0, 1.0)
                });
            }
        }
        None
    }
}

// 2. Celtic Mandelbrot
pub struct CelticMandelbrot;
impl Fractal for CelticMandelbrot {
    fn id(&self) -> &'static str { "celtic-mandelbrot" }
    fn name(&self) -> &'static str { "Celtic Mandelbrot" }
    fn family(&self) -> &'static str { "mandelbrot-family" }
    fn formula(&self) -> &'static str { "z_{n+1} = (|Re(z)| + i|Im(z)|)^2 + c" }
    fn description(&self) -> &'static str { "Mandelbrot variation with absolute value symmetries creating knot-like patterns." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::mandelbrot() }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let c = Complex64::new(x, y);
        let mut z = Complex64::zero();
        let max_iter = config.iterations;
        let escape_sqr = config.escape_radius * config.escape_radius;

        for i in 0..max_iter {
            let re = (z.re.powi(2) - z.im.powi(2)).abs() + c.re;
            let im = 2.0 * z.re * z.im + c.im;
            z = Complex64::new(re, im);
            let n_sqr = z.norm_sqr();
            if n_sqr > escape_sqr {
                return Some(smooth_escape_time(i, n_sqr, max_iter));
            }
        }
        None
    }
}

// 3. Multibrot
pub struct Multibrot;
impl Fractal for Multibrot {
    fn id(&self) -> &'static str { "multibrot" }
    fn name(&self) -> &'static str { "Multibrot" }
    fn family(&self) -> &'static str { "mandelbrot-family" }
    fn formula(&self) -> &'static str { "z_{n+1} = z_n^d + c, z_0 = 0" }
    fn description(&self) -> &'static str { "Higher-order power Mandelbrot producing multi-fold symmetries." }
    fn bounds(&self) -> InterestingBounds {
        InterestingBounds {
            default_offset_x: 0.0,
            default_offset_y: 0.0,
            ..InterestingBounds::mandelbrot()
        }
    }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let c = Complex64::new(x, y);
        let mut z = Complex64::zero();
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

// 4. Mutant Mandelbrot
pub struct MutantMandelbrot;
impl Fractal for MutantMandelbrot {
    fn id(&self) -> &'static str { "mutant-mandelbrot" }
    fn name(&self) -> &'static str { "Mutant Mandelbrot" }
    fn family(&self) -> &'static str { "mandelbrot-family" }
    fn formula(&self) -> &'static str { "z_{n+1} = z_n^2 + c + ε" }
    fn description(&self) -> &'static str { "Perturbed Mandelbrot iteration introducing morphing deformations." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::mandelbrot() }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let c = Complex64::new(x, y);
        let mut z = Complex64::zero();
        let max_iter = config.iterations;
        let escape_sqr = config.escape_radius * config.escape_radius;

        for i in 0..max_iter {
            let perturbation = Complex64::new(0.08 * (z.re * 3.0).sin(), 0.08 * (z.im * 3.0).cos());
            z = z.sqr() + c + perturbation;
            let n_sqr = z.norm_sqr();
            if n_sqr > escape_sqr {
                return Some(smooth_escape_time(i, n_sqr, max_iter));
            }
        }
        None
    }
}

// 5. Phoenix Mandelbrot
pub struct PhoenixMandelbrot;
impl Fractal for PhoenixMandelbrot {
    fn id(&self) -> &'static str { "phoenix-mandelbrot" }
    fn name(&self) -> &'static str { "Phoenix Mandelbrot" }
    fn family(&self) -> &'static str { "mandelbrot-family" }
    fn formula(&self) -> &'static str { "z_{n+1} = z_n^2 + c + p*z_{n-1}" }
    fn description(&self) -> &'static str { "Second-order recurrence with memory term creating flame-like spirals." }
    fn bounds(&self) -> InterestingBounds {
        InterestingBounds {
            default_offset_x: 0.0,
            default_offset_y: 0.0,
            min_offset_x: -2.0,
            max_offset_x: 2.0,
            min_offset_y: -2.0,
            max_offset_y: 2.0,
            ..InterestingBounds::default()
        }
    }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let c = Complex64::new(x, y);
        let mut z_prev = Complex64::zero();
        let mut z_curr = Complex64::new(y, x); // classic Phoenix coordinate swap
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

// 6. Burning Ship
pub struct BurningShip;
impl Fractal for BurningShip {
    fn id(&self) -> &'static str { "burning-ship" }
    fn name(&self) -> &'static str { "Burning Ship" }
    fn family(&self) -> &'static str { "mandelbrot-family" }
    fn formula(&self) -> &'static str { "z_{n+1} = (|Re(z)| + i|Im(z)|)^2 + c" }
    fn description(&self) -> &'static str { "Absolute components before squaring create the famous ship and mast structures." }
    fn bounds(&self) -> InterestingBounds {
        InterestingBounds {
            min_offset_x: -2.2,
            max_offset_x: 1.2,
            min_offset_y: -2.0,
            max_offset_y: 1.0,
            default_offset_x: -0.45,
            default_offset_y: -0.5,
            default_zoom: 1.0,
            ..InterestingBounds::default()
        }
    }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let c = Complex64::new(x, y);
        let mut z = Complex64::zero();
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

// 7. Tricorn
pub struct Tricorn;
impl Fractal for Tricorn {
    fn id(&self) -> &'static str { "tricorn" }
    fn name(&self) -> &'static str { "Tricorn" }
    fn family(&self) -> &'static str { "mandelbrot-family" }
    fn formula(&self) -> &'static str { "z_{n+1} = conj(z_n)^2 + c" }
    fn description(&self) -> &'static str { "Mandelbar fractal using complex conjugation creating tri-cornered symmetries." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::mandelbrot() }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let c = Complex64::new(x, y);
        let mut z = Complex64::zero();
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

// 8. Nebulabrot
pub struct Nebulabrot;
impl Fractal for Nebulabrot {
    fn id(&self) -> &'static str { "nebulabrot" }
    fn name(&self) -> &'static str { "Nebulabrot" }
    fn family(&self) -> &'static str { "mandelbrot-family" }
    fn formula(&self) -> &'static str { "Trajectory accumulation across variable iteration depth bands" }
    fn description(&self) -> &'static str { "Accumulates escaping orbits across multiple iteration bands into a nebula." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::mandelbrot() }
    fn render_mode(&self) -> FractalRenderMode { FractalRenderMode::Accumulation }

    fn render_accumulation(&self, width: u32, height: u32, config: &FractalConfig) -> Vec<f64> {
        render_orbit_accumulation(width, height, config, 150_000, 20, 500)
    }
}

// 9. Buddhabrot
pub struct Buddhabrot;
impl Fractal for Buddhabrot {
    fn id(&self) -> &'static str { "buddhabrot" }
    fn name(&self) -> &'static str { "Buddhabrot" }
    fn family(&self) -> &'static str { "mandelbrot-family" }
    fn formula(&self) -> &'static str { "P(z) = sum_{escaping orbits passing through z} 1" }
    fn description(&self) -> &'static str { "Probability density rendering of escaping Mandelbrot trajectories resembling a seated Buddha." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::mandelbrot() }
    fn render_mode(&self) -> FractalRenderMode { FractalRenderMode::Accumulation }

    fn render_accumulation(&self, width: u32, height: u32, config: &FractalConfig) -> Vec<f64> {
        render_orbit_accumulation(width, height, config, 250_000, 30, config.iterations.max(200))
    }
}

/// Helper for accumulation fractals (Buddhabrot, Nebulabrot)
fn render_orbit_accumulation(
    width: u32,
    height: u32,
    config: &FractalConfig,
    sample_count: usize,
    min_iter: u32,
    max_iter: u32,
) -> Vec<f64> {
    let mut buffer = vec![0.0f64; (width * height) as usize];
    let mut rng = SmallRng::seed_from_u64(42);
    let scale = 4.0 / config.zoom;
    let h = height as f64;

    for _ in 0..sample_count {
        let cr = rng.gen_range(-2.5..1.5);
        let ci = rng.gen_range(-2.0..2.0);
        let c = Complex64::new(cr, ci);

        let mut z = Complex64::zero();
        let mut orbit = Vec::with_capacity(max_iter as usize);
        let mut escapes = false;

        for _ in 0..max_iter {
            z = z.sqr() + c;
            orbit.push(z);
            if z.norm_sqr() > 4.0 {
                escapes = true;
                break;
            }
        }

        if escapes && orbit.len() >= min_iter as usize {
            for pt in orbit {
                // Map complex point (pt.re, pt.im) back to screen pixel
                // x = centered_x * scale * x_scale + offset_x
                // centered_x = (x - offset_x) / (scale * x_scale)
                let centered_x = (pt.re - config.offset_x) / (scale * config.x_scale);
                let centered_y = (pt.im - config.offset_y) / (scale * config.y_scale);

                let px = (centered_x * h + (width as f64 * 0.5)).round() as i32;
                let py = ((h * 0.5) - centered_y * h).round() as i32;

                if px >= 0 && px < width as i32 && py >= 0 && py < height as i32 {
                    let idx = (py as u32 * width + px as u32) as usize;
                    buffer[idx] += 1.0;
                }
            }
        }
    }
    buffer
}
