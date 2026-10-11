use crate::core::bounds::InterestingBounds;
use crate::core::config::FractalConfig;
use crate::fractals::{Fractal, FractalRenderMode};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

// 79. Lorenz Attractor
pub struct LorenzAttractor;
impl Fractal for LorenzAttractor {
    fn id(&self) -> &'static str { "lorenz-attractor" }
    fn name(&self) -> &'static str { "Lorenz Attractor" }
    fn family(&self) -> &'static str { "attractor-family" }
    fn formula(&self) -> &'static str { "dx/dt = σ(y - x), dy/dt = x(ρ - z) - y, dz/dt = xy - βz" }
    fn description(&self) -> &'static str { "Edward Lorenz's atmospheric convection strange attractor." }
    fn bounds(&self) -> InterestingBounds {
        InterestingBounds {
            default_offset_x: 0.0,
            default_offset_y: 25.0,
            default_zoom: 0.12,
            ..InterestingBounds::centered(30.0, 0.12)
        }
    }
    fn render_mode(&self) -> FractalRenderMode { FractalRenderMode::Accumulation }

    fn render_accumulation(&self, width: u32, height: u32, config: &FractalConfig) -> Vec<f64> {
        let mut buffer = vec![0.0f64; (width * height) as usize];
        let mut x = 0.1;
        let mut y = 0.0;
        let mut z = 0.0;
        let sigma = 10.0;
        let rho = 28.0;
        let beta = 8.0 / 3.0;
        let dt = 0.005;
        let steps = 300_000;
        let scale = 4.0 / config.zoom;
        let h = height as f64;

        for i in 0..steps {
            let dx = sigma * (y - x);
            let dy = x * (rho - z) - y;
            let dz = x * y - beta * z;
            x += dx * dt;
            y += dy * dt;
            z += dz * dt;

            if i > 1000 {
                // Project (x, z)
                let centered_x = (x - config.offset_x) / (scale * config.x_scale);
                let centered_y = (z - config.offset_y) / (scale * config.y_scale);
                let px = (centered_x * h + (width as f64 * 0.5)).round() as i32;
                let py = ((h * 0.5) - centered_y * h).round() as i32;

                if px >= 0 && px < width as i32 && py >= 0 && py < height as i32 {
                    buffer[(py as u32 * width + px as u32) as usize] += 1.0;
                }
            }
        }
        buffer
    }
}

// 80. Rössler Attractor
pub struct RosslerAttractor;
impl Fractal for RosslerAttractor {
    fn id(&self) -> &'static str { "rossler-attractor" }
    fn name(&self) -> &'static str { "Rössler Attractor" }
    fn family(&self) -> &'static str { "attractor-family" }
    fn formula(&self) -> &'static str { "dx/dt = -y - z, dy/dt = x + ay, dz/dt = b + z(x - c)" }
    fn description(&self) -> &'static str { "Otto Rössler's simplified continuous chaotic band attractor." }
    fn bounds(&self) -> InterestingBounds {
        InterestingBounds {
            default_offset_x: 0.0,
            default_offset_y: 0.0,
            default_zoom: 0.25,
            ..InterestingBounds::centered(15.0, 0.25)
        }
    }
    fn render_mode(&self) -> FractalRenderMode { FractalRenderMode::Accumulation }

    fn render_accumulation(&self, width: u32, height: u32, config: &FractalConfig) -> Vec<f64> {
        let mut buffer = vec![0.0f64; (width * height) as usize];
        let mut x = 0.1;
        let mut y = 0.0;
        let mut z = 0.0;
        let a = 0.2;
        let b = 0.2;
        let c = 5.7;
        let dt = 0.01;
        let steps = 300_000;
        let scale = 4.0 / config.zoom;
        let h = height as f64;

        for i in 0..steps {
            let dx = -y - z;
            let dy = x + a * y;
            let dz = b + z * (x - c);
            x += dx * dt;
            y += dy * dt;
            z += dz * dt;

            if i > 1000 {
                let centered_x = (x - config.offset_x) / (scale * config.x_scale);
                let centered_y = (y - config.offset_y) / (scale * config.y_scale);
                let px = (centered_x * h + (width as f64 * 0.5)).round() as i32;
                let py = ((h * 0.5) - centered_y * h).round() as i32;

                if px >= 0 && px < width as i32 && py >= 0 && py < height as i32 {
                    buffer[(py as u32 * width + px as u32) as usize] += 1.0;
                }
            }
        }
        buffer
    }
}

