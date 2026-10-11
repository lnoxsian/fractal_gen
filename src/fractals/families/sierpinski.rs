use crate::core::bounds::InterestingBounds;
use crate::core::config::FractalConfig;
use crate::fractals::Fractal;

// 20. Sierpinski Triangle
pub struct SierpinskiTriangle;
impl Fractal for SierpinskiTriangle {
    fn id(&self) -> &'static str { "sierpinski" }
    fn name(&self) -> &'static str { "Sierpinski Triangle" }
    fn family(&self) -> &'static str { "sierpinski-family" }
    fn formula(&self) -> &'static str { "Recursive triangular subdivision: D = log(3)/log(2) ≈ 1.585" }
    fn description(&self) -> &'static str { "Classic equilateral Sierpinski triangle produced by recursive subdivision." }
    fn bounds(&self) -> InterestingBounds {
        InterestingBounds {
            min_offset_x: -1.5, max_offset_x: 1.5,
            min_offset_y: -1.5, max_offset_y: 1.5,
            default_offset_x: 0.0, default_offset_y: 0.0,
            default_zoom: 1.2,
            ..InterestingBounds::default()
        }
    }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut px = x;
        let mut py = y + 0.5;
        let max_iter = config.iterations.min(32);

        for i in 0..max_iter {
            if px < 0.0 { px = -px; }
            if py < 0.0 { return Some(i as f64 / max_iter as f64); }
            // Fold along equilateral edges
            if px * 3.0f64.sqrt() + py > 1.0 {
                let temp = px;
                px = px - (px * 3.0f64.sqrt() + py - 1.0) * 3.0f64.sqrt() * 0.5;
                py = py - (temp * 3.0f64.sqrt() + py - 1.0) * 0.5;
            }
            px *= 2.0;
            py *= 2.0;
            py -= 0.5;
            if px * px + py * py > 4.0 {
                return Some(i as f64 / max_iter as f64);
            }
        }
        None
    }
}

// 21. Sierpinski Arrowhead
pub struct SierpinskiArrowhead;
impl Fractal for SierpinskiArrowhead {
    fn id(&self) -> &'static str { "sierpinski-arrowhead" }
    fn name(&self) -> &'static str { "Sierpinski Arrowhead" }
    fn family(&self) -> &'static str { "sierpinski-family" }
    fn formula(&self) -> &'static str { "Continuous curve tracing the Sierpinski triangle" }
    fn description(&self) -> &'static str { "Continuous fractal curve approximating the Sierpinski triangle." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.2) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let u = ((x + 1.0) * 256.0).floor() as i64;
        let v = ((y + 1.0) * 256.0).floor() as i64;
        if u < 0 || v < 0 { return Some(0.0); }
        let bit = (u & v) == 0;
        if bit {
            let d = ((x * x + y * y).sqrt() * 5.0).sin().abs();
            Some(d * (config.iterations as f64 / 125.0).min(1.0))
        } else {
            None
        }
    }
}

// 22. Sierpinski Carpet
pub struct SierpinskiCarpet;
impl Fractal for SierpinskiCarpet {
    fn id(&self) -> &'static str { "sierpinski-carpet" }
    fn name(&self) -> &'static str { "Sierpinski Carpet" }
    fn family(&self) -> &'static str { "sierpinski-family" }
    fn formula(&self) -> &'static str { "Divide square into 3x3 grid, remove center: D = log(8)/log(3) ≈ 1.893" }
    fn description(&self) -> &'static str { "Recursive 3x3 square grid with center tile removed at each scale." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.5) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut u = (x + 1.0) * 0.5;
        let mut v = (y + 1.0) * 0.5;
        if u < 0.0 || u > 1.0 || v < 0.0 || v > 1.0 {
            return Some(0.0);
        }
        let max_iter = config.iterations.min(16);
        for i in 0..max_iter {
            u *= 3.0;
            v *= 3.0;
            let iu = (u.floor() as i32) % 3;
            let iv = (v.floor() as i32) % 3;
            if iu == 1 && iv == 1 {
                return Some(i as f64 / max_iter as f64);
            }
            u -= u.floor();
            v -= v.floor();
        }
        None
    }
}

// 23. Sierpinski Gasket
pub struct SierpinskiGasket;
impl Fractal for SierpinskiGasket {
    fn id(&self) -> &'static str { "sierpinski-gasket" }
    fn name(&self) -> &'static str { "Sierpinski Gasket" }
    fn family(&self) -> &'static str { "sierpinski-family" }
    fn formula(&self) -> &'static str { "Generalized polygonal and triangular recursive gasket" }
    fn description(&self) -> &'static str { "Polygonal recursive subdivision gasket." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.2) }

