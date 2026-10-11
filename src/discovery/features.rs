use crate::core::config::FractalConfig;

/// Extract the 11-element feature vector defined in Section 6 of Plan.md:
/// [
///   typeHash,
///   logZoom,
///   offsetX,
///   offsetY,
///   iterations,
///   xScale,
///   yScale,
///   juliaCX,
///   juliaCY,
///   distanceFromOrigin,
///   zoomCategory
/// ]
pub fn extract_features(config: &FractalConfig) -> [f64; 11] {
    // 1. typeHash: simple char-code sum as in Plan.md
    let type_hash = config
        .fractal_type
        .chars()
        .map(|c| c as u32 as f64)
        .sum::<f64>()
        / 1000.0; // scaled for neural net stability

    // 2. logZoom: log-scaled zoom
    let log_zoom = (config.zoom + 1.0).ln();

    // 3. offsetX
    let offset_x = config.offset_x;

    // 4. offsetY
    let offset_y = config.offset_y;

    // 5. iterations: normalized
    let iterations = config.iterations as f64 / 400.0;

    // 6. xScale
    let x_scale = config.x_scale;

    // 7. yScale
    let y_scale = config.y_scale;

    // 8. juliaCX
    let julia_cx = config.julia_cx;

    // 9. juliaCY
    let julia_cy = config.julia_cy;

    // 10. distanceFromOrigin
    let dist_from_origin = (config.offset_x.powi(2) + config.offset_y.powi(2)).sqrt();

    // 11. zoomCategory: 0.0: overview (<2.0), 1.0: medium (2..15), 2.0: deep (>=15)
    let zoom_category = if config.zoom < 2.0 {
        0.0
    } else if config.zoom < 15.0 {
        1.0
    } else {
        2.0
    };

    [
        type_hash,
        log_zoom,
        offset_x,
        offset_y,
        iterations,
        x_scale,
        y_scale,
        julia_cx,
        julia_cy,
        dist_from_origin,
        zoom_category,
    ]
}
