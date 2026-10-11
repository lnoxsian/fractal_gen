pub mod families;

use crate::core::bounds::InterestingBounds;
use crate::core::config::FractalConfig;
use crate::core::double_double::DoubleDouble;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FractalRenderMode {
    PerPixel,
    Accumulation,
}

pub trait Fractal: Send + Sync {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn family(&self) -> &'static str;
    fn formula(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn bounds(&self) -> InterestingBounds;

    fn render_mode(&self) -> FractalRenderMode {
        FractalRenderMode::PerPixel
    }

    /// Evaluates coordinate (x, y). Returns Some(t) where t in [0.0, 1.0], or None if inside set (black).
    fn compute(&self, _x: f64, _y: f64, _config: &FractalConfig) -> Option<f64> {
        None
    }

    /// High-precision 106-bit evaluation for deep zooming beyond 10^13x
    fn compute_dd(&self, x: DoubleDouble, y: DoubleDouble, config: &FractalConfig) -> Option<f64> {
        self.compute(x.to_f64(), y.to_f64(), config)
    }

    /// For accumulation/density fractals (e.g. Buddhabrot, Lorenz)
    fn render_accumulation(&self, _width: u32, _height: u32, _config: &FractalConfig) -> Vec<f64> {
        vec![]
    }
}

pub fn get_all_fractals() -> Vec<Arc<dyn Fractal>> {
    families::all_fractals()
}

pub fn get_fractal_by_id(id: &str) -> Option<Arc<dyn Fractal>> {
    let clean = id.trim().to_ascii_lowercase().replace('_', "-");
    get_all_fractals().into_iter().find(|f| f.id() == clean)
}
