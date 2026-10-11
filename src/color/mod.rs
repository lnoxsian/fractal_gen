pub mod palette;
pub mod scheme;

pub use palette::{compute_color_for_scheme, Palette, PaletteCache, Rgba, PALETTE_SIZE};
pub use scheme::ColorScheme;