// 81. Lyapunov Fractal
pub struct LyapunovFractal;
impl Fractal for LyapunovFractal {
    fn id(&self) -> &'static str { "lyapunov" }
    fn name(&self) -> &'static str { "Lyapunov Fractal" }
    fn family(&self) -> &'static str { "attractor-family" }
    fn formula(&self) -> &'static str { "λ = lim (1/N) * Σ ln|r_n * (1 - 2x_n)| for AB sequence" }
    fn description(&self) -> &'static str { "Lyapunov exponent stability map evaluating chaos transitions of population growth." }
    fn bounds(&self) -> InterestingBounds {
        InterestingBounds {
            min_offset_x: 2.0, max_offset_x: 4.0,
            min_offset_y: 2.0, max_offset_y: 4.0,
            default_offset_x: 3.85, default_offset_y: 3.85,
            default_zoom: 1.5,
            ..InterestingBounds::default()
        }
    }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let a = x;
        let b = y;
        let mut population = 0.5;
        let max_iter = config.iterations.min(100);
        let seq = [true, false]; // AB string

        // Warm up
        for i in 0..30 {
            let r = if seq[i % 2] { a } else { b };
            population = r * population * (1.0 - population);
        }

        let mut lambda_sum = 0.0;
        for i in 0..max_iter {
            let r = if seq[i as usize % 2] { a } else { b };
            let deriv = (r * (1.0 - 2.0 * population)).abs();
            if deriv > 1e-12 {
                lambda_sum += deriv.ln();
            }
            population = r * population * (1.0 - population);
        }

        let lambda = lambda_sum / max_iter as f64;
        if lambda < 0.0 {
            // Stable periodic orbit
            let t = (-lambda * 0.5).clamp(0.0, 1.0);
            Some(t)
        } else {
            // Chaotic orbit
            None
        }
    }
}

// 82. Chua Attractor
pub struct ChuaAttractor;
impl Fractal for ChuaAttractor {
    fn id(&self) -> &'static str { "chua-attractor" }
    fn name(&self) -> &'static str { "Chua Attractor" }
    fn family(&self) -> &'static str { "attractor-family" }
    fn formula(&self) -> &'static str { "dx/dt = α(y - x - f(x)), dy/dt = x - y + z, dz/dt = -βy" }
    fn description(&self) -> &'static str { "Leon Chua's double-scroll electronic oscillator chaotic attractor." }
    fn bounds(&self) -> InterestingBounds {
        InterestingBounds {
            default_offset_x: 0.0,
            default_offset_y: 0.0,
            default_zoom: 0.5,
            ..InterestingBounds::centered(5.0, 0.5)
        }
    }
    fn render_mode(&self) -> FractalRenderMode { FractalRenderMode::Accumulation }

    fn render_accumulation(&self, width: u32, height: u32, config: &FractalConfig) -> Vec<f64> {
        let mut buffer = vec![0.0f64; (width * height) as usize];
        let mut x = 0.7;
        let mut y = 0.0;
        let mut z = 0.0;
        let alpha = 15.6;
        let beta = 28.0;
        let m0 = -1.143;
        let m1 = -0.714;
        let dt = 0.005;
        let steps = 300_000;
        let scale = 4.0 / config.zoom;
        let h = height as f64;

        for i in 0..steps {
            let fx = m1 * x + 0.5 * (m0 - m1) * ((x + 1.0f64).abs() - (x - 1.0f64).abs());
            let dx = alpha * (y - x - fx);
            let dy = x - y + z;
            let dz = -beta * y;
            x += dx * dt;
            y += dy * dt;
            z += dz * dt;

            if i > 1000 {
                let centered_x = (x - config.offset_x) / (scale * config.x_scale);
                let centered_y = (y - config.offset_y) / (scale * config.y_scale);
                let px = (centered_x * h + (width as f64 * 0.5)).round() as i32;
                let py = ((h * 0.5) - centered_y * h).round() as i32;

                if px >= 0 && px < width as i32 && py >= 0 && py < height as i32 {
                    buffer[(py as u32 * width + px as u32) as usize] += 1.0;
                }
            }
        }
        buffer
    }
}

// 83. IFS Spiral
pub struct IfsSpiral;
impl Fractal for IfsSpiral {
    fn id(&self) -> &'static str { "ifs-spiral" }
    fn name(&self) -> &'static str { "IFS Spiral" }
    fn family(&self) -> &'static str { "attractor-family" }
    fn formula(&self) -> &'static str { "Logarithmic contractive spiral affine IFS" }
    fn description(&self) -> &'static str { "Iterated function system creating curling spiral arms." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.2) }
    fn render_mode(&self) -> FractalRenderMode { FractalRenderMode::Accumulation }

