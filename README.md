# FractalAI Generator (`fractal_gen`)

A high-performance Rust fractal generator, interactive explorer, and AI-assisted "Surprise Me" discovery engine built according to [`docs/Plan.md`](file:///home/lnoxsian/lnox-files/project-rust/fractal_gen/docs/Plan.md).

## Features

- **114 Registered Fractal Types**: Complete implementations across all 16 families from `docs/Plan.md`:
  - **Mandelbrot Family (1–9)**: `mandelbrot`, `celtic-mandelbrot`, `multibrot`, `mutant-mandelbrot`, `phoenix-mandelbrot`, `burning-ship`, `tricorn`, `nebulabrot`, `buddhabrot`
  - **Julia Family (10–19)**: `julia`, `julia-snakes`, `multibrot-julia`, `burning-ship-julia`, `tricorn-julia`, `phoenix-julia`, `lambda-julia`, `hybrid-julia`, `pickover-stalks`, `biomorphs`
  - **Sierpinski Family (20–31)**: `sierpinski`, `sierpinski-arrowhead`, `sierpinski-carpet`, `sierpinski-gasket`, `sierpinski-hexagon`, `sierpinski-pentagon`, `sierpinski-lsystem`, `sierpinski-tetrahedron`, `menger-carpet`, `quadrilateral-subdivision`, `recursive-polygon-splitting`, `triangular-subdivision`
  - **Dragon Family (32–38)**: `binary-dragon`, `dragon-lsystem`, `folded-paper-dragon`, `heighway-dragon`, `levy-dragon`, `terdragon`, `twindragon`
  - **Space-Filling Family (39–44)**: `gosper-curve`, `hilbert-curve`, `levy-c-curve`, `moore-curve`, `peano-curve`, `sierpinski-curve`
  - **Root-Finding Basins (45–47)**: `newton`, `halley`, `nova`
  - **Plant and Tree Family (48–59)**: `plant`, `barnsley-fern`, `fractal-tree`, `pythagoras-tree`, `pythagoras-tree-wide`, `pythagoras-tree-narrow`, `binary-fractal-tree`, `ternary-fractal-tree`, `quaternary-fractal-tree`, `lsystem-tree-oak`, `lsystem-tree-pine`, `fractal-canopy`
  - **Koch Family (60–62)**: `fractal-islands`, `koch`, `quadratic-koch`
  - **Cantor Family (63–68)**: `cantor`, `cantor-dust-base-expansion`, `cantor-dust-circular`, `fat-cantor`, `smith-volterra-cantor`, `random-cantor`
  - **Tilings (69–78)**: `domino-substitution`, `pinwheel-tiling`, `snowflake-tiling`, `amman-tiling`, `penrose-substitution`, `rauzy`, `chair-tiling`, `rhombic-tiling`, `aperiodic-tilings`, `substitution-tilings`
  - **Attractors and Dynamical Systems (79–85)**: `lorenz-attractor`, `rossler-attractor`, `lyapunov`, `chua-attractor`, `ifs-spiral`, `ifs-maple`, `ifs-tree`
  - **Procedural Noise (86–89)**: `perlin-noise`, `simplex-noise`, `fractional-brownian-motion`, `random-midpoint-displacement`
  - **Physics & Random Growth (90–92)**: `diffusion-limited-aggregation`, `percolation-cluster`, `levy-flights`
  - **Geometric Fractals (93–101)**: `apollonian-gasket`, `carpenter-square`, `cross`, `box-variants`, `minkowski-sausage`, `cesaro`, `recursive-circle-removal`, `rose`, `menger-sponge`
  - **Self-Similar Functions (102–105)**: `de-rham-curve`, `weierstrass`, `takagi`, `blancmange`
  - **Other Procedural (Section P)**: `buffalo`, `popcorn`, `spider-set`, `magnet`, `fractal-dimension-plot`, `h-tree`, `h-tree-generalized`, `vicsek`, `fractal-flame`
- **48 Built-In Color Schemes**:
  - Implements all 48 color schemes from Section 3 of `Plan.md` (`classic`, `fire`, `ocean`, `rainbow`, `cosmic`, `lava`, `aurora`, `sakura`, `electric`, etc.) with precomputed 512-entry palette caching.
- **AI "Surprise Me" Stochastic Discovery Engine**:
  - Evaluates 200 candidate configurations within fractal-specific `interestingBounds`.
  - Heuristic validation and scoring: zoom preference, origin distance, iteration tuning.
  - 11-value ML feature vector extraction and optional Synaptic-style neural network training on favorites.
- **Multi-Threaded Rendering**:
  - Parallelized row rendering via `rayon`, achieving over 150–300 Mpixels/second.
- **Interactive Web Explorer**:
  - Built-in lightweight HTTP server serving a web UI with drag-to-pan, wheel zoom, fractal selector, palette switcher, live "Surprise Me", and HD PNG export.

---

## Installation & Build

```bash
cargo build --release
```

The compiled binary will be placed at `./target/release/fractal_gen`.

---

## CLI Usage

### 1. Render a Fractal to PNG

```bash
# Render Mandelbrot with Cosmic palette
fractal_gen render -f mandelbrot -c cosmic -o mandelbrot.png

# Render Burning Ship with Lava palette at 1920x1080
fractal_gen render -f burning-ship -c lava -W 1920 -H 1080 -o burning_ship.png

# Render Julia Set with custom constants
fractal_gen render -f julia -c electric --julia-cx -0.7269 --julia-cy 0.1889 -o julia.png

# Render Barnsley Fern with Emerald palette
fractal_gen render -f barnsley-fern -c emerald -o fern.png
```

### 2. "Surprise Me" AI Discovery

```bash
# Discover an interesting view for Mandelbrot and save it
fractal_gen surprise-me -f mandelbrot -o surprise.png

# Discover a view with 500 candidate evaluations
fractal_gen surprise-me -f julia --candidates 500 -o surprise_julia.png
```

### 3. List Fractals and Color Schemes

```bash
# List all 114 fractals grouped by family
fractal_gen list

# Filter by family
fractal_gen list -F attractor

# List all 48 color schemes
fractal_gen list --colors
```

### 4. Inspect Fractal Mathematical Information

```bash
fractal_gen info lorenz-attractor
fractal_gen info newton
fractal_gen info weierstrass
```

### 5. Launch Interactive Web Explorer

```bash
fractal_gen serve --port 8080
```
Then open `http://127.0.0.1:8080` in your web browser.

### 6. Benchmark Performance

```bash
fractal_gen benchmark -W 800 -H 600 -i 125
```

---

## Architecture

- [`src/core/`](file:///home/lnoxsian/lnox-files/project-rust/fractal_gen/src/core/): Complex numbers, mathematical coordinate mapping, configuration, interesting bounds, and parallel renderer.
- [`src/color/`](file:///home/lnoxsian/lnox-files/project-rust/fractal_gen/src/color/): 48 palette formulas, 512-entry lookup tables, and palette cache.
- [`src/fractals/`](file:///home/lnoxsian/lnox-files/project-rust/fractal_gen/src/fractals/): Registry and 16 family modules implementing all 114 fractal formulas.
- [`src/discovery/`](file:///home/lnoxsian/lnox-files/project-rust/fractal_gen/src/discovery/): "Surprise Me" candidate generation, validation, heuristic & neural net scoring, and favorites manager.
- [`src/server/`](file:///home/lnoxsian/lnox-files/project-rust/fractal_gen/src/server/): Embedded HTTP server and interactive Web UI.
