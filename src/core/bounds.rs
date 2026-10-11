use serde::{Deserialize, Serialize};

/// Represents the search bounds and interesting visual range for a fractal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterestingBounds {
    pub min_offset_x: f64,
    pub max_offset_x: f64,
    pub min_offset_y: f64,
    pub max_offset_y: f64,
    pub min_zoom: f64,
    pub max_zoom: f64,
    pub default_offset_x: f64,
    pub default_offset_y: f64,
    pub default_zoom: f64,
    pub min_julia_cx: f64,
    pub max_julia_cx: f64,
    pub min_julia_cy: f64,
    pub max_julia_cy: f64,
}

impl Default for InterestingBounds {
    fn default() -> Self {
        Self {
            min_offset_x: -2.0,
            max_offset_x: 2.0,
            min_offset_y: -2.0,
            max_offset_y: 2.0,
            min_zoom: 0.5,
            max_zoom: 100.0,
            default_offset_x: 0.0,
            default_offset_y: 0.0,
            default_zoom: 1.0,
            min_julia_cx: -1.5,
            max_julia_cx: 1.5,
            min_julia_cy: -1.5,
            max_julia_cy: 1.5,
        }
    }
}

impl InterestingBounds {
    pub fn mandelbrot() -> Self {
        Self {
            min_offset_x: -2.5,
            max_offset_x: 1.5,
            min_offset_y: -2.0,
            max_offset_y: 2.0,
            min_zoom: 0.5,
            max_zoom: 100.0,
            default_offset_x: -0.5,
            default_offset_y: 0.0,
            default_zoom: 1.0,
            ..Default::default()
        }
    }

    pub fn julia() -> Self {
        Self {
            min_offset_x: -2.0,
            max_offset_x: 2.0,
            min_offset_y: -2.0,
            max_offset_y: 2.0,
            min_zoom: 0.5,
            max_zoom: 50.0,
            default_offset_x: 0.0,
            default_offset_y: 0.0,
            default_zoom: 1.0,
            min_julia_cx: -1.0,
            max_julia_cx: 1.0,
            min_julia_cy: -1.0,
            max_julia_cy: 1.0,
        }
    }

    pub fn centered(size: f64, zoom: f64) -> Self {
        Self {
            min_offset_x: -size,
            max_offset_x: size,
            min_offset_y: -size,
            max_offset_y: size,
            min_zoom: 0.5,
            max_zoom: 50.0,
            default_offset_x: 0.0,
            default_offset_y: 0.0,
            default_zoom: zoom,
            ..Default::default()
        }
    }
}