    fn render_accumulation(&self, width: u32, height: u32, config: &FractalConfig) -> Vec<f64> {
        let mut buffer = vec![0.0f64; (width * height) as usize];
        let mut rng = SmallRng::seed_from_u64(999);
        let mut x = 0.0f64;
        let mut y = 0.0f64;
        let scale = 4.0 / config.zoom;
        let h = height as f64;

        for _ in 0..250_000 {
            let r: f64 = rng.gen_range(0.0..1.0);
            let (nx, ny) = if r < 0.85 {
                let angle = 0.15f64;
                let s = 0.96f64;
                (
                    s * (x * angle.cos() - y * angle.sin()),
                    s * (x * angle.sin() + y * angle.cos()) + 0.1,
                )
            } else {
                (-0.4 * x, 0.4 * y + 0.2)
            };
            x = nx;
            y = ny;

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

// 84. IFS Maple
pub struct IfsMaple;
impl Fractal for IfsMaple {
    fn id(&self) -> &'static str { "ifs-maple" }
    fn name(&self) -> &'static str { "IFS Maple" }
    fn family(&self) -> &'static str { "attractor-family" }
    fn formula(&self) -> &'static str { "Affine IFS maple leaf attractor" }
    fn description(&self) -> &'static str { "Botanical maple leaf generated by 4-transformation affine contractive mappings." }
    fn bounds(&self) -> InterestingBounds {
        InterestingBounds {
            default_offset_x: 0.0, default_offset_y: 1.0,
            ..InterestingBounds::centered(2.0, 1.2)
        }
    }
    fn render_mode(&self) -> FractalRenderMode { FractalRenderMode::Accumulation }

    fn render_accumulation(&self, width: u32, height: u32, config: &FractalConfig) -> Vec<f64> {
        let mut buffer = vec![0.0f64; (width * height) as usize];
        let mut rng = SmallRng::seed_from_u64(888);
        let mut x = 0.0f64;
        let mut y = 0.0f64;
        let scale = 4.0 / config.zoom;
        let h = height as f64;

        for _ in 0..250_000 {
            let r: f64 = rng.gen_range(0.0..1.0);
            let (nx, ny) = if r < 0.1 {
                (0.14 * x + 0.01 * y, -0.01 * x + 0.51 * y - 1.31)
            } else if r < 0.45 {
                (0.43 * x + 0.52 * y + 1.49, -0.45 * x + 0.5 * y - 0.75)
            } else if r < 0.8 {
                (0.45 * x - 0.49 * y - 1.62, 0.47 * x + 0.47 * y - 0.74)
            } else {
                (0.49 * x + 0.0 * y + 0.02, 0.0 * x + 0.51 * y + 1.62)
            };
            x = nx;
            y = ny;

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

// 85. IFS Tree
pub struct IfsTree;
impl Fractal for IfsTree {
    fn id(&self) -> &'static str { "ifs-tree" }
    fn name(&self) -> &'static str { "IFS Tree" }
    fn family(&self) -> &'static str { "attractor-family" }
    fn formula(&self) -> &'static str { "Contractive affine transformation tree" }
    fn description(&self) -> &'static str { "Iterated function system tree with dense twig attractors." }
    fn bounds(&self) -> InterestingBounds {
        InterestingBounds {
            default_offset_x: 0.0, default_offset_y: 2.0,
            ..InterestingBounds::centered(2.0, 1.2)
        }
    }
    fn render_mode(&self) -> FractalRenderMode { FractalRenderMode::Accumulation }

    fn render_accumulation(&self, width: u32, height: u32, config: &FractalConfig) -> Vec<f64> {
        let mut buffer = vec![0.0f64; (width * height) as usize];
        let mut rng = SmallRng::seed_from_u64(777);
        let mut x = 0.0f64;
        let mut y = 0.0f64;
        let scale = 4.0 / config.zoom;
        let h = height as f64;

        for _ in 0..250_000 {
            let r: f64 = rng.gen_range(0.0..1.0);
            let (nx, ny) = if r < 0.05 {
                (0.0, 0.5 * y)
            } else if r < 0.52 {
                (0.42 * x - 0.42 * y, 0.42 * x + 0.42 * y + 1.0)
            } else {
                (0.42 * x + 0.42 * y, -0.42 * x + 0.42 * y + 1.0)
            };
            x = nx;
            y = ny;

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
