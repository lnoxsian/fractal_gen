use crate::color::ColorScheme;
use crate::core::double_double::DoubleDouble;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FractalConfig {
    pub fractal_type: String,
    pub zoom: f64,
    pub offset_x: f64,
    pub offset_y: f64,
    pub offset_x_str: Option<String>,
    pub offset_y_str: Option<String>,
    pub iterations: u32,
    pub color_scheme: ColorScheme,
    pub x_scale: f64,
    pub y_scale: f64,
    pub julia_cx: f64,
    pub julia_cy: f64,
    pub power: f64,
    pub escape_radius: f64,
    pub smooth_coloring: bool,
}

impl Default for FractalConfig {
    fn default() -> Self {
        Self {
            fractal_type: "mandelbrot".to_string(),
            zoom: 1.0,
            offset_x: -0.5,
            offset_y: 0.0,
            offset_x_str: None,
            offset_y_str: None,
            iterations: 125,
            color_scheme: ColorScheme::Classic,
            x_scale: 1.0,
            y_scale: 1.0,
            // Default Julia constants from Plan.md: c = -0.7269 + 0.1889i
            julia_cx: -0.7269,
            julia_cy: 0.1889,
            power: 2.0,
            escape_radius: 2.0,
            smooth_coloring: true,
        }
    }
}

impl FractalConfig {
    pub fn offset_x_dd(&self) -> DoubleDouble {
        if let Some(ref s) = self.offset_x_str {
            if let Ok(dd) = s.parse::<DoubleDouble>() {
                return dd;
            }
        }
        DoubleDouble::from_f64(self.offset_x)
    }

    pub fn offset_y_dd(&self) -> DoubleDouble {
        if let Some(ref s) = self.offset_y_str {
            if let Ok(dd) = s.parse::<DoubleDouble>() {
                return dd;
            }
        }
        DoubleDouble::from_f64(self.offset_y)
    }

    pub fn new(fractal_type: impl Into<String>) -> Self {
        Self {
            fractal_type: fractal_type.into(),
            ..Default::default()
        }
    }
}
