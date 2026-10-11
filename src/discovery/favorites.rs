use super::features::extract_features;
use super::neural_net::DiscoveryNeuralNet;
use crate::color::ColorScheme;
use crate::core::config::FractalConfig;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteConfig {
    pub fractal_type: String,
    pub zoom: f64,
    pub offset_x: f64,
    pub offset_y: f64,
    pub iterations: u32,
    pub color_scheme: ColorScheme,
    pub x_scale: f64,
    pub y_scale: f64,
    pub julia_cx: f64,
    pub julia_cy: f64,
}

impl From<&FractalConfig> for FavoriteConfig {
    fn from(c: &FractalConfig) -> Self {
        Self {
            fractal_type: c.fractal_type.clone(),
            zoom: c.zoom,
            offset_x: c.offset_x,
            offset_y: c.offset_y,
            iterations: c.iterations,
            color_scheme: c.color_scheme,
            x_scale: c.x_scale,
            y_scale: c.y_scale,
            julia_cx: c.julia_cx,
            julia_cy: c.julia_cy,
        }
    }
}

impl From<&FavoriteConfig> for FractalConfig {
    fn from(f: &FavoriteConfig) -> Self {
        Self {
            fractal_type: f.fractal_type.clone(),
            zoom: f.zoom,
            offset_x: f.offset_x,
            offset_y: f.offset_y,
            iterations: f.iterations,
            color_scheme: f.color_scheme,
            x_scale: f.x_scale,
            y_scale: f.y_scale,
            julia_cx: f.julia_cx,
            julia_cy: f.julia_cy,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FavoritesManager {
    pub favorites: Vec<FavoriteConfig>,
    pub model: Option<DiscoveryNeuralNet>,
}

impl FavoritesManager {
    pub fn new() -> Self {
        Self {
            favorites: Vec::new(),
            model: None,
        }
    }

    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let text = fs::read_to_string(path)?;
        let mgr: Self = serde_json::from_str(&text).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, e)
        })?;
        Ok(mgr)
    }

    pub fn save_to_file(&self, path: impl AsRef<Path>) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(self).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, e)
        })?;
        if let Some(parent) = path.as_ref().parent() {
            let _ = fs::create_dir_all(parent);
        }
        fs::write(path, json)?;
        Ok(())
    }

    pub fn default_storage_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(".fractalai_favorites.json")
    }

    pub fn add_favorite(&mut self, config: &FractalConfig) {
        self.favorites.push(FavoriteConfig::from(config));
        // Train model if at least 5 favorites exist
        if self.favorites.len() >= 5 {
            self.train_model();
        }
    }

    /// Train DiscoveryNeuralNet according to Section 5:
    /// Positive examples: user favorites -> output 1.0
    /// Negative examples: 2x random configs -> output 0.0
    pub fn train_model(&mut self) {
        if self.favorites.len() < 5 {
            return;
        }

        let mut net = DiscoveryNeuralNet::new();
        let mut rng = SmallRng::seed_from_u64(999);
        let lr = 0.01;
        let epochs = 50;

        for _ in 0..epochs {
            // Positive examples
            for fav in &self.favorites {
                let conf = FractalConfig::from(fav);
                let features = extract_features(&conf);
                net.train_step(&features, 1.0, lr);
            }

            // Negative examples: favorites.len() * 2
            let neg_count = self.favorites.len() * 2;
            for _ in 0..neg_count {
                let neg_conf = FractalConfig {
                    fractal_type: "mandelbrot".to_string(),
                    zoom: rng.gen_range(0.1..100.0),
                    offset_x: rng.gen_range(-2.0..2.0),
                    offset_y: rng.gen_range(-2.0..2.0),
                    iterations: rng.gen_range(10..400),
                    color_scheme: ColorScheme::ALL[rng.gen_range(0..48)],
                    x_scale: 1.0,
                    y_scale: 1.0,
                    julia_cx: rng.gen_range(-1.5..1.5),
                    julia_cy: rng.gen_range(-1.5..1.5),
                    power: 2.0,
                    escape_radius: 2.0,
                    smooth_coloring: true,
                };
                let features = extract_features(&neg_conf);
                net.train_step(&features, 0.0, lr);
            }
        }

        self.model = Some(net);
    }
}
