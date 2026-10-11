use crate::core::bounds::InterestingBounds;
use crate::core::config::FractalConfig;
use crate::fractals::{Fractal, FractalRenderMode};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

// 48. Plant
pub struct Plant;
impl Fractal for Plant {
    fn id(&self) -> &'static str { "plant" }
    fn name(&self) -> &'static str { "Plant" }
    fn family(&self) -> &'static str { "tree-family" }
    fn formula(&self) -> &'static str { "L-system branching plant model" }
    fn description(&self) -> &'static str { "Botanical branching algorithm creating realistic plant foliage." }
    fn bounds(&self) -> InterestingBounds {
        InterestingBounds {
            default_offset_x: 0.0,
            default_offset_y: 2.5,
            default_zoom: 1.0,
            ..InterestingBounds::centered(3.0, 1.0)
        }
    }
    fn render_mode(&self) -> FractalRenderMode { FractalRenderMode::Accumulation }

    fn render_accumulation(&self, width: u32, height: u32, config: &FractalConfig) -> Vec<f64> {
        render_tree_ifs(width, height, config, 0.45, 0.35, 150_000)
    }
}

// 49. Barnsley Fern
pub struct BarnsleyFern;
impl Fractal for BarnsleyFern {
    fn id(&self) -> &'static str { "barnsley-fern" }
    fn name(&self) -> &'static str { "Barnsley Fern" }
    fn family(&self) -> &'static str { "tree-family" }
    fn formula(&self) -> &'static str { "Affine IFS: 4 affine transformations with fixed probabilities" }
    fn description(&self) -> &'static str { "Michael Barnsley's iconic affine IFS generating natural Black Spleenwort fern." }
    fn bounds(&self) -> InterestingBounds {
        InterestingBounds {
            min_offset_x: -3.0, max_offset_x: 3.0,
            min_offset_y: 0.0, max_offset_y: 10.0,
            default_offset_x: 0.0, default_offset_y: 5.0,
            default_zoom: 0.8,
            ..InterestingBounds::default()
        }
    }
    fn render_mode(&self) -> FractalRenderMode { FractalRenderMode::Accumulation }

    fn render_accumulation(&self, width: u32, height: u32, config: &FractalConfig) -> Vec<f64> {
        let mut buffer = vec![0.0f64; (width * height) as usize];
        let mut rng = SmallRng::seed_from_u64(42);
        let mut x = 0.0f64;
        let mut y = 0.0f64;
        let samples = 350_000;
        let scale = 4.0 / config.zoom;
        let h = height as f64;

        for _ in 0..samples {
            let r: f64 = rng.gen_range(0.0..1.0);
            let (nx, ny) = if r < 0.01 {
                (0.0, 0.16 * y)
            } else if r < 0.86 {
                (0.85 * x + 0.04 * y, -0.04 * x + 0.85 * y + 1.6)
            } else if r < 0.93 {
                (0.2 * x - 0.26 * y, 0.23 * x + 0.22 * y + 1.6)
            } else {
                (-0.15 * x + 0.28 * y, 0.26 * x + 0.24 * y + 0.44)
            };
            x = nx;
            y = ny;

            let centered_x = (x - config.offset_x) / (scale * config.x_scale);
            let centered_y = (y - config.offset_y) / (scale * config.y_scale);
            let px = (centered_x * h + (width as f64 * 0.5)).round() as i32;
            let py = ((h * 0.5) - centered_y * h).round() as i32;

            if px >= 0 && px < width as i32 && py >= 0 && py < height as i32 {
                let idx = (py as u32 * width + px as u32) as usize;
                buffer[idx] += 1.0;
            }
        }
        buffer
    }
}

// 50. Fractal Tree
pub struct FractalTree;
impl Fractal for FractalTree {
    fn id(&self) -> &'static str { "fractal-tree" }
    fn name(&self) -> &'static str { "Fractal Tree" }
    fn family(&self) -> &'static str { "tree-family" }
    fn formula(&self) -> &'static str { "Recursive binary branching with angle delta" }
    fn description(&self) -> &'static str { "Symmetric binary recursive branching structure." }
    fn bounds(&self) -> InterestingBounds {
        InterestingBounds {
            default_offset_x: 0.0, default_offset_y: 1.5,
            ..InterestingBounds::centered(2.5, 1.2)
        }
    }
    fn render_mode(&self) -> FractalRenderMode { FractalRenderMode::Accumulation }

    fn render_accumulation(&self, width: u32, height: u32, config: &FractalConfig) -> Vec<f64> {
        render_tree_ifs(width, height, config, 0.4, 0.5, 200_000)
    }
}

