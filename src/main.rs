use clap::Parser;
use fractal_gen::cli::{BenchmarkArgs, Cli, Commands, InfoArgs, ListArgs, RenderArgs, ServeArgs, SurpriseMeArgs};
use fractal_gen::color::ColorScheme;
use fractal_gen::core::config::FractalConfig;
use fractal_gen::core::renderer::FractalRenderer;
use fractal_gen::discovery::{discover_surprise_me, DiscoveryOptions, FavoritesManager};
use fractal_gen::fractals::{get_all_fractals, get_fractal_by_id};
use fractal_gen::server::start_server;
use std::collections::BTreeMap;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Render(args) => handle_render(args)?,
        Commands::SurpriseMe(args) => handle_surprise_me(args)?,
        Commands::List(args) => handle_list(args),
        Commands::Info(args) => handle_info(args),
        Commands::Serve(args) => handle_serve(args)?,
        Commands::Benchmark(args) => handle_benchmark(args)?,
    }

    Ok(())
}

fn handle_render(args: RenderArgs) -> Result<(), Box<dyn std::error::Error>> {
    let fractal = match get_fractal_by_id(&args.fractal) {
        Some(f) => f,
        None => {
            eprintln!("Error: Unknown fractal '{}'. Run 'fractal_gen list' to see all 114 fractals.", args.fractal);
            std::process::exit(1);
        }
    };

    let color_scheme = match args.color.parse::<ColorScheme>() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };

    let bounds = fractal.bounds();
    let config = FractalConfig {
        fractal_type: fractal.id().to_string(),
        zoom: args.zoom.unwrap_or(bounds.default_zoom),
        offset_x: args.offset_x.unwrap_or(bounds.default_offset_x),
        offset_y: args.offset_y.unwrap_or(bounds.default_offset_y),
        offset_x_str: None,
        offset_y_str: None,
        iterations: args.iterations,
        color_scheme,
        x_scale: 1.0,
        y_scale: 1.0,
        julia_cx: args.julia_cx,
        julia_cy: args.julia_cy,
        power: args.power,
        escape_radius: 2.0,
        smooth_coloring: true,
    };

    println!("Rendering {} ({})", fractal.name(), fractal.family());
    println!("Formula: {}", fractal.formula());
    println!(
        "Parameters: zoom={:.2}, offset=({:.4}, {:.4}), iter={}, color={}",
        config.zoom, config.offset_x, config.offset_y, config.iterations, config.color_scheme
    );

    let renderer = FractalRenderer::new();
    let t0 = Instant::now();
    let buffer = renderer.render(&config, args.width, args.height);
    let render_duration = t0.elapsed();

    let img = buffer.to_rgba_image();
    img.save(&args.output)?;
    println!(
        "Saved {} ({}x{}) in {:.2?} to '{}'",
        fractal.name(),
        args.width,
        args.height,
        render_duration,
        args.output
    );

    Ok(())
}

fn handle_surprise_me(args: SurpriseMeArgs) -> Result<(), Box<dyn std::error::Error>> {
    let fractal = match get_fractal_by_id(&args.fractal) {
        Some(f) => f,
        None => {
            eprintln!("Error: Unknown fractal '{}'.", args.fractal);
            std::process::exit(1);
        }
    };

    println!("Running 'Surprise Me' AI discovery for {}...", fractal.name());
    println!("Evaluating {} random candidate configurations...", args.candidates);

    let fav_path = FavoritesManager::default_storage_path();
    let fav_mgr = FavoritesManager::load_from_file(&fav_path).unwrap_or_default();

    let base_config = FractalConfig {
        fractal_type: fractal.id().to_string(),
        ..Default::default()
    };

    let options = DiscoveryOptions {
        candidate_count: args.candidates,
        min_score: args.min_score,
        randomize_color: !args.keep_color,
    };

    let t0 = Instant::now();
    let discovered = discover_surprise_me(&base_config, &options, fav_mgr.model.as_ref());
    let duration = t0.elapsed();

    match discovered {
        Some(candidate) => {
            println!("\n Discovered interesting configuration in {:.2?}!", duration);
            println!("Overall Score:    {:.3}", candidate.score);
            println!("Heuristic Score:  {:.3}", candidate.heuristic_score);
            if let Some(ml) = candidate.ml_score {
                println!("ML Model Score:   {:.3}", ml);
            }
            println!("Zoom:             {:.4}", candidate.config.zoom);
            println!("Offset X:         {:.6}", candidate.config.offset_x);
            println!("Offset Y:         {:.6}", candidate.config.offset_y);
            println!("Color Scheme:     {} ({})", candidate.config.color_scheme, candidate.config.color_scheme.description());
            if fractal.family() == "julia-family" {
                println!("Julia C:          {:.4} + {:.4}i", candidate.config.julia_cx, candidate.config.julia_cy);
            }

            if let Some(out_path) = args.output {
                let renderer = FractalRenderer::new();
                let buffer = renderer.render(&candidate.config, args.width, args.height);
                buffer.to_rgba_image().save(&out_path)?;
                println!("\n Rendered discovered view saved to '{}'", out_path);
            }
        }
        None => {
            println!("\n No interesting view found with score >= {:.2}. Try increasing candidates.", args.min_score);
        }
    }

    Ok(())
}

