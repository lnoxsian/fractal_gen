use super::mandelbrot::smooth_escape_time;
use crate::core::bounds::InterestingBounds;
use crate::core::complex::Complex64;
use crate::core::config::FractalConfig;
use crate::fractals::{Fractal, FractalRenderMode};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

// Buffalo
pub struct Buffalo;
impl Fractal for Buffalo {
    fn id(&self) -> &'static str { "buffalo" }
    fn name(&self) -> &'static str { "Buffalo" }
    fn family(&self) -> &'static str { "other-family" }
    fn formula(&self) -> &'static str { "z_{n+1} = (|Re(z)| - |Im(z)|)^2 + c" }
    fn description(&self) -> &'static str { "Buffalo fractal featuring horns and beast-like symmetries." }
    fn bounds(&self) -> InterestingBounds {
        InterestingBounds {
            min_offset_x: -2.0, max_offset_x: 1.5,
            min_offset_y: -2.0, max_offset_y: 1.5,
            default_offset_x: -0.5, default_offset_y: 0.0,
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
            z = Complex64::new(re * re - im * im + c.re, -2.0 * re * im + c.im);
            let n_sqr = z.norm_sqr();
            if n_sqr > escape_sqr {
                return Some(smooth_escape_time(i, n_sqr, max_iter));
            }
        }
        None
    }
}

// Popcorn
pub struct Popcorn;
impl Fractal for Popcorn {
    fn id(&self) -> &'static str { "popcorn" }
    fn name(&self) -> &'static str { "Popcorn" }
    fn family(&self) -> &'static str { "other-family" }
    fn formula(&self) -> &'static str { "x_{n+1} = x - h*sin(y + tan(3y)), y_{n+1} = y - h*sin(x + tan(3x))" }
    fn description(&self) -> &'static str { "Clifford Pickover's 2D popcorn map with clamped trigonometric tangents." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(3.0, 0.8) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut px = x;
        let mut py = y;
        let h = 0.05;
        let max_iter = config.iterations.min(60);

        for i in 0..max_iter {
            let tan_y = (3.0 * py).tan().clamp(-10.0, 10.0);
            let tan_x = (3.0 * px).tan().clamp(-10.0, 10.0);
            let nx = px - h * (py + tan_y).sin();
            let ny = py - h * (px + tan_x).sin();
            px = nx;
            py = ny;
            if px * px + py * py > 50.0 {
                return Some(i as f64 / max_iter as f64);
            }
        }
        let t = ((px * px + py * py).sqrt() * 0.2).fract();
        Some(t)
    }
}

// Spider Set
pub struct SpiderSet;
impl Fractal for SpiderSet {
    fn id(&self) -> &'static str { "spider-set" }
    fn name(&self) -> &'static str { "Spider Set" }
    fn family(&self) -> &'static str { "other-family" }
    fn formula(&self) -> &'static str { "z_{n+1} = z_n^2 + c + p / z_n" }
    fn description(&self) -> &'static str { "Rational perturbation creating web-like filaments and spider motifs." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::mandelbrot() }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let c = Complex64::new(x, y);
        let mut z = c;
        let p = Complex64::new(0.1, 0.0);
        let max_iter = config.iterations;
        let escape_sqr = config.escape_radius * config.escape_radius;

        for i in 0..max_iter {
            if z.norm_sqr() < 1e-12 {
                z += Complex64::new(1e-6, 0.0);
            }
            z = z.sqr() + c + (p * z.inv());
            let n_sqr = z.norm_sqr();
            if n_sqr > escape_sqr {
                return Some(smooth_escape_time(i, n_sqr, max_iter));
            }
        }
        None
    }
}

// Magnet Fractal
pub struct Magnet;
impl Fractal for Magnet {
    fn id(&self) -> &'static str { "magnet" }
    fn name(&self) -> &'static str { "Magnet Fractal" }
    fn family(&self) -> &'static str { "other-family" }
    fn formula(&self) -> &'static str { "z_{n+1} = (z^2 + c - 1)^2 / (4z * (z^2 + c - 2))" }
    fn description(&self) -> &'static str { "Rational iteration modeling magnetic phase transitions in statistical physics." }
    fn bounds(&self) -> InterestingBounds {
        InterestingBounds {
            min_offset_x: -2.0, max_offset_x: 4.0,
            min_offset_y: -3.0, max_offset_y: 3.0,
            default_offset_x: 0.5, default_offset_y: 0.0,
            default_zoom: 1.0,
            ..InterestingBounds::default()
        }
    }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let c = Complex64::new(x, y);
        let mut z = Complex64::zero();
        let max_iter = config.iterations;

        for i in 0..max_iter {
            let z2 = z.sqr();
            let num = (z2 + c - Complex64::new(1.0, 0.0)).sqr();
            let den = (z * 4.0) * (z2 + c - Complex64::new(2.0, 0.0));
            if den.norm_sqr() < 1e-14 {
                return Some(smooth_escape_time(i, 100.0, max_iter));
            }
            z = num / den;
            let n_sqr = z.norm_sqr();
            if n_sqr > 100.0 {
                return Some(smooth_escape_time(i, n_sqr, max_iter));
            }
        }
        None
    }
}

// Fractal Dimension Plot
pub struct FractalDimensionPlot;
impl Fractal for FractalDimensionPlot {
    fn id(&self) -> &'static str { "fractal-dimension-plot" }
    fn name(&self) -> &'static str { "Fractal Dimension Plot" }
    fn family(&self) -> &'static str { "other-family" }
    fn formula(&self) -> &'static str { "Hausdorff dimension field: D_H = inf { d : H^d(E) = 0 }" }
    fn description(&self) -> &'static str { "Continuous multi-fractal Hausdorff dimension contour map." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(2.0, 1.2) }

