use super::features::extract_features;
use super::neural_net::DiscoveryNeuralNet;
use crate::color::ColorScheme;
use crate::core::config::FractalConfig;
use crate::core::renderer::FractalRenderer;
use crate::fractals::get_fractal_by_id;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

#[derive(Debug, Clone)]
pub struct ScoredCandidate {
    pub config: FractalConfig,
    pub score: f64,
    pub heuristic_score: f64,
    pub ml_score: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct DiscoveryOptions {
    pub candidate_count: usize,
    pub min_score: f64,
    pub randomize_color: bool,
}

impl Default for DiscoveryOptions {
    fn default() -> Self {
        Self {
            candidate_count: 200,
            min_score: 0.4,
            randomize_color: true,
        }
    }
}

/// Fast validation to verify the view is visually interesting
/// (i.e. not entirely blank, trivial, or 100% inside/outside the set)
pub fn is_valid_interesting_view(config: &FractalConfig) -> bool {
    let fractal = match get_fractal_by_id(&config.fractal_type) {
        Some(f) => f,
        None => return false,
    };

    // Probe an 8x8 grid to evaluate visual contrast
    let probe_dim = 8u32;
    let mut in_count = 0;
    let mut out_count = 0;
    let mut values = Vec::with_capacity(64);

    for py in 0..probe_dim {
        for px in 0..probe_dim {
            let (x, y) = FractalRenderer::pixel_to_coord(px, py, probe_dim, probe_dim, config);
            match fractal.compute(x, y, config) {
                Some(val) => {
                    out_count += 1;
                    values.push(val);
                }
                None => {
                    in_count += 1;
                    values.push(0.0);
                }
            }
        }
    }

    let total = probe_dim * probe_dim;
    // If entirely inside (all black) or entirely outside (all background flat)
    if in_count == total || out_count == total {
        // Check if values have variance
        let avg: f64 = values.iter().sum::<f64>() / total as f64;
        let variance: f64 = values.iter().map(|v| (v - avg).powi(2)).sum::<f64>() / total as f64;
        if variance < 1e-4 {
            return false;
        }
    }
    true
}

/// Compute heuristic score strictly according to Step 5 of Plan.md:
/// base = 0.5
/// valid: +0.2, invalid: set to 0.1
/// zoom preference: 1 - |log10(zoom + 1) - 1| / 2 * 0.15
/// dist < 2.5: +0.1
/// iter preference: (1 - |iter - 125| / 125) * 0.05
/// clamped to [0, 1]
pub fn compute_heuristic_score(config: &FractalConfig, is_valid: bool) -> f64 {
    if !is_valid {
        return 0.1;
    }

    let mut score = 0.5;
    score += 0.2; // interesting view bonus

    // Zoom preference
    let zoom_score = 1.0 - ((config.zoom + 1.0).log10() - 1.0).abs() / 2.0;
    score += zoom_score * 0.15;

    // Distance from origin preference
    let dist = (config.offset_x.powi(2) + config.offset_y.powi(2)).sqrt();
    if dist < 2.5 {
        score += 0.1;
    }

    // Iteration preference around 125
    let iter_score = 1.0 - (config.iterations as f64 - 125.0).abs() / 125.0;
    score += iter_score * 0.05;

    score.clamp(0.0, 1.0)
}

/// Run Surprise Me discovery algorithm strictly according to Section 4 & 5 of Plan.md
pub fn discover_surprise_me(
    current_config: &FractalConfig,
    options: &DiscoveryOptions,
    model: Option<&DiscoveryNeuralNet>,
) -> Option<ScoredCandidate> {
    let fractal = get_fractal_by_id(&current_config.fractal_type)
        .unwrap_or_else(|| get_fractal_by_id("mandelbrot").unwrap());
    let bounds = fractal.bounds();

    let mut rng = SmallRng::from_entropy();
    let mut candidates = Vec::with_capacity(options.candidate_count);

    for _ in 0..options.candidate_count {
        // Zoom: minZoom + r * (maxZoom - minZoom)
        let r_zoom: f64 = rng.gen_range(0.0..1.0);
        let zoom = bounds.min_zoom + r_zoom * (bounds.max_zoom - bounds.min_zoom);

        // Offsets
        let offset_x = rng.gen_range(bounds.min_offset_x..bounds.max_offset_x);
        let offset_y = rng.gen_range(bounds.min_offset_y..bounds.max_offset_y);

        // Color scheme
        let color_scheme = if options.randomize_color {
            ColorScheme::ALL[rng.gen_range(0..48)]
        } else {
            current_config.color_scheme
        };

        // Julia parameters
        let (julia_cx, julia_cy) = if fractal.family() == "julia-family" {
            (
                rng.gen_range(bounds.min_julia_cx..bounds.max_julia_cx),
                rng.gen_range(bounds.min_julia_cy..bounds.max_julia_cy),
            )
        } else {
            (current_config.julia_cx, current_config.julia_cy)
        };

        let candidate_config = FractalConfig {
            fractal_type: current_config.fractal_type.clone(),
            zoom,
            offset_x,
            offset_y,
            iterations: 125, // default candidate iterations per Plan.md Step 3
            color_scheme,
            x_scale: 1.0,
            y_scale: 1.0,
            julia_cx,
            julia_cy,
            power: current_config.power,
            escape_radius: current_config.escape_radius,
            smooth_coloring: current_config.smooth_coloring,
        };

        // Step 4: Validation
        let is_valid = is_valid_interesting_view(&candidate_config);

        // Step 5: Heuristic scoring
        let heuristic_score = compute_heuristic_score(&candidate_config, is_valid);

        // Step 8: Hybrid scoring
        let (final_score, ml_score) = if let Some(net) = model {
            if heuristic_score < 0.3 {
                (heuristic_score, None)
            } else {
                let features = extract_features(&candidate_config);
                let ml = net.predict(&features);
                let hybrid = 0.3 * heuristic_score + 0.7 * ml;
                (hybrid, Some(ml))
            }
        } else {
            (heuristic_score, None)
        };

        candidates.push(ScoredCandidate {
            config: candidate_config,
            score: final_score,
            heuristic_score,
            ml_score,
        });
    }

    // Step 9: Filtering and Ranking
    let mut filtered: Vec<ScoredCandidate> = candidates
        .into_iter()
        .filter(|c| c.score >= options.min_score)
        .collect();

    filtered.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    filtered.into_iter().next()
}
