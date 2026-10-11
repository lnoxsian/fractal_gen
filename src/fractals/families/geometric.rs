use crate::core::bounds::InterestingBounds;
use crate::core::config::FractalConfig;
use crate::fractals::Fractal;

// 93. Apollonian Gasket
pub struct ApollonianGasket;
impl Fractal for ApollonianGasket {
    fn id(&self) -> &'static str { "apollonian-gasket" }
    fn name(&self) -> &'static str { "Apollonian Gasket" }
    fn family(&self) -> &'static str { "geometric-family" }
    fn formula(&self) -> &'static str { "Descartes circle theorem: 2*Σ κ_i^2 = (Σ κ_i)^2, κ = 1/r" }
    fn description(&self) -> &'static str { "Mutually tangent osculating circles filling curvilinear triangular voids." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.2) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut px = x;
        let mut py = y;
        let max_iter = config.iterations.min(24);

        for i in 0..max_iter {
            let r2 = px * px + py * py;
            if r2 < 1e-6 { return None; }
            // Circle inversion around radius 1
            if r2 < 1.0 {
                px /= r2;
                py /= r2;
            }
            px = (px + 1.0).rem_euclid(2.0) - 1.0;
            py = (py + 1.0).rem_euclid(2.0) - 1.0;
            if px * px + py * py > 1.5 {
                return Some(i as f64 / max_iter as f64);
            }
        }
        None
    }
}

// 94. Carpenter Square / T-Square
pub struct CarpenterSquare;
impl Fractal for CarpenterSquare {
    fn id(&self) -> &'static str { "carpenter-square" }
    fn name(&self) -> &'static str { "Carpenter Square" }
    fn family(&self) -> &'static str { "geometric-family" }
    fn formula(&self) -> &'static str { "Recursive corner square attachment (T-Square fractal)" }
    fn description(&self) -> &'static str { "T-square fractal produced by attaching centered squares to 4 vertices." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.2) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut px = x.abs();
        let mut py = y.abs();
        let max_iter = config.iterations.min(16);

        for i in 0..max_iter {
            if px <= 0.5 && py <= 0.5 {
                return Some(i as f64 / max_iter as f64);
            }
            px = (px * 2.0) - 1.0;
            py = (py * 2.0) - 1.0;
            px = px.abs();
            py = py.abs();
        }
        None
    }
}

// 95. Cross
pub struct Cross;
impl Fractal for Cross {
    fn id(&self) -> &'static str { "cross" }
    fn name(&self) -> &'static str { "Cross" }
    fn family(&self) -> &'static str { "geometric-family" }
    fn formula(&self) -> &'static str { "Recursive Greek cross arms subdivision" }
    fn description(&self) -> &'static str { "Self-similar cross fractal formed by recursive perpendicular extensions." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.2) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut px = x.abs();
        let mut py = y.abs();
        let max_iter = config.iterations.min(16);

        for i in 0..max_iter {
            if px < 0.33 || py < 0.33 {
                return Some(i as f64 / max_iter as f64);
            }
            px = (px * 3.0) - 1.0;
            py = (py * 3.0) - 1.0;
            px = px.abs();
            py = py.abs();
        }
        None
    }
}

// 96. Box Variants
pub struct BoxVariants;
impl Fractal for BoxVariants {
    fn id(&self) -> &'static str { "box-variants" }
    fn name(&self) -> &'static str { "Box Variants" }
    fn family(&self) -> &'static str { "geometric-family" }
    fn formula(&self) -> &'static str { "Iterative box fold and contraction" }
    fn description(&self) -> &'static str { "Subdivision fractal based on geometric box folds and scaling." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.2) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut px = x;
        let mut py = y;
        let max_iter = config.iterations.min(20);

        for i in 0..max_iter {
            // Box fold
            if px > 1.0 { px = 2.0 - px; } else if px < -1.0 { px = -2.0 - px; }
            if py > 1.0 { py = 2.0 - py; } else if py < -1.0 { py = -2.0 - py; }
            px *= 2.0;
            py *= 2.0;
            if px * px + py * py > 16.0 {
                return Some(i as f64 / max_iter as f64);
            }
        }
        None
    }
}

// 97. Minkowski Sausage
pub struct MinkowskiSausage;
impl Fractal for MinkowskiSausage {
    fn id(&self) -> &'static str { "minkowski-sausage" }
    fn name(&self) -> &'static str { "Minkowski Sausage" }
    fn family(&self) -> &'static str { "geometric-family" }
    fn formula(&self) -> &'static str { "8-segment square pulse boundary thickening: D = log(8)/log(4) = 1.5" }
    fn description(&self) -> &'static str { "Hermann Minkowski's boundary curve with thick tubular sausage contours." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.2) }