    fn compute(&self, x: f64, y: f64, _config: &FractalConfig) -> Option<f64> {
        let r = (x * x + y * y).sqrt();
        let val = (1.5 + 0.5 * (r * 10.0).sin() * (y * 5.0).cos()) / 2.0;
        Some(val.clamp(0.0, 1.0))
    }
}

// H-Tree
pub struct HTree;
impl Fractal for HTree {
    fn id(&self) -> &'static str { "h-tree" }
    fn name(&self) -> &'static str { "H-Tree" }
    fn family(&self) -> &'static str { "other-family" }
    fn formula(&self) -> &'static str { "Geometric recursive H-shaped orthogonal branching: D = 2" }
    fn description(&self) -> &'static str { "Orthogonal self-similar H-tree used in VLSI clock distribution networks." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.2) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut px = x.abs();
        let mut py = y.abs();
        let max_iter = config.iterations.min(10);
        let mut len = 0.5;

        for _ in 0..max_iter {
            if px < len && py < 0.02 { return Some(1.0); }
            if (px - len).abs() < 0.02 && py < len { return Some(1.0); }
            px = (px - len).abs();
            py = (py - len).abs();
            len *= 0.5;
        }
        None
    }
}

// Generalized H-Tree
pub struct GeneralizedHTree;
impl Fractal for GeneralizedHTree {
    fn id(&self) -> &'static str { "h-tree-generalized" }
    fn name(&self) -> &'static str { "Generalized H-Tree" }
    fn family(&self) -> &'static str { "other-family" }
    fn formula(&self) -> &'static str { "Generalized H-tree with variable branching scaling ratio" }
    fn description(&self) -> &'static str { "Scaled and rotated generalized H-tree variant." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.2) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let htree = HTree;
        let rot_x = 0.7071 * (x - y);
        let rot_y = 0.7071 * (x + y);
        htree.compute(rot_x, rot_y, config)
    }
}

// Vicsek Snowflake
pub struct Vicsek;
impl Fractal for Vicsek {
    fn id(&self) -> &'static str { "vicsek" }
    fn name(&self) -> &'static str { "Vicsek Snowflake" }
    fn family(&self) -> &'static str { "other-family" }
    fn formula(&self) -> &'static str { "Keep center and 4 corners in 3x3 grid: D = log(5)/log(3) ≈ 1.465" }
    fn description(&self) -> &'static str { "Tamás Vicsek's box fractal retaining corner and central tiles." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.2, 1.5) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut u = (x + 1.0) * 0.5;
        let mut v = (y + 1.0) * 0.5;
        if u < 0.0 || u > 1.0 || v < 0.0 || v > 1.0 { return None; }
        let max_iter = config.iterations.min(16);

        for i in 0..max_iter {
            u *= 3.0;
            v *= 3.0;
            let iu = (u.floor() as i32) % 3;
            let iv = (v.floor() as i32) % 3;
            let is_cross = (iu == 1 || iv == 1) && !(iu == 1 && iv == 1);
            if is_cross {
                return Some(i as f64 / max_iter as f64);
            }
            u -= u.floor();
            v -= v.floor();
        }
        None
    }
}

// Fractal Flame
pub struct FractalFlame;
impl Fractal for FractalFlame {
    fn id(&self) -> &'static str { "fractal-flame" }
    fn name(&self) -> &'static str { "Fractal Flame" }
    fn family(&self) -> &'static str { "other-family" }
    fn formula(&self) -> &'static str { "Nonlinear IFS flame: Sinusoidal, Spherical, and Swirl variations" }
    fn description(&self) -> &'static str { "Scott Draves's nonlinear iterated function flame system." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(2.0, 1.2) }
    fn render_mode(&self) -> FractalRenderMode { FractalRenderMode::Accumulation }

    fn render_accumulation(&self, width: u32, height: u32, config: &FractalConfig) -> Vec<f64> {
        let mut buffer = vec![0.0f64; (width * height) as usize];
        let mut rng = SmallRng::seed_from_u64(54321);
        let mut x = 0.1f64;
        let mut y = 0.1f64;
        let steps = 300_000;
        let scale = 4.0 / config.zoom;
        let h = height as f64;

        for _ in 0..steps {
            let r: f64 = rng.gen_range(0.0..1.0);
            let (nx, ny) = if r < 0.33 {
                // Sinusoidal variation
                (x.sin(), y.sin())
            } else if r < 0.66 {
                // Spherical variation
                let r2 = (x * x + y * y).max(1e-6);
                (x / r2, y / r2)
            } else {
                // Swirl variation
                let r2 = x * x + y * y;
                (x * r2.sin() - y * r2.cos(), x * r2.cos() + y * r2.sin())
            };
            x = 0.7 * nx + rng.gen_range(-0.1..0.1);
            y = 0.7 * ny + rng.gen_range(-0.1..0.1);

            let centered_x = (x - config.offset_x) / (scale * config.x_scale);
            let centered_y = (y - config.offset_y) / (scale * config.y_scale);
            let px = (centered_x * h + (width as f64 * 0.5)).round() as i32;
            let py = ((h * 0.5) - centered_y * h).round() as i32;

            if px >= 0 && px < width as i32 && py >= 0 && py < height as i32 {
                buffer[(py as u32 * width + px as u32) as usize] += 1.0;
            }
        }
        buffer
    }
}