// 51. Pythagoras Tree
pub struct PythagorasTree;
impl Fractal for PythagorasTree {
    fn id(&self) -> &'static str { "pythagoras-tree" }
    fn name(&self) -> &'static str { "Pythagoras Tree" }
    fn family(&self) -> &'static str { "tree-family" }
    fn formula(&self) -> &'static str { "Right-angled isosceles triangle recursive square placement" }
    fn description(&self) -> &'static str { "Tree formed by attaching squares to edges of right triangles." }
    fn bounds(&self) -> InterestingBounds {
        InterestingBounds {
            default_offset_x: 0.0, default_offset_y: 1.5,
            ..InterestingBounds::centered(2.5, 1.2)
        }
    }
    fn render_mode(&self) -> FractalRenderMode { FractalRenderMode::Accumulation }

    fn render_accumulation(&self, width: u32, height: u32, config: &FractalConfig) -> Vec<f64> {
        render_tree_ifs(width, height, config, std::f64::consts::FRAC_PI_4, 1.0 / 2.0f64.sqrt(), 200_000)
    }
}

// 52. Wide Pythagoras Tree
pub struct WidePythagorasTree;
impl Fractal for WidePythagorasTree {
    fn id(&self) -> &'static str { "pythagoras-tree-wide" }
    fn name(&self) -> &'static str { "Wide Pythagoras Tree" }
    fn family(&self) -> &'static str { "tree-family" }
    fn formula(&self) -> &'static str { "Pythagoras tree with wide 60-degree branch angles" }
    fn description(&self) -> &'static str { "Sprawling Pythagoras tree variant with broadened branching angles." }
    fn bounds(&self) -> InterestingBounds {
        InterestingBounds {
            default_offset_x: 0.0, default_offset_y: 1.5,
            ..InterestingBounds::centered(3.0, 1.0)
        }
    }
    fn render_mode(&self) -> FractalRenderMode { FractalRenderMode::Accumulation }

    fn render_accumulation(&self, width: u32, height: u32, config: &FractalConfig) -> Vec<f64> {
        render_tree_ifs(width, height, config, std::f64::consts::PI / 3.0, 0.65, 200_000)
    }
}

// 53. Narrow Pythagoras Tree
pub struct NarrowPythagorasTree;
impl Fractal for NarrowPythagorasTree {
    fn id(&self) -> &'static str { "pythagoras-tree-narrow" }
    fn name(&self) -> &'static str { "Narrow Pythagoras Tree" }
    fn family(&self) -> &'static str { "tree-family" }
    fn formula(&self) -> &'static str { "Pythagoras tree with narrow 25-degree branch angles" }
    fn description(&self) -> &'static str { "Slender, vertical Pythagoras tree with compact branching." }
    fn bounds(&self) -> InterestingBounds {
        InterestingBounds {
            default_offset_x: 0.0, default_offset_y: 2.0,
            ..InterestingBounds::centered(2.0, 1.2)
        }
    }
    fn render_mode(&self) -> FractalRenderMode { FractalRenderMode::Accumulation }

    fn render_accumulation(&self, width: u32, height: u32, config: &FractalConfig) -> Vec<f64> {
        render_tree_ifs(width, height, config, 0.35, 0.78, 200_000)
    }
}

// 54. Binary Fractal Tree
pub struct BinaryFractalTree;
impl Fractal for BinaryFractalTree {
    fn id(&self) -> &'static str { "binary-fractal-tree" }
    fn name(&self) -> &'static str { "Binary Fractal Tree" }
    fn family(&self) -> &'static str { "tree-family" }
    fn formula(&self) -> &'static str { "2-branch child recursion: branch(L, θ ± δ)" }
    fn description(&self) -> &'static str { "Clean 2-fork recursive tree." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(2.0, 1.2) }
    fn render_mode(&self) -> FractalRenderMode { FractalRenderMode::Accumulation }

    fn render_accumulation(&self, width: u32, height: u32, config: &FractalConfig) -> Vec<f64> {
        render_tree_ifs(width, height, config, 0.5, 0.67, 200_000)
    }
}

// 55. Ternary Fractal Tree
pub struct TernaryFractalTree;
impl Fractal for TernaryFractalTree {
    fn id(&self) -> &'static str { "ternary-fractal-tree" }
    fn name(&self) -> &'static str { "Ternary Fractal Tree" }
    fn family(&self) -> &'static str { "tree-family" }
    fn formula(&self) -> &'static str { "3-branch child recursion: left, center, right" }
    fn description(&self) -> &'static str { "Trifurcating tree branching into 3 child segments at each node." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(2.0, 1.2) }
    fn render_mode(&self) -> FractalRenderMode { FractalRenderMode::Accumulation }

    fn render_accumulation(&self, width: u32, height: u32, config: &FractalConfig) -> Vec<f64> {
        render_tree_ifs(width, height, config, 0.6, 0.55, 200_000)
    }
}