fn handle_list(args: ListArgs) {
    if args.colors {
        println!("============================================================");
        println!("  48 BUILT-IN COLOR SCHEMES (docs/Plan.md Section 3)");
        println!("============================================================");
        for c in &ColorScheme::ALL {
            println!("{:2} | {:18} | {}", c.index(), c.identifier(), c.description());
        }
        return;
    }

    let all = get_all_fractals();
    let mut by_family: BTreeMap<&'static str, Vec<std::sync::Arc<dyn fractal_gen::Fractal>>> = BTreeMap::new();
    for f in all {
        by_family.entry(f.family()).or_default().push(f);
    }

    println!("============================================================");
    println!("  114 REGISTERED FRACTAL INVENTORY (docs/Plan.md Section 2)");
    println!("============================================================");

    for (family, fractals) in by_family {
        if let Some(ref filter) = args.family {
            if !family.to_lowercase().contains(&filter.to_lowercase()) {
                continue;
            }
        }

        println!("\n=== {} ({} fractals) ===", family.to_uppercase(), fractals.len());
        for f in fractals {
            println!("  • {:28} | {}", f.id(), f.name());
        }
    }
    println!("\nTip: Use 'fractal_gen info <fractal>' for mathematical formulas and descriptions.");
}

fn handle_info(args: InfoArgs) {
    let f = match get_fractal_by_id(&args.fractal) {
        Some(f) => f,
        None => {
            eprintln!("Unknown fractal '{}'. Use 'fractal_gen list' to see all fractals.", args.fractal);
            std::process::exit(1);
        }
    };

    let b = f.bounds();
    println!("------------------------------------------------------------");
    println!("Fractal:      {}", f.name());
    println!("Identifier:   {}", f.id());
    println!("Family:       {}", f.family());
    println!("Formula:      {}", f.formula());
    println!("Description:  {}", f.description());
    println!("Bounds X:     [{:.2}, {:.2}] (default: {:.2})", b.min_offset_x, b.max_offset_x, b.default_offset_x);
    println!("Bounds Y:     [{:.2}, {:.2}] (default: {:.2})", b.min_offset_y, b.max_offset_y, b.default_offset_y);
    println!("Zoom Range:   [{:.2}, {:.2}] (default: {:.2})", b.min_zoom, b.max_zoom, b.default_zoom);
    println!("------------------------------------------------------------");
}

fn handle_serve(args: ServeArgs) -> Result<(), Box<dyn std::error::Error>> {
    start_server(&args.host, args.port)?;
    Ok(())
}

fn handle_benchmark(args: BenchmarkArgs) -> Result<(), Box<dyn std::error::Error>> {
    let renderer = FractalRenderer::new();
    let test_fractals = [
        "mandelbrot",
        "celtic-mandelbrot",
        "burning-ship",
        "julia",
        "sierpinski-carpet",
        "newton",
        "perlin-noise",
        "rossler-attractor",
    ];

    println!("============================================================");
    println!("  FRACTAL GENERATOR BENCHMARK ({}x{}, {} iterations)", args.width, args.height, args.iterations);
    println!("============================================================");

    for id in test_fractals {
        if let Some(f) = get_fractal_by_id(id) {
            let config = FractalConfig {
                fractal_type: f.id().to_string(),
                iterations: args.iterations,
                ..Default::default()
            };

            // Warm up
            let _ = renderer.render(&config, 100, 100);

            let t0 = Instant::now();
            let _ = renderer.render(&config, args.width, args.height);
            let duration = t0.elapsed();

            let total_pixels = (args.width * args.height) as f64;
            let mpx_per_sec = (total_pixels / 1_000_000.0) / duration.as_secs_f64();

            println!(
                "{:24} | {:8.2?} | {:6.2} Mpixels/sec",
                f.name(),
                duration,
                mpx_per_sec
            );
        }
    }

    Ok(())
}
