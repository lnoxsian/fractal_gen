pub mod attractor;
pub mod cantor;
pub mod dragon;
pub mod function;
pub mod geometric;
pub mod julia;
pub mod koch;
pub mod mandelbrot;
pub mod noise;
pub mod other;
pub mod physics;
pub mod root_finding;
pub mod sierpinski;
pub mod space_filling;
pub mod tiling;
pub mod tree;

use crate::fractals::Fractal;
use std::sync::Arc;

pub fn all_fractals() -> Vec<Arc<dyn Fractal>> {
    vec![
        // 1-9: Mandelbrot Family
        Arc::new(mandelbrot::Mandelbrot),
        Arc::new(mandelbrot::CelticMandelbrot),
        Arc::new(mandelbrot::Multibrot),
        Arc::new(mandelbrot::MutantMandelbrot),
        Arc::new(mandelbrot::PhoenixMandelbrot),
        Arc::new(mandelbrot::BurningShip),
        Arc::new(mandelbrot::Tricorn),
        Arc::new(mandelbrot::Nebulabrot),
        Arc::new(mandelbrot::Buddhabrot),

        // 10-19: Julia Family
        Arc::new(julia::Julia),
        Arc::new(julia::JuliaSnakes),
        Arc::new(julia::MultibrotJulia),
        Arc::new(julia::BurningShipJulia),
        Arc::new(julia::TricornJulia),
        Arc::new(julia::PhoenixJulia),
        Arc::new(julia::LambdaJulia),
        Arc::new(julia::HybridJulia),
        Arc::new(julia::PickoverStalks),
        Arc::new(julia::Biomorphs),

        // 20-31: Sierpinski Family
        Arc::new(sierpinski::SierpinskiTriangle),
        Arc::new(sierpinski::SierpinskiArrowhead),
        Arc::new(sierpinski::SierpinskiCarpet),
        Arc::new(sierpinski::SierpinskiGasket),
        Arc::new(sierpinski::SierpinskiHexagon),
        Arc::new(sierpinski::SierpinskiPentagon),
        Arc::new(sierpinski::SierpinskiLSystem),
        Arc::new(sierpinski::SierpinskiTetrahedron),
        Arc::new(sierpinski::MengerCarpet),
        Arc::new(sierpinski::QuadrilateralSubdivision),
        Arc::new(sierpinski::RecursivePolygonSplitting),
        Arc::new(sierpinski::TriangularSubdivision),

        // 32-38: Dragon Family
        Arc::new(dragon::BinaryDragon),
        Arc::new(dragon::DragonLSystem),
        Arc::new(dragon::FoldedPaperDragon),
        Arc::new(dragon::HeighwayDragon),
        Arc::new(dragon::LevyDragon),
        Arc::new(dragon::Terdragon),
        Arc::new(dragon::Twindragon),

        // 39-44: Space-Filling Family
        Arc::new(space_filling::GosperCurve),
        Arc::new(space_filling::HilbertCurve),
        Arc::new(space_filling::LevyCCurve),
        Arc::new(space_filling::MooreCurve),
        Arc::new(space_filling::PeanoCurve),
        Arc::new(space_filling::SierpinskiCurve),

        // 45-47: Root-Finding Family
        Arc::new(root_finding::Newton),
        Arc::new(root_finding::Halley),
        Arc::new(root_finding::Nova),

        // 48-59: Plant and Tree Family
        Arc::new(tree::Plant),
        Arc::new(tree::BarnsleyFern),
        Arc::new(tree::FractalTree),
        Arc::new(tree::PythagorasTree),
        Arc::new(tree::WidePythagorasTree),
        Arc::new(tree::NarrowPythagorasTree),
        Arc::new(tree::BinaryFractalTree),
        Arc::new(tree::TernaryFractalTree),
        Arc::new(tree::QuaternaryFractalTree),
        Arc::new(tree::OakLSystemTree),
        Arc::new(tree::PineLSystemTree),
        Arc::new(tree::FractalCanopy),

        // 60-62: Koch Family
        Arc::new(koch::FractalIslands),
        Arc::new(koch::KochSnowflake),
        Arc::new(koch::QuadraticKoch),

        // 63-68: Cantor Family
        Arc::new(cantor::CantorSet),
        Arc::new(cantor::CantorDustBaseExpansion),
        Arc::new(cantor::CircularCantorDust),
        Arc::new(cantor::FatCantorSet),
        Arc::new(cantor::SmithVolterraCantorSet),
        Arc::new(cantor::RandomCantorSet),

        // 69-78: Tiling Family
        Arc::new(tiling::DominoSubstitution),
        Arc::new(tiling::PinwheelTiling),
        Arc::new(tiling::SnowflakeTiling),
        Arc::new(tiling::AmmanTiling),
        Arc::new(tiling::PenroseSubstitution),
        Arc::new(tiling::RauzyFractal),
        Arc::new(tiling::ChairTiling),
        Arc::new(tiling::RhombicTiling),
        Arc::new(tiling::AperiodicTilings),
        Arc::new(tiling::SubstitutionTilings),

        // 79-85: Attractor Family
        Arc::new(attractor::LorenzAttractor),
        Arc::new(attractor::RosslerAttractor),
        Arc::new(attractor::LyapunovFractal),
        Arc::new(attractor::ChuaAttractor),
        Arc::new(attractor::IfsSpiral),
        Arc::new(attractor::IfsMaple),
        Arc::new(attractor::IfsTree),

        // 86-89: Noise Family
        Arc::new(noise::PerlinNoise),
        Arc::new(noise::SimplexNoise),
        Arc::new(noise::FractionalBrownianMotion),
        Arc::new(noise::RandomMidpointDisplacement),

        // 90-92: Physics Family
        Arc::new(physics::DiffusionLimitedAggregation),
        Arc::new(physics::PercolationCluster),
        Arc::new(physics::LevyFlights),

        // 93-101: Geometric Family
        Arc::new(geometric::ApollonianGasket),
        Arc::new(geometric::CarpenterSquare),
        Arc::new(geometric::Cross),
        Arc::new(geometric::BoxVariants),
        Arc::new(geometric::MinkowskiSausage),
        Arc::new(geometric::CesaroFractal),
        Arc::new(geometric::RecursiveCircleRemoval),
        Arc::new(geometric::RoseWindow),
        Arc::new(geometric::MengerSponge),

        // 102-105: Function Family
        Arc::new(function::DeRhamCurve),
        Arc::new(function::WeierstrassFunction),
        Arc::new(function::TakagiFunction),
        Arc::new(function::BlancmangeCurve),

        // Other Family
        Arc::new(other::Buffalo),
        Arc::new(other::Popcorn),
        Arc::new(other::SpiderSet),
        Arc::new(other::Magnet),
        Arc::new(other::FractalDimensionPlot),
        Arc::new(other::HTree),
        Arc::new(other::GeneralizedHTree),
        Arc::new(other::Vicsek),
        Arc::new(other::FractalFlame),
    ]
}
