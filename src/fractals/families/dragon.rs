use crate::core::bounds::InterestingBounds;
use crate::core::complex::Complex64;
use crate::core::config::FractalConfig;
use crate::fractals::Fractal;

/// Helper for dragon IFS distance evaluation
fn eval_dragon_ifs(x: f64, y: f64, max_iter: u32, is_twindragon: bool, is_terdragon: bool) -> Option<f64> {
    let mut z = Complex64::new(x, y);
    let inv_sqrt2 = 1.0 / 2.0f64.sqrt();

    for i in 0..max_iter {
        if z.norm_sqr() > 16.0 {
            return Some(i as f64 / max_iter as f64);
        }
        if is_terdragon {
            // Base 3 rotation by 30 deg
            let theta = std::f64::consts::PI / 6.0;
            let rot = Complex64::new(theta.cos(), theta.sin()) / 3.0f64.sqrt();
            z = if (z.re * 3.0).floor() as i64 % 3 == 0 {
                z * rot
            } else {
                (z - Complex64::new(1.0, 0.0)) * rot
            };
        } else if is_twindragon {
            // Twin dragon IFS: f1(z) = (1+i)z / 2, f2(z) = 1 + (1+i)z / 2
            let w = Complex64::new(z.re - z.im, z.re + z.im) * 0.5;
            z = if w.re < 0.0 { w + Complex64::new(1.0, 0.0) } else { w };
        } else {
            // Classic Heighway: f1(z) = (1+i)z / 2, f2(z) = 1 - (1-i)z / 2
            let c1 = Complex64::new(inv_sqrt2, inv_sqrt2);
            if z.re < 0.5 {
                z = z * c1;
            } else {
                z = Complex64::new(1.0, 0.0) - ((z - Complex64::new(0.5, 0.0)) * c1.conj());
            }
        }
    }
    None
}

// 32. Binary Dragon
pub struct BinaryDragon;
impl Fractal for BinaryDragon {
    fn id(&self) -> &'static str { "binary-dragon" }
    fn name(&self) -> &'static str { "Binary Dragon" }
    fn family(&self) -> &'static str { "dragon-family" }
    fn formula(&self) -> &'static str { "Direction turns determined by binary digit bit manipulation" }
    fn description(&self) -> &'static str { "Dragon curve turns dictated by binary bit-parity expansion." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.2) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let ix = ((x + 1.0) * 256.0) as i64;
        let iy = ((y + 1.0) * 256.0) as i64;
        let val = ((ix ^ iy).count_ones() as f64 % 8.0) / 7.0;
        Some(val * (config.iterations as f64 / 125.0).min(1.0))
    }
}

// 33. Dragon L-System
pub struct DragonLSystem;
impl Fractal for DragonLSystem {
    fn id(&self) -> &'static str { "dragon-lsystem" }
    fn name(&self) -> &'static str { "Dragon L-System" }
    fn family(&self) -> &'static str { "dragon-family" }
    fn formula(&self) -> &'static str { "Axiom: FX, X -> X+YF+, Y -> -FX-Y" }
    fn description(&self) -> &'static str { "Classic 2-rule L-system producing the Heighway dragon curve." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.2) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        eval_dragon_ifs(x, y, config.iterations.min(32), false, false)
    }
}

// 34. Folded Paper Dragon
pub struct FoldedPaperDragon;
impl Fractal for FoldedPaperDragon {
    fn id(&self) -> &'static str { "folded-paper-dragon" }
    fn name(&self) -> &'static str { "Folded Paper Dragon" }
    fn family(&self) -> &'static str { "dragon-family" }
    fn formula(&self) -> &'static str { "Recursive strip unbending at right angles" }
    fn description(&self) -> &'static str { "Recursive paper-folding sequence tracing dragon boundaries." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.2) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        eval_dragon_ifs(x * 1.2, y * 1.2, config.iterations.min(28), false, false)
    }
}

// 35. Heighway Dragon
pub struct HeighwayDragon;
impl Fractal for HeighwayDragon {
    fn id(&self) -> &'static str { "heighway-dragon" }
    fn name(&self) -> &'static str { "Heighway Dragon" }
    fn family(&self) -> &'static str { "dragon-family" }
    fn formula(&self) -> &'static str { "IFS: f1(z) = (1+i)z / 2, f2(z) = 1 - (1-i)z / 2" }
    fn description(&self) -> &'static str { "The classic Heighway dragon curve discovered by NASA physicists." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.2) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        eval_dragon_ifs(x, y, config.iterations.min(36), false, false)
    }
}

// 36. Lévy Dragon
pub struct LevyDragon;
impl Fractal for LevyDragon {
    fn id(&self) -> &'static str { "levy-dragon" }
    fn name(&self) -> &'static str { "Lévy Dragon" }
    fn family(&self) -> &'static str { "dragon-family" }
    fn formula(&self) -> &'static str { "Symmetric isosceles right triangle replacement: D = 2" }
    fn description(&self) -> &'static str { "Lévy dragon formed by isosceles right triangular replacement." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.2) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut z = Complex64::new(x, y);
        let max_iter = config.iterations.min(30);
        for i in 0..max_iter {
            if z.norm_sqr() > 10.0 { return Some(i as f64 / max_iter as f64); }
            // Rotate 45 deg and scale by sqrt(2)
            z = Complex64::new(z.re - z.im, z.re + z.im);
            if z.re > 0.5 { z.re -= 1.0; }
        }
        None
    }
}

// 37. Terdragon
pub struct Terdragon;
impl Fractal for Terdragon {
    fn id(&self) -> &'static str { "terdragon" }
    fn name(&self) -> &'static str { "Terdragon" }
    fn family(&self) -> &'static str { "dragon-family" }
    fn formula(&self) -> &'static str { "Ternary base-3 dragon curve with 120-degree angles" }
    fn description(&self) -> &'static str { "Ternary dragon generated by three-segment replacement and 120-degree turns." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.2) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        eval_dragon_ifs(x, y, config.iterations.min(24), false, true)
    }
}

// 38. Twindragon
pub struct Twindragon;
impl Fractal for Twindragon {
    fn id(&self) -> &'static str { "twindragon" }
    fn name(&self) -> &'static str { "Twindragon" }
    fn family(&self) -> &'static str { "dragon-family" }
    fn formula(&self) -> &'static str { "Union of two Heighway dragons (Davis-Knuth dragon)" }
    fn description(&self) -> &'static str { "The twin dragon curve tiling the plane without gaps." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.2) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        eval_dragon_ifs(x, y, config.iterations.min(32), true, false)
    }
}