    fn compute(&self, x: f64, y: f64, _config: &FractalConfig) -> Option<f64> {
        let r = (x * x + y * y).sqrt();
        let theta = y.atan2(x);
        let u = (r * (3.0 * theta).cos().abs() * 1024.0) as i64;
        let v = (r * (3.0 * theta).sin().abs() * 1024.0) as i64;
        if (u & v) == 0 {
            Some(((u ^ v) as f64 % 255.0) / 255.0)
        } else {
            None
        }
    }
}

// 24. Sierpinski Hexagon
pub struct SierpinskiHexagon;
impl Fractal for SierpinskiHexagon {
    fn id(&self) -> &'static str { "sierpinski-hexagon" }
    fn name(&self) -> &'static str { "Sierpinski Hexagon" }
    fn family(&self) -> &'static str { "sierpinski-family" }
    fn formula(&self) -> &'static str { "Recursive hexagonal subdivision with 6-fold symmetry" }
    fn description(&self) -> &'static str { "Hexagonal fractal formed by recursive scaling and 6-fold arrangement." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.2) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut r = (x * x + y * y).sqrt();
        let mut theta = y.atan2(x);
        let max_iter = config.iterations.min(16);
        for i in 0..max_iter {
            theta = (theta * 6.0).rem_euclid(std::f64::consts::TAU);
            r *= 3.0;
            if r > 2.0 && (theta - std::f64::consts::PI).abs() < 0.5 {
                return Some(i as f64 / max_iter as f64);
            }
            r = r.rem_euclid(2.0);
        }
        None
    }
}

// 25. Sierpinski Pentagon
pub struct SierpinskiPentagon;
impl Fractal for SierpinskiPentagon {
    fn id(&self) -> &'static str { "sierpinski-pentagon" }
    fn name(&self) -> &'static str { "Sierpinski Pentagon" }
    fn family(&self) -> &'static str { "sierpinski-family" }
    fn formula(&self) -> &'static str { "5-fold pentagonal recursive subdivision (pentaflake)" }
    fn description(&self) -> &'static str { "Five-fold symmetric fractal produced by recursive pentagram contraction." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.2) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut r = (x * x + y * y).sqrt();
        let mut theta = y.atan2(x);
        let max_iter = config.iterations.min(16);
        for i in 0..max_iter {
            theta = (theta * 5.0).rem_euclid(std::f64::consts::TAU);
            r *= 2.618; // golden ratio related scaling
            if r > 1.5 && theta > 1.0 && theta < 2.0 {
                return Some(i as f64 / max_iter as f64);
            }
            r = r.rem_euclid(1.5);
        }
        None
    }
}

// 26. Sierpinski L-System
pub struct SierpinskiLSystem;
impl Fractal for SierpinskiLSystem {
    fn id(&self) -> &'static str { "sierpinski-lsystem" }
    fn name(&self) -> &'static str { "Sierpinski L-System" }
    fn family(&self) -> &'static str { "sierpinski-family" }
    fn formula(&self) -> &'static str { "A -> B-A-B, B -> A+B+A L-system grammar" }
    fn description(&self) -> &'static str { "String rewriting L-system curve approximating the Sierpinski triangle." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.2) }

    fn compute(&self, x: f64, y: f64, _config: &FractalConfig) -> Option<f64> {
        let ix = ((x + 1.2) * 512.0) as i64;
        let iy = ((y + 1.2) * 512.0) as i64;
        if ix < 0 || iy < 0 { return None; }
        if (ix & iy) == 0 {
            let val = ((ix | iy) % 256) as f64 / 255.0;
            Some(val)
        } else {
            None
        }
    }
}

// 27. Sierpinski Tetrahedron
pub struct SierpinskiTetrahedron;
impl Fractal for SierpinskiTetrahedron {
    fn id(&self) -> &'static str { "sierpinski-tetrahedron" }
    fn name(&self) -> &'static str { "Sierpinski Tetrahedron" }
    fn family(&self) -> &'static str { "sierpinski-family" }
    fn formula(&self) -> &'static str { "3D recursive tetrahedral removal: D = log(4)/log(2) = 2" }
    fn description(&self) -> &'static str { "2D projection of the 3D Sierpinski tetrahedron fractal." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.2) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut px = x;
        let mut py = y;
        let mut pz = (1.0 - x * x - y * y).max(0.0).sqrt() * 0.5;
        let max_iter = config.iterations.min(24);

