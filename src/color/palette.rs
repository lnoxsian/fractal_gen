use super::scheme::ColorScheme;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::sync::RwLock;

pub const PALETTE_SIZE: usize = 512;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);

impl Rgba {
    pub const BLACK: Self = Self(0, 0, 0, 255);
    pub const WHITE: Self = Self(255, 255, 255, 255);

    #[inline(always)]
    pub fn from_f64(r: f64, g: f64, b: f64) -> Self {
        let clamp_byte = |v: f64| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
        Self(clamp_byte(r), clamp_byte(g), clamp_byte(b), 255)
    }

    #[inline(always)]
    pub fn to_bytes(self) -> [u8; 4] {
        [self.0, self.1, self.2, self.3]
    }
}

/// A precomputed 512-entry palette texture / lookup table
#[derive(Debug, Clone)]
pub struct Palette {
    pub scheme: ColorScheme,
    pub colors: Vec<Rgba>,
}

impl Palette {
    pub fn new(scheme: ColorScheme) -> Self {
        let mut colors = Vec::with_capacity(PALETTE_SIZE);
        for i in 0..PALETTE_SIZE {
            let t = i as f64 / (PALETTE_SIZE - 1) as f64;
            colors.push(compute_color_for_scheme(t, scheme));
        }
        Self { scheme, colors }
    }

    /// Sample the palette with normalized intensity t in [0, 1]
    #[inline(always)]
    pub fn sample(&self, t: f64) -> Rgba {
        if t <= 0.0 {
            return self.colors[0];
        }
        if t >= 1.0 {
            return self.colors[PALETTE_SIZE - 1];
        }
        let index = (t * (PALETTE_SIZE - 1) as f64) as usize;
        self.colors[index.min(PALETTE_SIZE - 1)]
    }
}

/// Global thread-safe palette cache
pub struct PaletteCache {
    cache: RwLock<HashMap<ColorScheme, Palette>>,
}

impl PaletteCache {
    pub fn new() -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
        }
    }

    pub fn get(&self, scheme: ColorScheme) -> Palette {
        {
            let read_lock = self.cache.read().unwrap();
            if let Some(pal) = read_lock.get(&scheme) {
                return pal.clone();
            }
        }
        let mut write_lock = self.cache.write().unwrap();
        write_lock
            .entry(scheme)
            .or_insert_with(|| Palette::new(scheme))
            .clone()
    }
}

impl Default for PaletteCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper for linear interpolation between two RGB points
#[inline(always)]
fn lerp_rgb(c1: (f64, f64, f64), c2: (f64, f64, f64), t: f64) -> (f64, f64, f64) {
    let t = t.clamp(0.0, 1.0);
    (
        c1.0 + (c2.0 - c1.0) * t,
        c1.1 + (c2.1 - c1.1) * t,
        c1.2 + (c2.2 - c1.2) * t,
    )
}

/// Helper for multi-stop color gradients
fn multi_stop_gradient(stops: &[(f64, (f64, f64, f64))], t: f64) -> (f64, f64, f64) {
    let t = t.clamp(0.0, 1.0);
    if stops.is_empty() {
        return (0.0, 0.0, 0.0);
    }
    if t <= stops[0].0 {
        return stops[0].1;
    }
    for i in 0..(stops.len() - 1) {
        let (p1, c1) = stops[i];
        let (p2, c2) = stops[i + 1];
        if t >= p1 && t <= p2 {
            let local_t = if (p2 - p1).abs() < 1e-6 { 0.0 } else { (t - p1) / (p2 - p1) };
            return lerp_rgb(c1, c2, local_t);
        }
    }
    stops.last().unwrap().1
}