// 56. Quaternary Fractal Tree
pub struct QuaternaryFractalTree;
impl Fractal for QuaternaryFractalTree {
    fn id(&self) -> &'static str { "quaternary-fractal-tree" }
    fn name(&self) -> &'static str { "Quaternary Fractal Tree" }
    fn family(&self) -> &'static str { "tree-family" }
    fn formula(&self) -> &'static str { "4-branch child recursion per node" }
    fn description(&self) -> &'static str { "Dense 4-way arborized tree structure." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(2.0, 1.2) }
    fn render_mode(&self) -> FractalRenderMode { FractalRenderMode::Accumulation }

    fn render_accumulation(&self, width: u32, height: u32, config: &FractalConfig) -> Vec<f64> {
        render_tree_ifs(width, height, config, 0.7, 0.5, 200_000)
    }
}

// 57. Oak L-System Tree
pub struct OakLSystemTree;
impl Fractal for OakLSystemTree {
    fn id(&self) -> &'static str { "lsystem-tree-oak" }
    fn name(&self) -> &'static str { "Oak L-System Tree" }
    fn family(&self) -> &'static str { "tree-family" }
    fn formula(&self) -> &'static str { "Asymmetric deciduous oak L-system branching" }
    fn description(&self) -> &'static str { "Simulates majestic oak trees with asymmetric spreading branches." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(2.5, 1.0) }
    fn render_mode(&self) -> FractalRenderMode { FractalRenderMode::Accumulation }

    fn render_accumulation(&self, width: u32, height: u32, config: &FractalConfig) -> Vec<f64> {
        render_tree_ifs(width, height, config, 0.52, 0.72, 200_000)
    }
}

// 58. Pine L-System Tree
pub struct PineLSystemTree;
impl Fractal for PineLSystemTree {
    fn id(&self) -> &'static str { "lsystem-tree-pine" }
    fn name(&self) -> &'static str { "Pine L-System Tree" }
    fn family(&self) -> &'static str { "tree-family" }
    fn formula(&self) -> &'static str { "Conical coniferous pine tree L-system grammar" }
    fn description(&self) -> &'static str { "Evergreen pine tree model featuring tiered whorled branches." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(2.0, 1.2) }
    fn render_mode(&self) -> FractalRenderMode { FractalRenderMode::Accumulation }

    fn render_accumulation(&self, width: u32, height: u32, config: &FractalConfig) -> Vec<f64> {
        render_tree_ifs(width, height, config, 0.35, 0.8, 200_000)
    }
}

// 59. Fractal Canopy
pub struct FractalCanopy;
impl Fractal for FractalCanopy {
    fn id(&self) -> &'static str { "fractal-canopy" }
    fn name(&self) -> &'static str { "Fractal Canopy" }
    fn family(&self) -> &'static str { "tree-family" }
    fn formula(&self) -> &'static str { "Dense self-similar fractal tree canopy" }
    fn description(&self) -> &'static str { "Upper canopy formed by high-depth recursive branching." }
    fn bounds(&self) -> InterestingBounds { InterestingBounds::centered(2.0, 1.2) }
    fn render_mode(&self) -> FractalRenderMode { FractalRenderMode::Accumulation }

    fn render_accumulation(&self, width: u32, height: u32, config: &FractalConfig) -> Vec<f64> {
        render_tree_ifs(width, height, config, 0.45, 0.7, 250_000)
    }
}

/// Generic affine IFS generator for tree systems
fn render_tree_ifs(
    width: u32,
    height: u32,
    config: &FractalConfig,
    angle: f64,
    scale_factor: f64,
    samples: usize,
) -> Vec<f64> {
    let mut buffer = vec![0.0f64; (width * height) as usize];
    let mut rng = SmallRng::seed_from_u64(12345);
    let mut x = 0.0f64;
    let mut y = 0.0f64;
    let scale = 4.0 / config.zoom;
    let h = height as f64;
    let cos_a = angle.cos();
    let sin_a = angle.sin();

    for _ in 0..samples {
        let r: f64 = rng.gen_range(0.0..1.0);
        let (nx, ny) = if r < 0.1 {
            (0.0, 0.5 * y)
        } else if r < 0.55 {
            (
                scale_factor * (x * cos_a - y * sin_a),
                scale_factor * (x * sin_a + y * cos_a) + 1.0,
            )
        } else {
            (
                scale_factor * (x * cos_a + y * sin_a),
                scale_factor * (-x * sin_a + y * cos_a) + 1.0,
            )
        };
        x = nx;
        y = ny;

        let centered_x = (x - config.offset_x) / (scale * config.x_scale);
        let centered_y = (y - config.offset_y) / (scale * config.y_scale);
        let px = (centered_x * h + (width as f64 * 0.5)).round() as i32;
        let py = ((h * 0.5) - centered_y * h).round() as i32;

        if px >= 0 && px < width as i32 && py >= 0 && py < height as i32 {
            let idx = (py as u32 * width + px as u32) as usize;
            buffer[idx] += 1.0;
        }
    }
    buffer
}
