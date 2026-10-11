pub mod cli;
pub mod color;
pub mod core;
pub mod discovery;
pub mod fractals;
pub mod server;

pub use cli::Cli;
pub use color::{compute_color_for_scheme, ColorScheme, Palette, PaletteCache, Rgba};
pub use core::{Complex64, FractalConfig, FractalRenderer, InterestingBounds, RenderBuffer};
pub use discovery::{
    discover_surprise_me, extract_features, is_valid_interesting_view, DiscoveryNeuralNet,
    DiscoveryOptions, FavoriteConfig, FavoritesManager, ScoredCandidate,
};
pub use fractals::{get_all_fractals, get_fractal_by_id, Fractal, FractalRenderMode};
