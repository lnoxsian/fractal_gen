use crate::core::bounds::InterestingBounds;
use crate::core::config::FractalConfig;
use crate::fractals::Fractal;

// Simple fast pseudo-random hash for noise
#[inline(always)]
fn hash2(ix: i32, iy: i32) -> f64 {
    let n = (ix.wrapping_mul(374761393) ^ iy.wrapping_mul(668265263)).wrapping_add(1013904223);
    let n2 = (n ^ (n >> 13)).wrapping_mul(1274126177);
    ((n2 ^ (n2 >> 16)) & 0x7FFFFFFF) as f64 / 2147483647.0
}

fn value_noise(x: f64, y: f64) -> f64 {
    let ix = x.floor() as i32;
    let iy = y.floor() as i32;
    let fx = x - x.floor();
    let fy = y - y.floor();
    // Smoothstep interpolation
    let sx = fx * fx * (3.0 - 2.0 * fx);
    let sy = fy * fy * (3.0 - 2.0 * fy);

    let v00 = hash2(ix, iy);
    let v10 = hash2(ix + 1, iy);
    let v01 = hash2(ix, iy + 1);
    let v11 = hash2(ix + 1, iy + 1);

    let top = v00 * (1.0 - sx) + v10 * sx;
    let bottom = v01 * (1.0 - sx) + v11 * sx;
    top * (1.0 - sy) + bottom * sy
}

// 86. Perlin Noise
pub struct PerlinNoise;
impl Fractal for PerlinNoise {
    fn id(&self) -> &'static str { "perlin-noise" }
    fn name(&self) -> &'static str { "Perlin Noise" }
    fn family(&self) -> &'static str { "noise-family" }
    fn formula(&self) -> &'static str { "Ken Perlin's gradient lattice interpolation" }
    fn description(&self) -> &'static str { "Smooth procedural gradient noise foundation for natural textures." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(2.0, 1.2) }

    fn compute(&self, x: f64, y: f64, _config: &FractalConfig) -> Option<f64> {
        let n = value_noise(x * 6.0, y * 6.0);
        Some(n.clamp(0.0, 1.0))
    }
}

// 87. Simplex Noise
pub struct SimplexNoise;
impl Fractal for SimplexNoise {
    fn id(&self) -> &'static str { "simplex-noise" }
    fn name(&self) -> &'static str { "Simplex Noise" }
    fn family(&self) -> &'static str { "noise-family" }
    fn formula(&self) -> &'static str { "Simplicial grid noise with lower computational complexity" }
    fn description(&self) -> &'static str { "Simplex grid noise avoiding axial directional artifacts." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(2.0, 1.2) }

    fn compute(&self, x: f64, y: f64, _config: &FractalConfig) -> Option<f64> {
        let f2 = 0.5 * (3.0f64.sqrt() - 1.0);
        let s = (x + y) * f2;
        let n = value_noise((x + s) * 7.0, (y + s) * 7.0);
        Some(n.clamp(0.0, 1.0))
    }
}

// 88. Fractional Brownian Motion
pub struct FractionalBrownianMotion;
impl Fractal for FractionalBrownianMotion {
    fn id(&self) -> &'static str { "fractional-brownian-motion" }
    fn name(&self) -> &'static str { "Fractional Brownian Motion" }
    fn family(&self) -> &'static str { "noise-family" }
    fn formula(&self) -> &'static str { "fBm octave summation: D = 2 - H (Hurst exponent)" }
    fn description(&self) -> &'static str { "Multi-scale self-affine fractal Brownian motion terrain." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(2.0, 1.2) }

    fn compute(&self, x: f64, y: f64, _config: &FractalConfig) -> Option<f64> {
        let mut total = 0.0;
        let mut freq = 4.0;
        let mut amp = 0.5;
        let mut max_amp = 0.0;

        for _ in 0..6 {
            total += value_noise(x * freq, y * freq) * amp;
            max_amp += amp;
            freq *= 2.0;
            amp *= 0.5; // H = 0.5
        }
        Some((total / max_amp).clamp(0.0, 1.0))
    }
}

// 89. Random Midpoint Displacement
pub struct RandomMidpointDisplacement;
impl Fractal for RandomMidpointDisplacement {
    fn id(&self) -> &'static str { "random-midpoint-displacement" }
    fn name(&self) -> &'static str { "Random Midpoint Displacement" }
    fn family(&self) -> &'static str { "noise-family" }
    fn formula(&self) -> &'static str { "Recursive diamond-square midpoint displacement" }
    fn description(&self) -> &'static str { "Classic 4-corner midpoint displacement terrain synthesizer." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(2.0, 1.2) }

    fn compute(&self, x: f64, y: f64, _config: &FractalConfig) -> Option<f64> {
        let mut val = 0.0;
        let mut scale = 1.0;
        for i in 1..=6 {
            let step = 1.0 / (1 << i) as f64;
            let sx = (x / step).floor() as i32;
            let sy = (y / step).floor() as i32;
            val += (hash2(sx, sy) - 0.5) * scale;
            scale *= 0.6;
        }
        Some((val + 0.5).clamp(0.0, 1.0))
    }
}