/// Cosine based color palette generator: color = a + b * cos(2*pi*(c*t + d))
#[inline(always)]
fn cosine_palette(
    t: f64,
    a: (f64, f64, f64),
    b: (f64, f64, f64),
    c: (f64, f64, f64),
    d: (f64, f64, f64),
) -> (f64, f64, f64) {
    let two_pi = 2.0 * PI;
    (
        a.0 + b.0 * (two_pi * (c.0 * t + d.0)).cos(),
        a.1 + b.1 * (two_pi * (c.1 * t + d.1)).cos(),
        a.2 + b.2 * (two_pi * (c.2 * t + d.2)).cos(),
    )
}

/// Compute RGB color for normalized value t in [0, 1] according to Plan.md formulas
pub fn compute_color_for_scheme(t: f64, scheme: ColorScheme) -> Rgba {
    let t = t.clamp(0.0, 1.0);
    let (r, g, b) = match scheme {
        // 0. Classic: R = 0.5t, G = t, B = min(1.5t, 1)
        ColorScheme::Classic => (0.5 * t, t, (1.5 * t).min(1.0)),

        // 1. Fire: Red, orange, yellow-white
        ColorScheme::Fire => multi_stop_gradient(
            &[
                (0.0, (0.0, 0.0, 0.0)),
                (0.33, (0.85, 0.1, 0.0)),
                (0.66, (1.0, 0.6, 0.0)),
                (0.9, (1.0, 0.95, 0.2)),
                (1.0, (1.0, 1.0, 1.0)),
            ],
            t,
        ),

        // 2. Ocean: R = 0, G = 0.5t, B = t
        ColorScheme::Ocean => multi_stop_gradient(
            &[
                (0.0, (0.0, 0.02, 0.1)),
                (0.4, (0.0, 0.25, 0.65)),
                (0.8, (0.05, 0.65, 0.9)),
                (1.0, (0.6, 0.95, 1.0)),
            ],
            t,
        ),

        // 3. Rainbow: Phase-shifted cosine waves R=0.5+0.5cos(θ), G=cos(θ+2.09), B=cos(θ+4.18)
        ColorScheme::Rainbow => {
            let theta = 2.0 * PI * t;
            (
                0.5 + 0.5 * theta.cos(),
                0.5 + 0.5 * (theta + 2.09).cos(),
                0.5 + 0.5 * (theta + 4.18).cos(),
            )
        }

        // 4. Monochrome: R = G = B = t
        ColorScheme::Monochrome => (t, t, t),

        // 5. Forest: R = 0.3t, G = 0.8t, B = 0.4t
        ColorScheme::Forest => multi_stop_gradient(
            &[
                (0.0, (0.02, 0.1, 0.03)),
                (0.5, (0.1, 0.55, 0.18)),
                (0.85, (0.35, 0.85, 0.4)),
                (1.0, (0.8, 0.98, 0.6)),
            ],
            t,
        ),

        // 6. Sunset: Red, orange, warm brown
        ColorScheme::Sunset => multi_stop_gradient(
            &[
                (0.0, (0.15, 0.03, 0.1)),
                (0.35, (0.75, 0.12, 0.15)),
                (0.7, (1.0, 0.45, 0.05)),
                (0.92, (1.0, 0.8, 0.3)),
                (1.0, (1.0, 0.98, 0.85)),
            ],
            t,
        ),

        // 7. Purple: Purple and violet
        ColorScheme::Purple => multi_stop_gradient(
            &[
                (0.0, (0.05, 0.0, 0.1)),
                (0.4, (0.45, 0.08, 0.65)),
                (0.75, (0.75, 0.25, 0.9)),
                (1.0, (0.95, 0.75, 1.0)),
            ],
            t,
        ),

        // 8. Cyan: Cyan and turquoise
        ColorScheme::Cyan => multi_stop_gradient(
            &[
                (0.0, (0.0, 0.08, 0.1)),
                (0.4, (0.02, 0.5, 0.6)),
                (0.8, (0.1, 0.88, 0.95)),
                (1.0, (0.8, 1.0, 1.0)),
            ],
            t,
        ),

        // 9. Gold: Gold and yellow
        ColorScheme::Gold => multi_stop_gradient(
            &[
                (0.0, (0.12, 0.08, 0.01)),
                (0.4, (0.65, 0.45, 0.05)),
                (0.8, (1.0, 0.82, 0.15)),
                (1.0, (1.0, 0.98, 0.75)),
            ],
            t,
        ),

        // 10. Ice: Pale blue and white
        ColorScheme::Ice => multi_stop_gradient(
            &[
                (0.0, (0.05, 0.1, 0.2)),
                (0.5, (0.4, 0.7, 0.9)),
                (0.85, (0.75, 0.9, 0.98)),
                (1.0, (1.0, 1.0, 1.0)),
            ],
            t,
        ),

        // 11. Neon: Pulsing bright neon tones
        ColorScheme::Neon => {
            let pulse = (0.5 * ((PI * t).sin() + 1.0)).clamp(0.0, 1.0);
            let theta = 2.0 * PI * t * 1.5;
            let base_r = 0.5 + 0.5 * (theta).cos();
            let base_g = 0.5 + 0.5 * (theta + 2.0).cos();
            let base_b = 0.5 + 0.5 * (theta + 4.0).cos();
            (
                (base_r * 0.7 + pulse * 0.5).min(1.0),
                (base_g * 0.7 + pulse * 0.5).min(1.0),
                (base_b * 0.7 + pulse * 0.5).min(1.0),
            )
        }

        // 12. Rainbow Pastel: Soft pastel rainbow
        ColorScheme::RainbowPastel => cosine_palette(
            t,
            (0.8, 0.8, 0.8),
            (0.2, 0.2, 0.2),
            (1.0, 1.0, 1.0),
            (0.0, 0.33, 0.67),
        ),

        // 13. Rainbow Dark: Dark saturated rainbow
        ColorScheme::RainbowDark => cosine_palette(
            t,
            (0.35, 0.35, 0.35),
            (0.35, 0.35, 0.35),
            (1.0, 1.0, 1.0),
            (0.0, 0.33, 0.67),
        ),

        // 14. Rainbow Vibrant: High-saturation rainbow
        ColorScheme::RainbowVibrant => cosine_palette(
            t,
            (0.5, 0.5, 0.5),
            (0.5, 0.5, 0.5),
            (1.0, 1.0, 1.0),
            (0.0, 0.333, 0.666),
        ),

        // 15. Rainbow Double: Two-cycle rainbow across the range
        ColorScheme::RainbowDouble => cosine_palette(
            t,
            (0.5, 0.5, 0.5),
            (0.5, 0.5, 0.5),
            (2.0, 2.0, 2.0),
            (0.0, 0.33, 0.67),
        ),

        // 16. Rainbow Shifted: Rainbow with hue rotation
        ColorScheme::RainbowShifted => cosine_palette(
            t,
            (0.5, 0.5, 0.5),
            (0.5, 0.5, 0.5),
            (1.0, 1.0, 1.0),
            (0.25, 0.58, 0.92),
        ),

        // 17. Cosmic: Dark purple/blue to cyan-white with sparkle
        ColorScheme::Cosmic => {
            let base = multi_stop_gradient(
                &[
                    (0.0, (0.02, 0.01, 0.08)),
                    (0.4, (0.25, 0.05, 0.5)),
                    (0.75, (0.1, 0.55, 0.85)),
                    (1.0, (0.9, 0.98, 1.0)),
                ],
                t,
            );
            let sparkle = 0.15 * (16.0 * PI * t).sin().abs() * t;
            (
                (base.0 + sparkle).min(1.0),
                (base.1 + sparkle).min(1.0),
                (base.2 + sparkle).min(1.0),
            )
        }

        // 18. Aurora: Green, cyan, blue, purple
        ColorScheme::Aurora => multi_stop_gradient(
            &[
                (0.0, (0.01, 0.05, 0.08)),
                (0.3, (0.05, 0.75, 0.45)),
                (0.65, (0.1, 0.6, 0.95)),
                (0.9, (0.6, 0.2, 0.8)),
                (1.0, (0.9, 0.8, 1.0)),
            ],
            t,
        ),

        // 19. Coral: Deep blue, teal, coral-pink
        ColorScheme::Coral => multi_stop_gradient(
            &[
                (0.0, (0.05, 0.08, 0.25)),
                (0.35, (0.08, 0.55, 0.65)),
                (0.7, (0.95, 0.45, 0.4)),
                (1.0, (1.0, 0.8, 0.75)),
            ],
            t,
        ),

        // 20. Autumn: Red, orange, gold
        ColorScheme::Autumn => multi_stop_gradient(
            &[
                (0.0, (0.15, 0.04, 0.02)),
                (0.35, (0.7, 0.15, 0.05)),
                (0.7, (0.9, 0.5, 0.1)),
                (0.92, (0.95, 0.8, 0.2)),
                (1.0, (1.0, 0.95, 0.7)),
            ],
            t,
        ),

        // 21. Midnight: Navy, purple, magenta
        ColorScheme::Midnight => multi_stop_gradient(
            &[
                (0.0, (0.02, 0.02, 0.08)),
                (0.4, (0.1, 0.1, 0.45)),
                (0.75, (0.5, 0.1, 0.55)),
                (1.0, (0.95, 0.35, 0.85)),
            ],
            t,
        ),

        // 22. Emerald: Dark green, emerald, gold
        ColorScheme::Emerald => multi_stop_gradient(
            &[
                (0.0, (0.01, 0.08, 0.03)),
                (0.45, (0.05, 0.65, 0.3)),
                (0.8, (0.4, 0.85, 0.2)),
                (1.0, (0.95, 0.85, 0.3)),
            ],
            t,
        ),

        // 23. Rosegold: Pink, rose, gold
        ColorScheme::Rosegold => multi_stop_gradient(
            &[
                (0.0, (0.12, 0.05, 0.08)),
                (0.4, (0.75, 0.35, 0.45)),
                (0.75, (0.95, 0.6, 0.6)),
                (1.0, (1.0, 0.88, 0.7)),
            ],
            t,
        ),

        // 24. Electric: Bright complementary neon colors
        ColorScheme::Electric => cosine_palette(
            t,
            (0.5, 0.5, 0.5),
            (0.5, 0.5, 0.5),
            (1.0, 1.0, 0.5),
            (0.8, 0.9, 0.3),
        ),

        // 25. Vintage: Muted pastel/sepia colors
        ColorScheme::Vintage => multi_stop_gradient(
            &[
                (0.0, (0.18, 0.14, 0.11)),
                (0.4, (0.55, 0.45, 0.35)),
                (0.75, (0.8, 0.72, 0.58)),
                (1.0, (0.94, 0.9, 0.8)),
            ],
            t,
        ),

        // 26. Tropical: Cyan, magenta, yellow
        ColorScheme::Tropical => multi_stop_gradient(
            &[
                (0.0, (0.05, 0.1, 0.3)),
                (0.35, (0.0, 0.85, 0.85)),
                (0.7, (0.95, 0.1, 0.65)),
                (1.0, (1.0, 0.95, 0.1)),
            ],
            t,
        ),

        // 27. Galaxy: Indigo, violet, bright star-like highlights
        ColorScheme::Galaxy => {
            let base = multi_stop_gradient(
                &[
                    (0.0, (0.01, 0.01, 0.05)),
                    (0.35, (0.2, 0.05, 0.45)),
                    (0.7, (0.55, 0.15, 0.75)),
                    (1.0, (0.85, 0.75, 1.0)),
                ],
                t,
            );
            let highlight = 0.2 * (24.0 * PI * t).cos().max(0.0) * t;
            (
                (base.0 + highlight).min(1.0),
                (base.1 + highlight).min(1.0),
                (base.2 + highlight).min(1.0),
            )
        }

        // 28. Lava: Dark red, orange, yellow-white
        ColorScheme::Lava => multi_stop_gradient(
            &[
                (0.0, (0.08, 0.0, 0.0)),
                (0.3, (0.6, 0.05, 0.0)),
                (0.65, (1.0, 0.35, 0.0)),
                (0.9, (1.0, 0.85, 0.1)),
                (1.0, (1.0, 1.0, 0.8)),
            ],
            t,
        ),

        // 29. Arctic: Blue, cyan, white
        ColorScheme::Arctic => multi_stop_gradient(
            &[
                (0.0, (0.02, 0.05, 0.15)),
                (0.4, (0.1, 0.4, 0.8)),
                (0.8, (0.45, 0.85, 0.98)),
                (1.0, (1.0, 1.0, 1.0)),
            ],
            t,
        ),

        // 30. Sakura: Cherry-blossom pinks
        ColorScheme::Sakura => multi_stop_gradient(
            &[
                (0.0, (0.15, 0.05, 0.1)),
                (0.4, (0.75, 0.3, 0.45)),
                (0.75, (0.95, 0.65, 0.78)),
                (1.0, (1.0, 0.92, 0.95)),
            ],
            t,
        ),

        // 31. Volcanic: Black, dark red, orange
        ColorScheme::Volcanic => multi_stop_gradient(
            &[
                (0.0, (0.0, 0.0, 0.0)),
                (0.35, (0.4, 0.02, 0.02)),
                (0.7, (0.85, 0.2, 0.0)),
                (1.0, (1.0, 0.6, 0.1)),
            ],
            t,
        ),

        // 32. Mint: Green, mint, teal
        ColorScheme::Mint => multi_stop_gradient(
            &[
                (0.0, (0.02, 0.1, 0.08)),
                (0.4, (0.1, 0.55, 0.45)),
                (0.75, (0.35, 0.9, 0.7)),
                (1.0, (0.85, 1.0, 0.95)),
            ],
            t,
        ),

        // 33. Sunrise: Pink, orange, yellow
        ColorScheme::Sunrise => multi_stop_gradient(
            &[
                (0.0, (0.1, 0.02, 0.15)),
                (0.35, (0.7, 0.15, 0.4)),
                (0.7, (1.0, 0.45, 0.2)),
                (1.0, (1.0, 0.9, 0.3)),
            ],
            t,
        ),

        // 34. Steel: Metallic gray-blue
        ColorScheme::Steel => multi_stop_gradient(
            &[
                (0.0, (0.08, 0.1, 0.12)),
                (0.4, (0.3, 0.38, 0.45)),
                (0.75, (0.6, 0.7, 0.78)),
                (1.0, (0.9, 0.94, 0.98)),
            ],
            t,
        ),

        // 35. Prism: Inverted complementary rainbow
        ColorScheme::Prism => cosine_palette(
            t,
            (0.5, 0.5, 0.5),
            (-0.5, -0.5, -0.5),
            (1.0, 1.0, 1.0),
            (0.0, 0.33, 0.67),
        ),

        // 36. Mystic: Purple, magenta, pink with sparkle
        ColorScheme::Mystic => {
            let base = multi_stop_gradient(
                &[
                    (0.0, (0.1, 0.0, 0.15)),
                    (0.4, (0.55, 0.05, 0.5)),
                    (0.75, (0.85, 0.25, 0.65)),
                    (1.0, (1.0, 0.75, 0.9)),
                ],
                t,
            );
            let sparkle = 0.12 * (18.0 * PI * t).sin().abs() * t;
            (
                (base.0 + sparkle).min(1.0),
                (base.1 + sparkle).min(1.0),
                (base.2 + sparkle).min(1.0),
            )
        }

        // 37. Amber: Brown, amber, golden yellow
        ColorScheme::Amber => multi_stop_gradient(
            &[
                (0.0, (0.1, 0.05, 0.01)),
                (0.4, (0.55, 0.28, 0.05)),
                (0.75, (0.9, 0.6, 0.1)),
                (1.0, (1.0, 0.88, 0.35)),
            ],
            t,
        ),

        // 38. Noir Ember: Near-black to ember red and white
        ColorScheme::NoirEmber => multi_stop_gradient(
            &[
                (0.0, (0.01, 0.01, 0.01)),
                (0.45, (0.35, 0.02, 0.02)),
                (0.8, (0.9, 0.2, 0.05)),
                (1.0, (1.0, 0.95, 0.9)),
            ],
            t,
        ),

        // 39. Ultraviolet: Black, violet, electric cyan
        ColorScheme::Ultraviolet => multi_stop_gradient(
            &[
                (0.0, (0.01, 0.0, 0.03)),
                (0.35, (0.35, 0.05, 0.6)),
                (0.7, (0.65, 0.1, 0.9)),
                (1.0, (0.1, 0.95, 1.0)),
            ],
            t,
        ),

        // 40. Toxic: Black, neon green, sulfur yellow
        ColorScheme::Toxic => multi_stop_gradient(
            &[
                (0.0, (0.0, 0.02, 0.0)),
                (0.4, (0.15, 0.75, 0.05)),
                (0.75, (0.65, 0.95, 0.1)),
                (1.0, (0.95, 1.0, 0.3)),
            ],
            t,
        ),

        // 41. Blood Moon: Black, crimson, magenta
        ColorScheme::BloodMoon => multi_stop_gradient(
            &[
                (0.0, (0.02, 0.0, 0.01)),
                (0.4, (0.5, 0.02, 0.08)),
                (0.75, (0.85, 0.08, 0.25)),
                (1.0, (0.95, 0.35, 0.6)),
            ],
            t,
        ),

        // 42. Void Ice: Black, deep blue, icy white
        ColorScheme::VoidIce => multi_stop_gradient(
            &[
                (0.0, (0.0, 0.0, 0.02)),
                (0.35, (0.05, 0.15, 0.5)),
                (0.7, (0.2, 0.6, 0.95)),
                (1.0, (0.9, 0.98, 1.0)),
            ],
            t,
        ),

        // 43. Acid Rain: Teal, acid green, mint
        ColorScheme::AcidRain => multi_stop_gradient(
            &[
                (0.0, (0.01, 0.1, 0.1)),
                (0.35, (0.05, 0.55, 0.45)),
                (0.7, (0.5, 0.9, 0.2)),
                (1.0, (0.8, 1.0, 0.65)),
            ],
            t,
        ),

        // 44. Solar Flare: Black, orange, yellow-white
        ColorScheme::SolarFlare => multi_stop_gradient(
            &[
                (0.0, (0.02, 0.01, 0.0)),
                (0.35, (0.7, 0.25, 0.0)),
                (0.7, (1.0, 0.65, 0.05)),
                (1.0, (1.0, 0.98, 0.7)),
            ],
            t,
        ),

        // 45. Deep Sea: Navy, teal, bioluminescent cyan
        ColorScheme::DeepSea => multi_stop_gradient(
            &[
                (0.0, (0.01, 0.02, 0.08)),
                (0.4, (0.02, 0.25, 0.45)),
                (0.75, (0.05, 0.7, 0.8)),
                (1.0, (0.4, 1.0, 0.95)),
            ],
            t,
        ),

        // 46. Chiaroscuro: High-contrast grayscale with blue tint
        ColorScheme::Chiaroscuro => {
            let curve = if t < 0.5 {
                2.0 * t * t
            } else {
                1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
            };
            (curve * 0.9, curve * 0.92, curve * 1.0)
        }

        // 47. Spectral Night: Dark, saturated rainbow
        ColorScheme::SpectralNight => cosine_palette(
            t,
            (0.3, 0.3, 0.3),
            (0.3, 0.3, 0.3),
            (1.0, 1.0, 1.0),
            (0.5, 0.83, 0.17),
        ),
    };

    Rgba::from_f64(r, g, b)
}
