use crate::core::bounds::InterestingBounds;
use crate::core::config::FractalConfig;
use crate::fractals::{Fractal, FractalRenderMode};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

// 90. Diffusion-Limited Aggregation
pub struct DiffusionLimitedAggregation;
impl Fractal for DiffusionLimitedAggregation {
    fn id(&self) -> &'static str { "diffusion-limited-aggregation" }
    fn name(&self) -> &'static str { "Diffusion-Limited Aggregation" }
    fn family(&self) -> &'static str { "physics-family" }
    fn formula(&self) -> &'static str { "Witten–Sander Brownian particle aggregation: D ≈ 1.71" }
    fn description(&self) -> &'static str { "Fractal dendrites formed by random-walking particles sticking to a central cluster." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.2) }
    fn render_mode(&self) -> FractalRenderMode { FractalRenderMode::Accumulation }

    fn render_accumulation(&self, width: u32, height: u32, _config: &FractalConfig) -> Vec<f64> {
        let mut grid = vec![0.0f64; (width * height) as usize];
        let cx = width / 2;
        let cy = height / 2;
        grid[(cy * width + cx) as usize] = 1.0;

        let mut rng = SmallRng::seed_from_u64(101);
        let particles = 15_000;
        let max_r = (width.min(height) / 2) as i32 - 10;

        for _ in 0..particles {
            let angle: f64 = rng.gen_range(0.0..std::f64::consts::TAU);
            let start_r = (max_r as f64 * 0.85).max(10.0);
            let mut px = cx as i32 + (start_r * angle.cos()) as i32;
            let mut py = cy as i32 + (start_r * angle.sin()) as i32;

            for _ in 0..600 {
                let dx: i32 = rng.gen_range(-1..=1);
                let dy: i32 = rng.gen_range(-1..=1);
                px += dx;
                py += dy;

                if px <= 1 || px >= width as i32 - 2 || py <= 1 || py >= height as i32 - 2 {
                    break;
                }

                // Check neighboring stuck particles
                let mut stuck = false;
                for ox in -1..=1 {
                    for oy in -1..=1 {
                        if ox == 0 && oy == 0 { continue; }
                        let n_idx = ((py + oy) as u32 * width + (px + ox) as u32) as usize;
                        if grid[n_idx] > 0.0 {
                            stuck = true;
                            break;
                        }
                    }
                    if stuck { break; }
                }

                if stuck {
                    let idx = (py as u32 * width + px as u32) as usize;
                    grid[idx] = 1.0;
                    break;
                }
            }
        }
        grid
    }
}

// 91. Percolation Cluster
pub struct PercolationCluster;
impl Fractal for PercolationCluster {
    fn id(&self) -> &'static str { "percolation-cluster" }
    fn name(&self) -> &'static str { "Percolation Cluster" }
    fn family(&self) -> &'static str { "physics-family" }
    fn formula(&self) -> &'static str { "Square lattice site percolation at critical threshold pc ≈ 0.5927" }
    fn description(&self) -> &'static str { "Statistical fractal formed by critical site connectivity transitions." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.2) }

    fn compute(&self, x: f64, y: f64, _config: &FractalConfig) -> Option<f64> {
        let ix = ((x + 2.0) * 128.0) as i64;
        let iy = ((y + 2.0) * 128.0) as i64;
        let h = (ix.wrapping_mul(48271) ^ iy.wrapping_mul(16807)).wrapping_add(997);
        let p = (h & 0xFFFF) as f64 / 65535.0;
        let pc = 0.592746; // 2D square critical percolation threshold
        if p < pc {
            Some(p / pc)
        } else {
            None
        }
    }
}

// 92. Lévy Flights
pub struct LevyFlights;
impl Fractal for LevyFlights {
    fn id(&self) -> &'static str { "levy-flights" }
    fn name(&self) -> &'static str { "Lévy Flights" }
    fn family(&self) -> &'static str { "physics-family" }
    fn formula(&self) -> &'static str { "Power-law step distribution: P(L) ~ L^(-α), 0 < α < 2" }
    fn description(&self) -> &'static str { "Superdiffusive flight path exhibiting clusters connected by long trajectories." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(2.0, 1.2) }
    fn render_mode(&self) -> FractalRenderMode { FractalRenderMode::Accumulation }

    fn render_accumulation(&self, width: u32, height: u32, config: &FractalConfig) -> Vec<f64> {
        let mut buffer = vec![0.0f64; (width * height) as usize];
        let mut rng = SmallRng::seed_from_u64(2026);
        let mut x = 0.0f64;
        let mut y = 0.0f64;
        let steps = 60_000;
        let scale = 4.0 / config.zoom;
        let h = height as f64;

        for _ in 0..steps {
            let u: f64 = rng.gen_range(0.001..1.0);
            let step_len = 0.02 * (u.powf(-1.0 / 1.5)); // Pareto alpha = 1.5
            let angle: f64 = rng.gen_range(0.0..std::f64::consts::TAU);
            x += step_len * angle.cos();
            y += step_len * angle.sin();

            // Wrap around
            if x.abs() > 3.0 { x = -x * 0.5; }
            if y.abs() > 3.0 { y = -y * 0.5; }

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
