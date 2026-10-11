use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "fractal_gen",
    author = "FractalAI Generator",
    version = "0.1.0",
    about = "Comprehensive 114-Fractal, 48-Palette Generator and AI 'Surprise Me' Explorer based on Plan.md"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Render a fractal directly to a high-resolution PNG image
    Render(RenderArgs),

    /// Run the 'Surprise Me' stochastic AI discovery algorithm
    SurpriseMe(SurpriseMeArgs),

    /// List all 114 registered fractals and 48 color schemes
    List(ListArgs),

    /// Display mathematical formula, family, and description for a fractal
    Info(InfoArgs),

    /// Start the local interactive Web Explorer GUI server
    Serve(ServeArgs),

    /// Benchmark multi-threaded rendering performance across fractal families
    Benchmark(BenchmarkArgs),
}

#[derive(Args, Debug)]
pub struct RenderArgs {
    /// Fractal identifier (e.g. mandelbrot, julia, burning-ship, lorenz-attractor, newton)
    #[arg(short = 'f', long, default_value = "mandelbrot")]
    pub fractal: String,

    /// Color scheme identifier or index 0-47 (e.g. classic, fire, ocean, rainbow, cosmic)
    #[arg(short = 'c', long, default_value = "classic")]
    pub color: String,

    /// Output image width in pixels
    #[arg(short = 'W', long, default_value_t = 800)]
    pub width: u32,

    /// Output image height in pixels
    #[arg(short = 'H', long, default_value_t = 600)]
    pub height: u32,

    /// Zoom scale factor
    #[arg(short = 'z', long)]
    pub zoom: Option<f64>,

    /// Horizontal coordinate offset
    #[arg(short = 'x', long = "offset-x")]
    pub offset_x: Option<f64>,

    /// Vertical coordinate offset
    #[arg(short = 'y', long = "offset-y")]
    pub offset_y: Option<f64>,

    /// Maximum iterations depth
    #[arg(short = 'i', long, default_value_t = 125)]
    pub iterations: u32,

    /// Julia constant Real component
    #[arg(long = "julia-cx", default_value_t = -0.7269)]
    pub julia_cx: f64,

    /// Julia constant Imaginary component
    #[arg(long = "julia-cy", default_value_t = 0.1889)]
    pub julia_cy: f64,

    /// Multibrot power exponent d
    #[arg(short = 'p', long, default_value_t = 2.0)]
    pub power: f64,

    /// Destination image file path
    #[arg(short = 'o', long, default_value = "fractal.png")]
    pub output: String,
}

#[derive(Args, Debug)]
pub struct SurpriseMeArgs {
    /// Fractal type to explore (keeps this fractal type as specified in Plan.md)
    #[arg(short = 'f', long, default_value = "mandelbrot")]
    pub fractal: String,

    /// Number of candidate configurations to evaluate
    #[arg(long, default_value_t = 200)]
    pub candidates: usize,

    /// Minimum threshold score to accept (default: 0.4 per Plan.md)
    #[arg(long, default_value_t = 0.4)]
    pub min_score: f64,

    /// Do not randomize the color scheme
    #[arg(long)]
    pub keep_color: bool,

    /// Save the discovered view to PNG image
    #[arg(short = 'o', long)]
    pub output: Option<String>,

    /// Width of rendered image if saving
    #[arg(short = 'W', long, default_value_t = 800)]
    pub width: u32,

    /// Height of rendered image if saving
    #[arg(short = 'H', long, default_value_t = 600)]
    pub height: u32,
}

#[derive(Args, Debug)]
pub struct ListArgs {
    /// Show color schemes instead of fractals
    #[arg(short = 'c', long)]
    pub colors: bool,

    /// Filter by family name (e.g. mandelbrot, julia, tree, dragon, tiling)
    #[arg(short = 'F', long)]
    pub family: Option<String>,
}

#[derive(Args, Debug)]
pub struct InfoArgs {
    /// Fractal identifier (e.g. mandelbrot, buddhabrot, halley, lorenz-attractor)
    pub fractal: String,
}

#[derive(Args, Debug)]
pub struct ServeArgs {
    /// Host address to bind
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,

    /// Port number to listen
    #[arg(short = 'p', long, default_value_t = 8080)]
    pub port: u16,
}

#[derive(Args, Debug)]
pub struct BenchmarkArgs {
    /// Width for benchmark renders
    #[arg(short = 'W', long, default_value_t = 800)]
    pub width: u32,

    /// Height for benchmark renders
    #[arg(short = 'H', long, default_value_t = 600)]
    pub height: u32,

    /// Iteration count
    #[arg(short = 'i', long, default_value_t = 125)]
    pub iterations: u32,
}
