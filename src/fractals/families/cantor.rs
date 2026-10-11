use crate::core::bounds::InterestingBounds;
use crate::core::config::FractalConfig;
use crate::fractals::Fractal;

// 63. Cantor Set
pub struct CantorSet;
impl Fractal for CantorSet {
    fn id(&self) -> &'static str { "cantor" }
    fn name(&self) -> &'static str { "Cantor Set" }
    fn family(&self) -> &'static str { "cantor-family" }
    fn formula(&self) -> &'static str { "Middle-third removal: D = log(2)/log(3) ≈ 0.631" }
    fn description(&self) -> &'static str { "Classic Cantor ternary set repeated along vertical bands." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.2, 1.5) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut u = (x + 1.0) * 0.5;
        if u < 0.0 || u > 1.0 { return Some(0.0); }
        let max_iter = config.iterations.min(16);
        for i in 0..max_iter {
            u *= 3.0;
            let digit = (u.floor() as i64) % 3;
            if digit == 1 {
                return Some(i as f64 / max_iter as f64);
            }
            u -= u.floor();
        }
        let t = (y.abs() * 2.0).clamp(0.0, 1.0);
        Some(t)
    }
}

// 64. Cantor Dust by Base Expansion
pub struct CantorDustBaseExpansion;
impl Fractal for CantorDustBaseExpansion {
    fn id(&self) -> &'static str { "cantor-dust-base-expansion" }
    fn name(&self) -> &'static str { "Cantor Dust by Base Expansion" }
    fn family(&self) -> &'static str { "cantor-family" }
    fn formula(&self) -> &'static str { "2D Cartesian product of two Cantor sets: D = 2 * log(2)/log(3) ≈ 1.262" }
    fn description(&self) -> &'static str { "Totally disconnected 2D fractal dust created by ternary digit filtering." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.2, 1.5) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut u = (x + 1.0) * 0.5;
        let mut v = (y + 1.0) * 0.5;
        if u < 0.0 || u > 1.0 || v < 0.0 || v > 1.0 { return Some(0.0); }
        let max_iter = config.iterations.min(16);
        for i in 0..max_iter {
            u *= 3.0;
            v *= 3.0;
            let du = (u.floor() as i64) % 3;
            let dv = (v.floor() as i64) % 3;
            if du == 1 || dv == 1 {
                return Some(i as f64 / max_iter as f64);
            }
            u -= u.floor();
            v -= v.floor();
        }
        None
    }
}

// 65. Circular Cantor Dust
pub struct CircularCantorDust;
impl Fractal for CircularCantorDust {
    fn id(&self) -> &'static str { "cantor-dust-circular" }
    fn name(&self) -> &'static str { "Circular Cantor Dust" }
    fn family(&self) -> &'static str { "cantor-family" }
    fn formula(&self) -> &'static str { "Concentric radial middle-third removal" }
    fn description(&self) -> &'static str { "Concentric annular Cantor rings." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.2) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let r = (x * x + y * y).sqrt();
        let mut u = r;
        let max_iter = config.iterations.min(16);
        for i in 0..max_iter {
            u *= 3.0;
            let digit = (u.floor() as i64) % 3;
            if digit == 1 {
                return Some(i as f64 / max_iter as f64);
            }
            u -= u.floor();
        }
        None
    }
}

// 66. Fat Cantor Set
pub struct FatCantorSet;
impl Fractal for FatCantorSet {
    fn id(&self) -> &'static str { "fat-cantor" }
    fn name(&self) -> &'static str { "Fat Cantor Set" }
    fn family(&self) -> &'static str { "cantor-family" }
    fn formula(&self) -> &'static str { "Cantor set with decreasing removed intervals preserving positive Lebesgue measure" }
    fn description(&self) -> &'static str { "Nowhere-dense set retaining positive geometric measure." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.2, 1.5) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut u = (x + 1.0) * 0.5;
        if u < 0.0 || u > 1.0 { return Some(0.0); }
        let max_iter = config.iterations.min(16);
        let mut width = 1.0;
        for i in 0..max_iter {
            let removed_fraction = 1.0 / (3.0 * (i + 1) as f64).powi(2);
            let center = width * 0.5;
            let half_rem = width * removed_fraction * 0.5;
            if (u - center).abs() < half_rem {
                return Some(i as f64 / max_iter as f64);
            }
            u = (u * 2.0).fract();
            width *= 0.5;
        }
        Some(y.abs().clamp(0.0, 1.0))
    }
}

// 67. Smith–Volterra–Cantor Set
pub struct SmithVolterraCantorSet;
impl Fractal for SmithVolterraCantorSet {
    fn id(&self) -> &'static str { "smith-volterra-cantor" }
    fn name(&self) -> &'static str { "Smith–Volterra–Cantor Set" }
    fn family(&self) -> &'static str { "cantor-family" }
    fn formula(&self) -> &'static str { "Progressive removal of intervals of length 1/4^n: Measure = 1/2" }
    fn description(&self) -> &'static str { "Classic positive-measure (1/2) Cantor set used in real analysis." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.2, 1.5) }

    fn compute(&self, x: f64, _y: f64, config: &FractalConfig) -> Option<f64> {
        let mut u = (x + 1.0) * 0.5;
        if u < 0.0 || u > 1.0 { return Some(0.0); }
        let max_iter = config.iterations.min(14);
        for i in 0..max_iter {
            let rem_len = 1.0 / 4.0f64.powi((i + 1) as i32);
            if (u - 0.5).abs() < rem_len * 0.5 {
                return Some(i as f64 / max_iter as f64);
            }
            u = if u > 0.5 { (u - 0.5) * 2.0 } else { u * 2.0 };
        }
        None
    }
}

// 68. Random Cantor Set
pub struct RandomCantorSet;
impl Fractal for RandomCantorSet {
    fn id(&self) -> &'static str { "random-cantor" }
    fn name(&self) -> &'static str { "Random Cantor Set" }
    fn family(&self) -> &'static str { "cantor-family" }
    fn formula(&self) -> &'static str { "Stochastically positioned sub-interval removals" }
    fn description(&self) -> &'static str { "Randomized Cantor construction with stochastic seed variations." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.2, 1.5) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let ix = ((x + 2.0) * 1024.0) as u64;
        let iy = ((y + 2.0) * 1024.0) as u64;
        let hash = (ix.wrapping_mul(31415926535) ^ iy.wrapping_mul(27182818284)).rotate_left(13);
        let max_iter = config.iterations.min(16);
        let mut h = hash;
        for i in 0..max_iter {
            if (h % 3) == 1 {
                return Some(i as f64 / max_iter as f64);
            }
            h /= 3;
        }
        None
    }
}
