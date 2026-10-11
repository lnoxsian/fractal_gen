pub mod algorithm;
pub mod favorites;
pub mod features;
pub mod neural_net;

pub use algorithm::{discover_surprise_me, is_valid_interesting_view, DiscoveryOptions, ScoredCandidate};
pub use favorites::{FavoriteConfig, FavoritesManager};
pub use features::extract_features;
pub use neural_net::DiscoveryNeuralNet;