        for i in 0..max_iter {
            if px + py < 0.0 { let t = -py; py = -px; px = t; }
            if px + pz < 0.0 { let t = -pz; pz = -px; px = t; }
            if py + pz < 0.0 { let t = -pz; pz = -py; py = t; }
            px = px * 2.0 - 1.0;
            py = py * 2.0 - 1.0;
            pz = pz * 2.0 - 1.0;
            if px * px + py * py + pz * pz > 12.0 {
                return Some(i as f64 / max_iter as f64);
            }
        }
        None
    }
}

// 28. Menger Carpet
pub struct MengerCarpet;
impl Fractal for MengerCarpet {
    fn id(&self) -> &'static str { "menger-carpet" }
    fn name(&self) -> &'static str { "Menger Carpet" }
    fn family(&self) -> &'static str { "sierpinski-family" }
    fn formula(&self) -> &'static str { "2D projection of Menger sponge: D = log(20)/log(3) ≈ 2.727" }
    fn description(&self) -> &'static str { "Subdivision pattern representing a 2D cross-section of the Menger sponge." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.5) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut u = (x + 1.0) * 0.5;
        let mut v = (y + 1.0) * 0.5;
        if u < 0.0 || u > 1.0 || v < 0.0 || v > 1.0 { return Some(0.0); }
        let max_iter = config.iterations.min(16);
        for i in 0..max_iter {
            u *= 3.0;
            v *= 3.0;
            let iu = (u.floor() as i32) % 3;
            let iv = (v.floor() as i32) % 3;
            if (iu == 1 && iv == 1) || (iu == 1 && iv == 0 && i % 2 == 1) {
                return Some(i as f64 / max_iter as f64);
            }
            u -= u.floor();
            v -= v.floor();
        }
        None
    }
}

// 29. Quadrilateral Subdivision
pub struct QuadrilateralSubdivision;
impl Fractal for QuadrilateralSubdivision {
    fn id(&self) -> &'static str { "quadrilateral-subdivision" }
    fn name(&self) -> &'static str { "Quadrilateral Subdivision" }
    fn family(&self) -> &'static str { "sierpinski-family" }
    fn formula(&self) -> &'static str { "Recursive quadrilateral tile replacement" }
    fn description(&self) -> &'static str { "Subdivides quadrilaterals with asymmetric scaling and rotations." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.2) }

    fn compute(&self, x: f64, y: f64, config: &FractalConfig) -> Option<f64> {
        let mut u = x.abs();
        let mut v = y.abs();
        let max_iter = config.iterations.min(20);
        for i in 0..max_iter {
            if u > 1.0 || v > 1.0 { return Some(i as f64 / max_iter as f64); }
            let nu = (u - v).abs() * 1.5;
            let nv = (u + v) * 0.75;
            u = nu;
            v = nv;
        }
        None
    }
}

// 30. Recursive Polygon Splitting
pub struct RecursivePolygonSplitting;
impl Fractal for RecursivePolygonSplitting {
    fn id(&self) -> &'static str { "recursive-polygon-splitting" }
    fn name(&self) -> &'static str { "Recursive Polygon Splitting" }
    fn family(&self) -> &'static str { "sierpinski-family" }
    fn formula(&self) -> &'static str { "Iterative polygon dissection" }
    fn description(&self) -> &'static str { "Splits polygons recursively along centroid rays." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.2) }

    fn compute(&self, x: f64, y: f64, _config: &FractalConfig) -> Option<f64> {
        let r = (x * x + y * y).sqrt();
        let theta = y.atan2(x);
        let n_sides = 6.0;
        let sector = (theta * n_sides / std::f64::consts::TAU).floor();
        let t = ((sector * 17.0 + r * 20.0).sin() * 0.5 + 0.5).clamp(0.0, 1.0);
        Some(t)
    }
}

// 31. Triangular Subdivision
pub struct TriangularSubdivision;
impl Fractal for TriangularSubdivision {
    fn id(&self) -> &'static str { "triangular-subdivision" }
    fn name(&self) -> &'static str { "Triangular Subdivision" }
    fn family(&self) -> &'static str { "sierpinski-family" }
    fn formula(&self) -> &'static str { "Recursive triangular tiling patterns" }
    fn description(&self) -> &'static str { "Recursive triangle dissection generating complex tessellations." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(1.5, 1.2) }

    fn compute(&self, x: f64, y: f64, _config: &FractalConfig) -> Option<f64> {
        let h = 3.0f64.sqrt() * 0.5;
        let u = x;
        let v = y / h;
        let tri1 = (u + v).floor() as i64;
        let tri2 = (u - v).floor() as i64;
        let val = ((tri1.abs() ^ tri2.abs()) % 16) as f64 / 15.0;
        Some(val)
    }
}
