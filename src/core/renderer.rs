use crate::color::{PaletteCache, Rgba};
use crate::core::config::FractalConfig;
use crate::fractals::{get_fractal_by_id, FractalRenderMode};
use image::{ImageBuffer, RgbaImage};
use rayon::prelude::*;

pub struct RenderBuffer {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Rgba>,
}

impl RenderBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![Rgba::BLACK; (width * height) as usize],
        }
    }

    pub fn to_rgba_image(&self) -> RgbaImage {
        let mut img = ImageBuffer::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let rgba = self.pixels[(y * self.width + x) as usize];
                img.put_pixel(x, y, image::Rgba(rgba.to_bytes()));
            }
        }
        img
    }

    pub fn to_png_bytes(&self) -> Result<Vec<u8>, image::ImageError> {
        let img = self.to_rgba_image();
        let mut bytes = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut bytes);
        img.write_to(&mut cursor, image::ImageFormat::Png)?;
        Ok(bytes)
    }
}

pub struct FractalRenderer {
    pub palette_cache: PaletteCache,
}

impl FractalRenderer {
    pub fn new() -> Self {
        Self {
            palette_cache: PaletteCache::new(),
        }
    }

    /// Convert pixel coordinate (px, py) to complex plane (x, y)
    #[inline(always)]
    pub fn pixel_to_coord(
        px: u32,
        py: u32,
        width: u32,
        height: u32,
        config: &FractalConfig,
    ) -> (f64, f64) {
        let scale = 4.0 / config.zoom;
        let h = height as f64;
        // Centered coordinates, where y increases upwards
        let centered_x = (px as f64 + 0.5 - (width as f64 * 0.5)) / h;
        let centered_y = ((h * 0.5) - (py as f64 + 0.5)) / h;

        let x = centered_x * scale * config.x_scale + config.offset_x;
        let y = centered_y * scale * config.y_scale + config.offset_y;
        (x, y)
    }

    /// Render a fractal image in parallel
    pub fn render(&self, config: &FractalConfig, width: u32, height: u32) -> RenderBuffer {
        let palette = self.palette_cache.get(config.color_scheme);
        let fractal = get_fractal_by_id(&config.fractal_type)
            .unwrap_or_else(|| get_fractal_by_id("mandelbrot").unwrap());

        match fractal.render_mode() {
            FractalRenderMode::PerPixel => {
                let total_pixels = (width * height) as usize;
                let mut pixels = vec![Rgba::BLACK; total_pixels];
                let use_dd = config.zoom > 1e13 || config.offset_x_str.is_some() || config.offset_y_str.is_some();

                if use_dd {
                    let offset_x_dd = config.offset_x_dd();
                    let offset_y_dd = config.offset_y_dd();
                    let scale = 4.0 / config.zoom;
                    let h = height as f64;
                    let w_half = width as f64 * 0.5;
                    let h_half = height as f64 * 0.5;

                    pixels
                        .par_chunks_mut(width as usize)
                        .enumerate()
                        .for_each(|(y_idx, row)| {
                            let py = y_idx as u32;
                            let centered_y = (h_half - (py as f64 + 0.5)) / h;
                            let dy = centered_y * scale * config.y_scale;
                            let y_dd = offset_y_dd.add_f64(dy);

                            for px in 0..width {
                                let centered_x = (px as f64 + 0.5 - w_half) / h;
                                let dx = centered_x * scale * config.x_scale;
                                let x_dd = offset_x_dd.add_f64(dx);

                                let val = fractal.compute_dd(x_dd, y_dd, config);
                                row[px as usize] = match val {
                                    Some(t) => palette.sample(t),
                                    None => Rgba::BLACK,
                                };
                            }
                        });
                } else {
                    pixels
                        .par_chunks_mut(width as usize)
                        .enumerate()
                        .for_each(|(y_idx, row)| {
                            let py = y_idx as u32;
                            for px in 0..width {
                                let (x, y) = Self::pixel_to_coord(px, py, width, height, config);
                                let val = fractal.compute(x, y, config);
                                row[px as usize] = match val {
                                    Some(t) => palette.sample(t),
                                    None => Rgba::BLACK,
                                };
                            }
                        });
                }

                RenderBuffer {
                    width,
                    height,
                    pixels,
                }
            }
            FractalRenderMode::Accumulation => {
                let density = fractal.render_accumulation(width, height, config);
                let mut pixels = Vec::with_capacity((width * height) as usize);

                // Find maximum density to normalize
                let mut max_density = 0.0f64;
                for &d in &density {
                    if d > max_density {
                        max_density = d;
                    }
                }

                if max_density <= 0.0 {
                    max_density = 1.0;
                }

                for &d in &density {
                    if d <= 0.0 {
                        pixels.push(Rgba::BLACK);
                    } else {
                        // Logarithmic scale for accumulation to reveal faint details
                        let t = (1.0 + d).ln() / (1.0 + max_density).ln();
                        pixels.push(palette.sample(t.clamp(0.0, 1.0)));
                    }
                }

                RenderBuffer {
                    width,
                    height,
                    pixels,
                }
            }
        }
    }
}

impl Default for FractalRenderer {
    fn default() -> Self {
        Self::new()
    }
}
