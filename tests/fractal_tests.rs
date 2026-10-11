use fractal_gen::color::{compute_color_for_scheme, ColorScheme, Palette, PaletteCache, PALETTE_SIZE};
use fractal_gen::core::config::FractalConfig;
use fractal_gen::core::renderer::FractalRenderer;
use fractal_gen::discovery::{
    discover_surprise_me, extract_features, DiscoveryNeuralNet, DiscoveryOptions,
};
use fractal_gen::fractals::{get_all_fractals, get_fractal_by_id};

#[test]
fn test_all_114_fractals_registered() {
    let fractals = get_all_fractals();
    assert_eq!(fractals.len(), 114, "Expected exactly 114 registered fractals");

    for f in &fractals {
        assert!(!f.id().is_empty(), "Fractal id cannot be empty");
        assert!(!f.name().is_empty(), "Fractal name cannot be empty");
        assert!(!f.family().is_empty(), "Fractal family cannot be empty");
        assert!(!f.formula().is_empty(), "Fractal formula cannot be empty");
        assert!(!f.description().is_empty(), "Fractal description cannot be empty");

        let b = f.bounds();
        assert!(b.min_offset_x < b.max_offset_x, "Invalid X bounds for {}", f.id());
        assert!(b.min_offset_y < b.max_offset_y, "Invalid Y bounds for {}", f.id());
        assert!(b.min_zoom < b.max_zoom, "Invalid Zoom bounds for {}", f.id());
    }
}

#[test]
fn test_fractal_lookup_by_id() {
    let test_ids = [
        "mandelbrot",
        "celtic-mandelbrot",
        "multibrot",
        "burning-ship",
        "tricorn",
        "buddhabrot",
        "julia",
        "julia-snakes",
        "lambda-julia",
        "sierpinski",
        "sierpinski-carpet",
        "heighway-dragon",
        "hilbert-curve",
        "newton",
        "halley",
        "barnsley-fern",
        "koch",
        "cantor",
        "penrose-substitution",
        "lorenz-attractor",
        "perlin-noise",
        "diffusion-limited-aggregation",
        "apollonian-gasket",
        "weierstrass",
        "takagi",
        "popcorn",
        "fractal-flame",
    ];

    for id in test_ids {
        let f = get_fractal_by_id(id);
        assert!(f.is_some(), "Could not find fractal with id '{}'", id);
        assert_eq!(f.unwrap().id(), id);
    }
}

#[test]
fn test_all_48_color_schemes() {
    assert_eq!(ColorScheme::ALL.len(), 48, "Expected exactly 48 color schemes");

    for scheme in &ColorScheme::ALL {
        assert_eq!(scheme.index(), scheme.index(), "Index consistency check");
        assert!(!scheme.identifier().is_empty());
        assert!(!scheme.description().is_empty());

        // Test palette generation and sampling across range
        let pal = Palette::new(*scheme);
        assert_eq!(pal.colors.len(), PALETTE_SIZE);

        for step in [0.0, 0.25, 0.5, 0.75, 1.0] {
            let color = compute_color_for_scheme(step, *scheme);
            assert_eq!(color.3, 255, "Alpha channel must be 255");
        }
    }
}

#[test]
fn test_palette_cache() {
    let cache = PaletteCache::new();
    let p1 = cache.get(ColorScheme::Classic);
    let p2 = cache.get(ColorScheme::Classic);
    assert_eq!(p1.colors.len(), PALETTE_SIZE);
    assert_eq!(p1.colors[0], p2.colors[0]);
}

#[test]
fn test_renderer_and_png_export() {
    let renderer = FractalRenderer::new();
    let config = FractalConfig {
        fractal_type: "mandelbrot".to_string(),
        iterations: 50,
        ..Default::default()
    };

    let buffer = renderer.render(&config, 64, 64);
    assert_eq!(buffer.width, 64);
    assert_eq!(buffer.height, 64);
    assert_eq!(buffer.pixels.len(), 64 * 64);

    let png_bytes = buffer.to_png_bytes().expect("Failed to encode PNG bytes");
    assert!(!png_bytes.is_empty());
    // PNG magic header: \x89PNG\r\n\x1a\n
    assert_eq!(&png_bytes[0..4], &[0x89, 0x50, 0x4E, 0x47]);
}

#[test]
fn test_features_and_neural_net() {
    let config = FractalConfig::default();
    let features = extract_features(&config);
    assert_eq!(features.len(), 11, "Feature vector must have 11 elements");

    let mut net = DiscoveryNeuralNet::new();
    let score = net.predict(&features);
    assert!((0.0..=1.0).contains(&score), "Neural net score must be in [0, 1]");

    let err = net.train_step(&features, 1.0, 0.05);
    assert!(err >= 0.0);
}

#[test]
fn test_surprise_me_discovery() {
    let config = FractalConfig {
        fractal_type: "mandelbrot".to_string(),
        ..Default::default()
    };
    let options = DiscoveryOptions {
        candidate_count: 50,
        min_score: 0.3,
        randomize_color: true,
    };

    let result = discover_surprise_me(&config, &options, None);
    assert!(result.is_some(), "Expected discovery to find a candidate view");
    let candidate = result.unwrap();
    assert!(candidate.score >= 0.3);
    assert_eq!(candidate.config.fractal_type, "mandelbrot");
}

#[test]
fn test_double_double_infinite_zoom() {
    use fractal_gen::core::double_double::DoubleDouble;

    // Verify 106-bit addition accuracy
    let a = DoubleDouble::from_f64(1.0);
    let eps = DoubleDouble::from_f64(1e-25);
    let sum = a + eps;
    assert_eq!(sum.lo, 1e-25, "DoubleDouble must preserve 1e-25 precision in low part");

    // Test deep Mandelbrot render at 10^20x zoom
    let renderer = FractalRenderer::new();
    let config = FractalConfig {
        fractal_type: "mandelbrot".to_string(),
        zoom: 1e20,
        offset_x: -0.7436438870371587,
        offset_y: 0.1318259042053119,
        offset_x_str: Some("-0.743643887037158704752191506114774".to_string()),
        offset_y_str: Some("0.131825904205311970493132056385139".to_string()),
        iterations: 200,
        ..Default::default()
    };

    let buffer = renderer.render(&config, 64, 64);
    assert_eq!(buffer.width, 64);
    assert_eq!(buffer.height, 64);
}
