use crate::core::bounds::InterestingBounds;
use crate::core::config::FractalConfig;
use crate::fractals::Fractal;

const PHI: f64 = 1.618033988749895; // Golden ratio

// 69. Domino Substitution
pub struct DominoSubstitution;
impl Fractal for DominoSubstitution {
    fn id(&self) -> &'static str { "domino-substitution" }
    fn name(&self) -> &'static str { "Domino Substitution" }
    fn family(&self) -> &'static str { "tiling-family" }
    fn formula(&self) -> &'static str { "Recursive domino tile replacement grammar" }
    fn description(&self) -> &'static str { "Substitution system replacing 2x1 dominoes with hierarchical patterns." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(2.0, 1.2) }

    fn compute(&self, x: f64, y: f64, _config: &FractalConfig) -> Option<f64> {
        let u = (x * 8.0).floor() as i64;
        let v = (y * 4.0).floor() as i64;
        let val = ((u ^ (v * 2)).abs() % 7) as f64 / 6.0;
        Some(val)
    }
}

// 70. Pinwheel Tiling
pub struct PinwheelTiling;
impl Fractal for PinwheelTiling {
    fn id(&self) -> &'static str { "pinwheel-tiling" }
    fn name(&self) -> &'static str { "Pinwheel Tiling" }
    fn family(&self) -> &'static str { "tiling-family" }
    fn formula(&self) -> &'static str { "Radin 1:2:sqrt(5) right-triangle aperiodic substitution with dense rotations" }
    fn description(&self) -> &'static str { "Charles Radin's pinwheel tiling featuring infinitely many triangle orientations." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(2.0, 1.2) }

    fn compute(&self, x: f64, y: f64, _config: &FractalConfig) -> Option<f64> {
        let mut px = x * 4.0;
        let mut py = y * 4.0;
        let mut angle = 0.0f64;
        for _ in 0..6 {
            let theta = py.atan2(px) + angle;
            angle += 0.463647609; // arctan(1/2)
            let r = (px * px + py * py).sqrt();
            px = (r * theta.cos()).fract() * 2.0 - 1.0;
            py = (r * theta.sin()).fract() * 2.0 - 1.0;
        }
        let t = ((px * px + py * py).sqrt()).clamp(0.0, 1.0);
        Some(t)
    }
}

// 71. Snowflake Tiling
pub struct SnowflakeTiling;
impl Fractal for SnowflakeTiling {
    fn id(&self) -> &'static str { "snowflake-tiling" }
    fn name(&self) -> &'static str { "Snowflake Tiling" }
    fn family(&self) -> &'static str { "tiling-family" }
    fn formula(&self) -> &'static str { "Hexagonal snowflake tile substitution" }
    fn description(&self) -> &'static str { "Aperiodic snowflake motifs tessellating the 2D plane." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(2.0, 1.2) }

    fn compute(&self, x: f64, y: f64, _config: &FractalConfig) -> Option<f64> {
        let r = (x * x + y * y).sqrt() * 6.0;
        let theta = y.atan2(x);
        let s = (6.0 * theta).cos();
        let val = ((r * (1.0 + 0.3 * s)).fract()).clamp(0.0, 1.0);
        Some(val)
    }
}

// 72. Ammann Tiling
pub struct AmmanTiling;
impl Fractal for AmmanTiling {
    fn id(&self) -> &'static str { "amman-tiling" }
    fn name(&self) -> &'static str { "Ammann Tiling" }
    fn family(&self) -> &'static str { "tiling-family" }
    fn formula(&self) -> &'static str { "Robert Ammann's golden ratio aperiodic tile matching rules" }
    fn description(&self) -> &'static str { "Aperiodic tilings governed by Ammann bars and golden ratio proportions." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(2.0, 1.2) }

    fn compute(&self, x: f64, y: f64, _config: &FractalConfig) -> Option<f64> {
        let u = (x * PHI * 4.0).fract();
        let v = (y * PHI * 4.0).fract();
        let val = (u * v * 4.0).clamp(0.0, 1.0);
        Some(val)
    }
}

// 73. Penrose Substitution
pub struct PenroseSubstitution;
impl Fractal for PenroseSubstitution {
    fn id(&self) -> &'static str { "penrose-substitution" }
    fn name(&self) -> &'static str { "Penrose Substitution" }
    fn family(&self) -> &'static str { "tiling-family" }
    fn formula(&self) -> &'static str { "Thick and thin rhombi substitution: φ = (1 + sqrt(5))/2 ≈ 1.618" }
    fn description(&self) -> &'static str { "Roger Penrose's famous 5-fold quasiperiodic tiling." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(2.0, 1.2) }

    fn compute(&self, x: f64, y: f64, _config: &FractalConfig) -> Option<f64> {
        // 5-fold grid projection (de Bruijn's pentagrid dual)
        let mut sum = 0.0;
        for j in 0..5 {
            let angle = j as f64 * std::f64::consts::TAU / 5.0;
            let proj = x * angle.cos() + y * angle.sin();
            sum += (proj * 4.0 * PHI).cos();
        }
        let t = (sum * 0.2 + 0.5).clamp(0.0, 1.0);
        Some(t)
    }
}