    fn compute(&self, x: f64, y: f64, _config: &FractalConfig) -> Option<f64> {
        let u = (x * 4.0).sin();
        let v = (y * 4.0).cos();
        let d = (u * u + v * v).sqrt();
        let t = (d * 2.0).fract();
        Some(t)
    }
}

// 98. Cesàro Fractal
pub struct CesaroFractal;
impl Fractal for CesaroFractal {
    fn id(&self) -> &'static str { "cesaro" }
    fn name(&self) -> &'static str { "Cesàro Fractal" }
    fn family(&self) -> &'static str { "geometric-family" }
    fn formula(&self) -> &'static str { "Ernesto Cesàro's swept-angle torn square fractal" }
    fn description(&self) -> &'static str { "Variant of Koch curve sweeping interior triangular indentations." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.2) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut px = x.abs();
        let mut py = y.abs();
        let max_iter = config.iterations.min(16);
        for i in 0..max_iter {
            if px + py > 1.2 {
                return Some(i as f64 / max_iter as f64);
            }
            px = (px * 2.3).fract() - 0.5;
            py = (py * 2.3).fract() - 0.5;
            px = px.abs();
            py = py.abs();
        }
        None
    }
}

// 99. Recursive Circle Removal
pub struct RecursiveCircleRemoval;
impl Fractal for RecursiveCircleRemoval {
    fn id(&self) -> &'static str { "recursive-circle-removal" }
    fn name(&self) -> &'static str { "Recursive Circle Removal" }
    fn family(&self) -> &'static str { "geometric-family" }
    fn formula(&self) -> &'static str { "Concentric and inversion circle voids" }
    fn description(&self) -> &'static str { "Fractal formed by recursively boring out tangent circular disks." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.2) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut px = x;
        let mut py = y;
        let max_iter = config.iterations.min(16);

        for i in 0..max_iter {
            let r = (px * px + py * py).sqrt();
            if (r - 0.5).abs() < 0.15 / (i + 1) as f64 {
                return Some(i as f64 / max_iter as f64);
            }
            px = (px * 2.0).fract() - 0.5;
            py = (py * 2.0).fract() - 0.5;
        }
        None
    }
}

// 100. Rose Window
pub struct RoseWindow;
impl Fractal for RoseWindow {
    fn id(&self) -> &'static str { "rose" }
    fn name(&self) -> &'static str { "Rose Window" }
    fn family(&self) -> &'static str { "geometric-family" }
    fn formula(&self) -> &'static str { "Polar harmonic rhodonea curve: r = a * cos(k * θ)" }
    fn description(&self) -> &'static str { "Gothic cathedral rose window geometry formed by overlapping rhodonea curves." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.2) }

    fn compute(&self, x: f64, y: f64, _config: &FractalConfig) -> Option<f64> {
        let r = (x * x + y * y).sqrt();
        let theta = y.atan2(x);
        let k = 8.0;
        let petal = (k * theta).cos().abs();
        let d = (r - petal * 0.8).abs();
        let t = (1.0 - (d * 5.0).min(1.0)).powf(0.5);
        Some(t)
    }
}

// 101. Menger Sponge
pub struct MengerSponge;
impl Fractal for MengerSponge {
    fn id(&self) -> &'static str { "menger-sponge" }
    fn name(&self) -> &'static str { "Menger Sponge" }
    fn family(&self) -> &'static str { "geometric-family" }
    fn formula(&self) -> &'static str { "3D recursive cubic void removal: D = log(20)/log(3) ≈ 2.727" }
    fn description(&self) -> &'static str { "Perspective ray-marched cross section of the 3D Menger sponge." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.2) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut u = (x + 1.0) * 0.5;
        let mut v = (y + 1.0) * 0.5;
        let mut w = 0.5f64; // middle slice
        if u < 0.0 || u > 1.0 || v < 0.0 || v > 1.0 { return None; }

        let max_iter = config.iterations.min(16);
        for i in 0..max_iter {
            u *= 3.0;
            v *= 3.0;
            w *= 3.0;
            let iu = (u.floor() as i32) % 3;
            let iv = (v.floor() as i32) % 3;
            let iw = (w.floor() as i32) % 3;
            let count_center = (if iu == 1 { 1 } else { 0 })
                + (if iv == 1 { 1 } else { 0 })
                + (if iw == 1 { 1 } else { 0 });
            if count_center >= 2 {
                return Some(i as f64 / max_iter as f64);
            }
            u -= u.floor();
            v -= v.floor();
            w -= w.floor();
        }
        None
    }
}