// 74. Rauzy Fractal
pub struct RauzyFractal;
impl Fractal for RauzyFractal {
    fn id(&self) -> &'static str { "rauzy" }
    fn name(&self) -> &'static str { "Rauzy Fractal" }
    fn family(&self) -> &'static str { "tiling-family" }
    fn formula(&self) -> &'static str { "Tribonacci substitution system projection" }
    fn description(&self) -> &'static str { "Gérard Rauzy's fractal domain associated with the Tribonacci substitution." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(2.0, 1.2) }

    fn compute(&self, x: f64, y: f64, _config: &FractalConfig) -> Option<f64> {
        // Tribonacci constant λ ≈ 1.839286755
        let lambda = 1.839286755;
        let u = (x * lambda * 3.0).fract();
        let v = (y * lambda * 3.0).fract();
        let val = ((u + v * 1.5).sin().abs()).clamp(0.0, 1.0);
        Some(val)
    }
}

// 75. Chair Tiling
pub struct ChairTiling;
impl Fractal for ChairTiling {
    fn id(&self) -> &'static str { "chair-tiling" }
    fn name(&self) -> &'static str { "Chair Tiling" }
    fn family(&self) -> &'static str { "tiling-family" }
    fn formula(&self) -> &'static str { "L-tromino 'chair' 4-for-1 substitution" }
    fn description(&self) -> &'static str { "Aperiodic substitution system using chair-shaped tromino tiles." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(2.0, 1.2) }

    fn compute(&self, x: f64, y: f64, _config: &FractalConfig) -> Option<f64> {
        let mut px = (x + 2.0).fract();
        let mut py = (y + 2.0).fract();
        for _ in 0..5 {
            px *= 2.0;
            py *= 2.0;
            let ix = (px.floor() as i64) % 2;
            let iy = (py.floor() as i64) % 2;
            if ix == 1 && iy == 1 {
                return Some(0.8);
            }
            px = px.fract();
            py = py.fract();
        }
        Some(0.2)
    }
}

// 76. Rhombic Tiling
pub struct RhombicTiling;
impl Fractal for RhombicTiling {
    fn id(&self) -> &'static str { "rhombic-tiling" }
    fn name(&self) -> &'static str { "Rhombic Tiling" }
    fn family(&self) -> &'static str { "tiling-family" }
    fn formula(&self) -> &'static str { "Recursive rhombic substitution tiling" }
    fn description(&self) -> &'static str { "Rhombus-based substitution generating quasiperiodic symmetries." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(2.0, 1.2) }

    fn compute(&self, x: f64, y: f64, _config: &FractalConfig) -> Option<f64> {
        let u = (x * 3.0 + y * 1.732).fract();
        let v = (x * 3.0 - y * 1.732).fract();
        let val = (u.abs() - v.abs()).abs().clamp(0.0, 1.0);
        Some(val)
    }
}

// 77. Aperiodic Tilings
pub struct AperiodicTilings;
impl Fractal for AperiodicTilings {
    fn id(&self) -> &'static str { "aperiodic-tilings" }
    fn name(&self) -> &'static str { "Aperiodic Tilings" }
    fn family(&self) -> &'static str { "tiling-family" }
    fn formula(&self) -> &'static str { "Non-periodic translational plane tessellation" }
    fn description(&self) -> &'static str { "General framework of aperiodic tiling without translational periodicity." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(2.0, 1.2) }

    fn compute(&self, x: f64, y: f64, _config: &FractalConfig) -> Option<f64> {
        let wave1 = (x * 3.0 + y * 2.0).cos();
        let wave2 = (x * 3.0 * PHI - y * 2.0).cos();
        let wave3 = (y * 4.0 * PHI).cos();
        let t = ((wave1 + wave2 + wave3) / 3.0 * 0.5 + 0.5).clamp(0.0, 1.0);
        Some(t)
    }
}

// 78. Substitution Tilings
pub struct SubstitutionTilings;
impl Fractal for SubstitutionTilings {
    fn id(&self) -> &'static str { "substitution-tilings" }
    fn name(&self) -> &'static str { "Substitution Tilings" }
    fn family(&self) -> &'static str { "tiling-family" }
    fn formula(&self) -> &'static str { "Grammar-based tile inflation and deflation rules" }
    fn description(&self) -> &'static str { "Comprehensive hierarchical substitution tiling grammar." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(2.0, 1.2) }

    fn compute(&self, x: f64, y: f64, _config: &FractalConfig) -> Option<f64> {
        let mut u = (x * 2.0).abs();
        let mut v = (y * 2.0).abs();
        for _ in 0..5 {
            let next_u = (u * 1.5 - v * 0.5).fract();
            let next_v = (u * 0.5 + v * 1.5).fract();
            u = next_u;
            v = next_v;
        }
        Some(((u + v) * 0.5).clamp(0.0, 1.0))
    }
}
