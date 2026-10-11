# FractalAI: Comprehensive Fractal, Formula, Color, and “Surprise Me” Guide

FractalAI is a browser-based WebGL fractal explorer built with JavaScript, Vite, `regl`, GLSL fragment shaders, and Synaptic.js. The repository currently registers **105 fractal types**, organized into dynamically loaded family modules.

The main implementation locations are:

- Fractal registry and family mapping: `static/js/fractals/loader.js`
- Mathematical descriptions: `static/js/fractals/fractal-info.js`
- Rendering utilities and color palettes: `static/js/fractals/utils.js`
- Individual GLSL implementations: `static/js/fractals/2d/`
- Machine-learning discovery: `static/js/discovery/`
- Discovery UI: `static/js/ui/discovery-ui.js`
- Global color list: `static/js/core/config.js`

> **Important distinction:** `fractal-info.js` contains human-readable mathematical descriptions, while the actual rendering behavior is implemented in individual GLSL shader files. Some metadata descriptions are generalized or simplified compared with the exact shader implementation.

---

## 1. How FractalAI Renders Fractals

Most 2D fractals follow this pipeline:

1. A full-screen WebGL quad is rendered.
2. Each pixel is converted into a mathematical coordinate.
3. A fractal-specific GLSL `computeFractal(vec2 c)` function evaluates that coordinate.
4. The returned iteration, density, distance, orbit, or intensity value is normalized.
5. The normalized value indexes a 512×1 color palette texture.
6. Points that remain inside an escape-time set are usually rendered black.

The coordinate conversion is performed in `static/js/fractals/utils.js`:

```text
scale = 4 / zoom

x = centeredPixelX × scale × aspectRatio × xScale + offsetX
y = centeredPixelY × scale × yScale + offsetY
```

The common color flow is:

```text
fractalValue
    → normalized value t in [0, 1]
    → palette lookup
    → RGB output
```

For escape-time fractals:

```text
t = clamp(iterationsReturned / maxIterations, 0, 1)
```

If the returned value reaches the maximum iteration count, the pixel is treated as belonging to the set and is generally forced to black.

---

# 2. Complete Fractal Inventory

The repository’s loader registers the following **105 fractal types**.

---

## A. Mandelbrot Family

Files are grouped in `static/js/fractals/2d/families/mandelbrot-family.js`.

### 1. Mandelbrot

Identifier:

```text
mandelbrot
```

Core formula:

\[
z_{n+1}=z_n^2+c
\]

with:

\[
z_0=0
\]

A point belongs to the Mandelbrot set if the sequence remains bounded. The shader uses an escape radius of 2, implemented as a squared escape radius of 4.

The shader also uses smooth coloring:

\[
\nu = \log_2(\log |z|)
\]

This prevents harsh bands between iteration levels.

Implementation:

`static/js/fractals/2d/mandelbrot.js`

---

### 2. Celtic Mandelbrot

Identifier:

```text
celtic-mandelbrot
```

Formula:

\[
z_{n+1} =
\left(
|\operatorname{Re}(z_n)|
+i|\operatorname{Im}(z_n)|
\right)^2+c
\]

The absolute values introduce additional symmetry and knot-like structures.

---

### 3. Multibrot

Identifier:

```text
multibrot
```

Formula:

\[
z_{n+1}=z_n^d+c
\]

where \(d\) is a configurable or shader-defined exponent. The classic Mandelbrot set is the special case \(d=2\).

As \(d\) changes, the familiar cardioid-and-bulbs shape becomes more circular, star-like, or polygonal.

---

### 4. Mutant Mandelbrot

Identifier:

```text
mutant-mandelbrot
```

General formula:

\[
z_{n+1}=z_n^2+c+\varepsilon
\]

where \(\varepsilon\) is a perturbation term.

The perturbation shifts or deforms the standard Mandelbrot structure.

---

### 5. Phoenix Mandelbrot

Identifier:

```text
phoenix-mandelbrot
```

Formula:

\[
z_{n+1}=z_n^2+c+pz_{n-1}
\]

This is a second-order recurrence because the current value depends on the previous iteration. The memory term \(pz_{n-1}\) produces flame-like, spiral, and phoenix-shaped structures.

---

### 6. Burning Ship

Identifier:

```text
burning-ship
```

Formula:

\[
z_{n+1}=
\left(
|\operatorname{Re}(z_n)|
+i|\operatorname{Im}(z_n)|
\right)^2+c
\]

The absolute values are applied before squaring. This produces the characteristic ship-like structures.

---

### 7. Tricorn

Identifier:

```text
tricorn
```

Formula:

\[
z_{n+1}=\overline{z_n}^{\,2}+c
\]

The complex conjugate changes the sign of the imaginary component:

\[
\overline{x+iy}=x-iy
\]

The shader therefore uses:

```text
z = vec2(zx² - zy², -2zxzy) + c
```

---

### 8. Nebulabrot

Identifier:

```text
nebulabrot
```

This is not simply a standard escape-time coloring method. It accumulates trajectories of escaping points.

Conceptually:

1. Choose a sample \(c\).
2. Iterate \(z_{n+1}=z_n^2+c\).
3. If the orbit escapes, record the positions visited by the orbit.
4. Accumulate those positions into a density image.

This creates cloud-, nebula-, and galaxy-like structures.

---

### 9. Buddhabrot

Identifier:

```text
buddhabrot
```

The Buddhabrot also accumulates escaping trajectories, but emphasizes the density of paths through each pixel.

Conceptually:

\[
P(z)=
\sum_{\text{escaping orbits passing through }z}1
\]

The implementation samples candidate complex parameters, tracks their orbits, and adds density when an orbit passes near the current pixel.

Implementation:

`static/js/fractals/2d/buddhabrot.js`

---

## B. Julia Family

Files are grouped in `static/js/fractals/2d/families/julia-family.js`.

Julia-style fractals use a fixed complex parameter \(c\), while \(z_0\) varies across the image.

---

### 10. Julia Set

Identifier:

```text
julia
```

Formula:

\[
z_{n+1}=z_n^2+c
\]

Difference from Mandelbrot:

- Mandelbrot: \(c\) varies and \(z_0=0\)
- Julia: \(c\) is fixed and \(z_0\) varies

FractalAI’s default Julia parameter is defined in `config.js`:

```text
c = -0.7269 + 0.1889i
```

---

### 11. Julia Snakes

Identifier:

```text
julia-snakes
```

Uses the standard Julia iteration:

\[
z_{n+1}=z_n^2+c
\]

The appearance comes from selected Julia parameters that produce snake-like filaments and branches.

---

### 12. Multibrot Julia

Identifier:

```text
multibrot-julia
```

Formula:

\[
z_{n+1}=z_n^d+c
\]

Here \(c\) remains fixed while \(z_0\) varies.

---

### 13. Burning Ship Julia

Identifier:

```text
burning-ship-julia
```

Formula:

\[
z_{n+1}=
\left(
|\operatorname{Re}(z_n)|
+i|\operatorname{Im}(z_n)|
\right)^2+c
\]

This is the Julia-set equivalent of Burning Ship.

---

### 14. Tricorn Julia

Identifier:

```text
tricorn-julia
```

Formula:

\[
z_{n+1}=\overline{z_n}^{\,2}+c
\]

This combines Julia dynamics with the Tricorn conjugation operation.

---

### 15. Phoenix Julia

Identifier:

```text
phoenix-julia
```

Formula:

\[
z_{n+1}=z_n^2+c+pz_{n-1}
\]

The fixed Julia parameter and the memory term generate flame and spiral patterns.

---

### 16. Lambda Julia

Identifier:

```text
lambda-julia
```

Formula:

\[
z_{n+1}=\lambda z_n(1-z_n)
\]

This is based on the complex logistic map.

The parameter \(\lambda\) controls whether the result is stable, periodic, chaotic, or fractal-like.

---

### 17. Hybrid Julia

Identifier:

```text
hybrid-julia
```

Uses a hybrid iteration strategy combining multiple transformations or Julia-style formulas.

The metadata describes this as:

```text
A hybrid iteration formula
```

The precise behavior should be read from:

```text
static/js/fractals/2d/hybrid-julia.js
```

---

### 18. Pickover Stalks

Identifier:

```text
pickover-stalks
```

This is primarily an orbit-visualization technique. It highlights points whose iterated trajectories approach or cross selected axes or regions.

The general basis is Julia-style iteration, but the color/intensity signal is derived from orbit behavior rather than only escape time.

---

### 19. Biomorphs

Identifier:

```text
biomorphs
```

Uses escape-time iteration, commonly of the form:

\[
z_{n+1}=z_n^d+c
\]

but modifies the termination or coloring conditions to produce biological or organism-like forms.

---

## C. Sierpinski and Recursive Subdivision Family

### 20. Sierpinski Triangle

Identifier:

```text
sierpinski
```

Construction:

1. Start with a triangle.
2. Divide it into four smaller triangles.
3. Remove the central inverted triangle.
4. Repeat recursively.

Dimension:

\[
D=\frac{\log 3}{\log 2}\approx1.585
\]

---

### 21. Sierpinski Arrowhead

Identifier:

```text
sierpinski-arrowhead
```

A continuous curve-based construction that traces the structure of the Sierpinski triangle.

It is generated using recursive turning and line subdivision rather than ordinary escape-time iteration.

---

### 22. Sierpinski Carpet

Identifier:

```text
sierpinski-carpet
```

Construction:

1. Start with a square.
2. Divide it into a 3×3 grid.
3. Remove the center square.
4. Repeat for the remaining eight squares.

Dimension:

\[
D=\frac{\log 8}{\log 3}\approx1.893
\]

---

### 23. Sierpinski Gasket

Identifier:

```text
sierpinski-gasket
```

A generalized Sierpinski construction using polygonal or triangular recursive subdivision.

---

### 24. Sierpinski Hexagon

Identifier:

```text
sierpinski-hexagon
```

Recursively subdivides hexagons and removes selected regions according to a repeating geometric rule.

---

### 25. Sierpinski Pentagon

Identifier:

```text
sierpinski-pentagon
```

Uses recursive pentagonal subdivision with five-fold symmetry.

---

### 26. Sierpinski L-System

Identifier:

```text
sierpinski-lsystem
```

L-system rules:

```text
Axiom: A
A → B-A-B
B → A+B+A
```

This produces a Sierpinski-like curve through string rewriting and turtle-graphics interpretation.

---

### 27. Sierpinski Tetrahedron

Identifier:

```text
sierpinski-tetrahedron
```

A 3D-style recursive tetrahedral construction rendered as a 2D projection.

Dimension:

\[
D=\frac{\log 4}{\log 2}=2
\]

---

### 28. Menger Carpet

Identifier:

```text
menger-carpet
```

A 2D representation of the Menger sponge construction.

Dimension:

\[
D=\frac{\log20}{\log3}\approx2.727
\]

---

### 29. Quadrilateral Subdivision

Identifier:

```text
quadrilateral-subdivision
```

Recursively subdivides squares or quadrilaterals and removes, transforms, or emphasizes central regions.

---

### 30. Recursive Polygon Splitting

Identifier:

```text
recursive-polygon-splitting
```

Recursively divides polygons into smaller polygons according to a subdivision rule.

---

### 31. Triangular Subdivision

Identifier:

```text
triangular-subdivision
```

Recursively subdivides triangles using a pattern related to, but not necessarily identical to, the classic Sierpinski triangle.

---

## D. Dragon Curves

### 32. Binary Dragon

Identifier:

```text
binary-dragon
```

Uses binary representation to decide curve turns.

Each binary digit determines a directional or rotational decision.

---

### 33. Dragon L-System

Identifier:

```text
dragon-lsystem
```

Rules:

```text
Axiom: FX
X → X+YF+
Y → -FX-Y
```

---

### 34. Folded Paper Dragon

Identifier:

```text
folded-paper-dragon
```

Constructed by repeatedly folding a strip of paper and unfolding it into right-angle turns.

---

### 35. Heighway Dragon

Identifier:

```text
heighway-dragon
```

The classic dragon curve produced through recursive paper-folding rules.

---

### 36. Lévy Dragon

Identifier:

```text
levy-dragon
```

Uses a 45-degree recursive replacement pattern and is related to the Lévy C curve.

---

### 37. Terdragon

Identifier:

```text
terdragon
```

A ternary or base-3 dragon curve. The turning structure is related to ternary digit representation.

---

### 38. Twindragon

Identifier:

```text
twindragon
```

Combines two symmetric dragon curves into a double-dragon arrangement.

---

## E. Space-Filling Curves

### 39. Gosper Curve

Identifier:

```text
gosper-curve
```

A recursive hexagonal space-filling curve.

Its construction is related to seven-way recursive subdivision.

---

### 40. Hilbert Curve

Identifier:

```text
hilbert-curve
```

A recursive curve that fills a square.

Each iteration replaces a square with four recursively oriented sub-squares.

---

### 41. Lévy C Curve

Identifier:

```text
levy-c-curve
```

Construction:

1. Start with a line segment.
2. Replace it with two segments at a right angle.
3. Apply the same operation recursively.

---

### 42. Moore Curve

Identifier:

```text
moore-curve
```

A closed-loop variation of the Hilbert curve.

---

### 43. Peano Curve

Identifier:

```text
peano-curve
```

The first classical space-filling curve, using 3×3 recursive subdivision.

---

### 44. Sierpinski Curve

Identifier:

```text
sierpinski-curve
```

A continuous path tracing the Sierpinski triangle.

---

## F. Root-Finding Fractals

### 45. Newton Fractal

Identifier:

```text
newton
```

Newton’s method:

\[
z_{n+1}
=
z_n-\frac{f(z_n)}{f'(z_n)}
\]

A common example is:

\[
f(z)=z^3-1
\]

Each starting point converges toward one of the polynomial’s roots. Colors identify the destination root, while brightness or iteration count can represent convergence speed.

---

### 46. Halley Fractal

Identifier:

```text
halley
```

Halley’s method:

\[
z_{n+1}
=
z_n-
\frac{2f(z_n)f'(z_n)}
{2[f'(z_n)]^2-f(z_n)f''(z_n)}
\]

It is a higher-order root-finding method that generally converges faster than Newton’s method.

---

### 47. Nova Fractal

Identifier:

```text
nova
```

A modified Newton iteration with memory:

\[
z_{n+1}
=
z_n
-\alpha\frac{f(z_n)}{f'(z_n)}
+\beta(z_n-z_{n-1})
\]

The memory term creates additional spirals and explosive structures.

---

## G. Plant and Tree Fractals

### 48. Plant

Identifier:

```text
plant
```

Uses L-system or IFS-style rules to create branching plant structures.

---

### 49. Barnsley Fern

Identifier:

```text
barnsley-fern
```

Uses an iterated function system consisting of affine transformations:

\[
\begin{pmatrix}x'\\y'\end{pmatrix}
=
\begin{pmatrix}a_i&b_i\\c_i&d_i\end{pmatrix}
\begin{pmatrix}x\\y\end{pmatrix}
+
\begin{pmatrix}e_i\\f_i\end{pmatrix}
\]

Each transformation is selected with a probability.

---

### 50. Fractal Tree

Identifier:

```text
fractal-tree
```

Recursive binary branching:

```text
branch(length, angle):
    draw branch
    branch(smallerLength, angle - delta)
    branch(smallerLength, angle + delta)
```

---

### 51. Pythagoras Tree

Identifier:

```text
pythagoras-tree
```

Starts with a square and recursively attaches two smaller squares arranged around a right triangle.

---

### 52. Wide Pythagoras Tree

Identifier:

```text
pythagoras-tree-wide
```

A Pythagoras tree with wider branch angles.

---

### 53. Narrow Pythagoras Tree

Identifier:

```text
pythagoras-tree-narrow
```

A compact Pythagoras tree with narrower branch angles.

---

### 54. Binary Fractal Tree

Identifier:

```text
binary-fractal-tree
```

Each branch creates two child branches.

---

### 55. Ternary Fractal Tree

Identifier:

```text
ternary-fractal-tree
```

Each branch creates three child branches.

---

### 56. Quaternary Fractal Tree

Identifier:

```text
quaternary-fractal-tree
```

Each branch creates four child branches.

---

### 57. Oak L-System Tree

Identifier:

```text
lsystem-tree-oak
```

Uses oak-like L-system branching rules.

---

### 58. Pine L-System Tree

Identifier:

```text
lsystem-tree-pine
```

Uses conical, pine-like L-system branching rules.

---

### 59. Fractal Canopy

Identifier:

```text
fractal-canopy
```

Focuses on recursively generated branching in the upper canopy of a tree.

---

## H. Koch Family

### 60. Fractal Islands

Identifier:

```text
fractal-islands
```

Uses Koch-style recursive boundary deformation to create island and coastline shapes.

---

### 61. Koch Snowflake

Identifier:

```text
koch
```

Construction:

1. Start with an equilateral triangle.
2. Divide each side into three parts.
3. Replace the middle third with two segments forming a triangular bump.
4. Repeat recursively.

Dimension:

\[
D=\frac{\log4}{\log3}\approx1.262
\]

Perimeter:

\[
P_n=P_0\left(\frac43\right)^n
\]

---

### 62. Quadratic Koch

Identifier:

```text
quadratic-koch
```

Uses square-based or quadratic replacement rules.

Dimension:

\[
D=\frac{\log8}{\log4}=1.5
\]

---

## I. Cantor Family

### 63. Cantor Set

Identifier:

```text
cantor
```

Construction:

1. Start with \([0,1]\).
2. Remove the middle third.
3. Repeat on the two remaining intervals.

Dimension:

\[
D=\frac{\log2}{\log3}\approx0.631
\]

---

### 64. Cantor Dust by Base Expansion

Identifier:

```text
cantor-dust-base-expansion
```

Uses positional-number representations to determine which regions remain or are removed.

---

### 65. Circular Cantor Dust

Identifier:

```text
cantor-dust-circular
```

Extends Cantor-style removal into circular regions, producing a 2D dust-like structure.

---

### 66. Fat Cantor Set

Identifier:

```text
fat-cantor
```

Removes smaller middle portions than the ordinary Cantor construction, preserving positive measure.

---

### 67. Smith–Volterra–Cantor Set

Identifier:

```text
smith-volterra-cantor
```

A positive-measure Cantor-like set created by progressively removing smaller intervals.

---

### 68. Random Cantor Set

Identifier:

```text
random-cantor
```

Randomizes the size or position of removed sections.

Because it is stochastic, repeated renders may not be identical.

---

## J. Tilings

### 69. Domino Substitution

Identifier:

```text
domino-substitution
```

Recursively replaces domino tiles according to substitution rules.

---

### 70. Pinwheel Tiling

Identifier:

```text
pinwheel-tiling
```

Uses right triangles in multiple orientations to create an aperiodic tiling.

---

### 71. Snowflake Tiling

Identifier:

```text
snowflake-tiling
```

Uses recursive substitution to generate snowflake-like tile motifs.

---

### 72. Ammann Tiling

Identifier:

```text
amman-tiling
```

Uses Ammann-style tile shapes and matching rules.

---

### 73. Penrose Substitution

Identifier:

```text
penrose-substitution
```

Uses kite-and-dart or rhombus tile substitution rules.

The golden ratio is central to Penrose tilings:

\[
\varphi=\frac{1+\sqrt5}{2}
\]

---

### 74. Rauzy Fractal

Identifier:

```text
rauzy
```

A substitution-system fractal related to numeration and tiling theory.

---

### 75. Chair Tiling

Identifier:

```text
chair-tiling
```

Uses recursively substituted chair-shaped tiles.

---

### 76. Rhombic Tiling

Identifier:

```text
rhombic-tiling
```

Uses rhombus-shaped tiles with recursive or aperiodic placement rules.

---

### 77. Aperiodic Tilings

Identifier:

```text
aperiodic-tilings
```

Tilings that cover the plane without repeating under translation.

---

### 78. Substitution Tilings

Identifier:

```text
substitution-tilings
```

A general framework where each tile is replaced with smaller tiles according to a recursive grammar.

---

## K. Attractors and Dynamical Systems

These are grouped in:

```text
static/js/fractals/2d/families/attractor-family.js
```

### 79. Lorenz Attractor

Identifier:

```text
lorenz-attractor
```

\[
\frac{dx}{dt}=\sigma(y-x)
\]

\[
\frac{dy}{dt}=x(\rho-z)-y
\]

\[
\frac{dz}{dt}=xy-\beta z
\]

The system is integrated numerically to generate a chaotic trajectory.

---

### 80. Rössler Attractor

Identifier:

```text
rossler-attractor
```

\[
\frac{dx}{dt}=-y-z
\]

\[
\frac{dy}{dt}=x+ay
\]

\[
\frac{dz}{dt}=b+z(x-c)
\]

---

### 81. Lyapunov Fractal

Identifier:

```text
lyapunov
```

Based on the Lyapunov exponent:

\[
\lambda=
\lim_{n\to\infty}
\frac1n
\sum_{i=0}^{n-1}\ln|f'(x_i)|
\]

Positive values indicate local chaotic divergence.

---

### 82. Chua Attractor

Identifier:

```text
chua-attractor
```

\[
\frac{dx}{dt}=\alpha(y-x-f(x))
\]

\[
\frac{dy}{dt}=x-y+z
\]

\[
\frac{dz}{dt}=-\beta y
\]

where \(f(x)\) is a piecewise-linear nonlinear function.

---

### 83. IFS Spiral

Identifier:

```text
ifs-spiral
```

Uses affine transformations selected repeatedly to generate spiral-like attractors.

---

### 84. IFS Maple

Identifier:

```text
ifs-maple
```

An affine iterated function system designed to produce maple-leaf-like shapes.

---

### 85. IFS Tree

Identifier:

```text
ifs-tree
```

Uses affine transformations arranged to create branching tree structures.

---

## L. Noise and Procedural Terrain

### 86. Perlin Noise

Identifier:

```text
perlin-noise
```

Gradient noise created by interpolating pseudo-random gradients across a lattice.

---

### 87. Simplex Noise

Identifier:

```text
simplex-noise
```

An alternative to Perlin noise that uses simplex cells instead of square or cubic lattice cells.

---

### 88. Fractional Brownian Motion

Identifier:

```text
fractional-brownian-motion
```

A generalized Brownian process controlled by a Hurst exponent \(H\):

\[
B_H(t)
=
\frac{1}{\Gamma(H+\frac12)}
\int_{-\infty}^{t}
(t-s)^{H-\frac12}\,dB(s)
\]

A common relationship between dimension and Hurst exponent is:

\[
D=2-H
\]

---

### 89. Random Midpoint Displacement

Identifier:

```text
random-midpoint-displacement
```

Algorithm:

1. Start with endpoints.
2. Find the midpoint.
3. Add random displacement.
4. Reduce displacement amplitude.
5. Repeat recursively.

This is useful for terrain-like or mountain-like forms.

---

## M. Physics-Based and Random Growth

### 90. Diffusion-Limited Aggregation

Identifier:

```text
diffusion-limited-aggregation
```

Particles perform random walks and stick when they touch an existing cluster.

The resulting cluster typically has an approximate dimension near:

\[
D\approx1.7
\]

---

### 91. Percolation Cluster

Identifier:

```text
percolation-cluster
```

Sites or bonds are occupied randomly with probability \(p\).

For a square lattice, the critical probability is approximately:

\[
p_c\approx0.5927
\]

---

### 92. Lévy Flights

Identifier:

```text
levy-flights
```

Step lengths follow a heavy-tailed power law:

\[
P(\ell)\sim\ell^{-\alpha}
\]

where \(0<\alpha<2\).

---

## N. Geometric Fractals

### 93. Apollonian Gasket

Identifier:

```text
apollonian-gasket
```

Uses recursively tangent circles governed by Descartes’ circle theorem:

\[
2(\kappa_1^2+\kappa_2^2+\kappa_3^2+\kappa_4^2)
=
(\kappa_1+\kappa_2+\kappa_3+\kappa_4)^2
\]

where:

\[
\kappa=\frac1r
\]

---

### 94. Carpenter Square / T-Square

Identifier:

```text
carpenter-square
```

Starts with a square and recursively attaches smaller squares to its sides.

---

### 95. Cross

Identifier:

```text
cross
```

Recursively replaces cross arms with smaller crosses.

---

### 96. Box Variants

Identifier:

```text
box-variants
```

A collection of box-based recursive subdivision patterns.

---

### 97. Minkowski Sausage

Identifier:

```text
minkowski-sausage
```

Thickens a curve by including every point within distance \(r\).

For a simple curve of length \(L\):

\[
A\approx2rL+\pi r^2
\]

---

### 98. Cesàro Fractal

Identifier:

```text
cesaro
```

A Koch-like recursive curve that uses arcs or smoothed replacements instead of only straight segments.

---

### 99. Recursive Circle Removal

Identifier:

```text
recursive-circle-removal
```

Recursively removes or inserts circles according to a geometric rule similar to Apollonian constructions.

---

### 100. Rose Window

Identifier:

```text
rose
```

Based on the polar rose curve:

\[
r=a\cos(k\theta)
\]

or:

\[
r=a\sin(k\theta)
\]

The number of petals depends on \(k\).

---

### 101. Menger Sponge

Identifier:

```text
menger-sponge
```

3D recursive cube removal:

1. Divide a cube into 27 smaller cubes.
2. Remove the central cube and face-center cubes.
3. Repeat on the remaining 20 cubes.

Dimension:

\[
D=\frac{\log20}{\log3}\approx2.727
\]

---

## O. Self-Similar and Nowhere-Differentiable Functions

### 102. De Rham Curve

Identifier:

```text
de-rham-curve
```

A recursively constructed continuous, often nowhere-differentiable curve.

---

### 103. Weierstrass Function

Identifier:

```text
weierstrass
```

\[
f(x)=
\sum_{n=0}^{\infty}
a^n\cos(b^n\pi x)
\]

for suitable \(a\) and odd integer \(b\).

This produces a continuous but nowhere-differentiable function.

---

### 104. Takagi Function

Identifier:

```text
takagi
```

\[
T(x)=
\sum_{n=0}^{\infty}
\frac{1}{2^n}
\operatorname{dist}(2^nx,\mathbb Z)
\]

The shader maps the x-coordinate into \([0,1]\), calculates the Takagi sum, and renders a narrow band around the resulting curve.

Implementation:

`static/js/fractals/2d/takagi.js`

---

### 105. Blancmange Curve

Identifier:

```text
blancmange
```

\[
B(x)=
\sum_{n=0}^{\infty}
\frac{s(2^nx)}{2^n}
\]

where \(s(x)\) is a sawtooth or distance-to-nearest-integer function.

---

## P. Other Escape-Time and Procedural Fractals

### Buffalo

Identifier:

```text
buffalo
```

Formula:

\[
z_{n+1}
=
\left(
|\operatorname{Re}(z_n)|
-
|\operatorname{Im}(z_n)|
\right)^2+c
\]

---

### Popcorn

Identifier:

```text
popcorn
```

The metadata gives a simplified tangent-based formula, while the shader uses a two-dimensional Popcorn map similar to:

\[
x_{n+1}=x_n-h\sin(y_n+\tan(ay_n))
\]

\[
y_{n+1}=y_n-h\sin(x_n+\tan(bx_n))
\]

The shader clamps tangent values to prevent numerical explosions and colors bounded points using orbit length.

Implementation:

`static/js/fractals/2d/popcorn.js`

---

### Spider Set

Identifier:

```text
spider-set
```

Formula:

\[
z_{n+1}=z_n^2+c+\frac{p}{z_n}
\]

The reciprocal term creates web- or spider-like structures.

---

### Magnet Fractal

Identifier:

```text
magnet
```

Formula:

\[
z_{n+1}
=
\frac{(z_n^2+c-1)^2}
{4z_n(z_n^2+c-2)}
\]

This is a rational iteration that produces magnetic-field-like structures.

---

### Fractal Dimension Plot

Identifier:

```text
fractal-dimension-plot
```

Based on the Hausdorff dimension:

\[
D_H=
\inf\{d:H^d(E)=0\}
\]

This is primarily a visualization of dimension rather than a conventional iterative fractal.

---

### H-Tree

Identifier:

```text
h-tree
```

Starts with a line segment and recursively attaches perpendicular segments to form H shapes.

---

### Generalized H-Tree

Identifier:

```text
h-tree-generalized
```

Extends the H-tree by varying branching angles, branch counts, or scaling factors.

---

### Vicsek Snowflake

Identifier:

```text
vicsek
```

Construction:

1. Start with a square.
2. Divide it into a 3×3 grid.
3. Keep the center and four corners.
4. Repeat recursively.

Dimension:

\[
D=\frac{\log5}{\log3}\approx1.465
\]

---

### Fractal Flame

Identifier:

```text
fractal-flame
```

Uses nonlinear iterated function systems and color mapping.

The general model is:

\[
x_{n+1}=F(x_n)
\]

where \(F\) may include nonlinear variations such as sinusoidal, spherical, swirl, or other transformations.

---

# 3. Color Schemes

FractalAI defines **48 built-in color schemes**, indexed from 0 to 47.

The master list is in:

```text
static/js/core/config.js
```

The color implementation is in:

```text
static/js/fractals/utils.js
```

Color schemes are generated by `computeColorForScheme(t, scheme)` where:

```text
t ∈ [0, 1]
```

represents normalized fractal intensity, escape time, density, or orbit value.

The palette is precomputed into a 512×1 RGBA WebGL texture and then sampled by the fragment shader.

---

## Complete Color Scheme List

| Index | Scheme | General appearance |
|---:|---|---|
| 0 | `classic` | Green-to-cyan-blue gradient |
| 1 | `fire` | Red, orange, and dark tones |
| 2 | `ocean` | Blue and cyan |
| 3 | `rainbow` | Full cosine-based spectrum |
| 4 | `monochrome` | Grayscale |
| 5 | `forest` | Green |
| 6 | `sunset` | Red, orange, warm brown |
| 7 | `purple` | Purple and violet |
| 8 | `cyan` | Cyan and turquoise |
| 9 | `gold` | Gold and yellow |
| 10 | `ice` | Pale blue and white |
| 11 | `neon` | Pulsing bright neon tones |
| 12 | `rainbow-pastel` | Soft pastel rainbow |
| 13 | `rainbow-dark` | Dark saturated rainbow |
| 14 | `rainbow-vibrant` | High-saturation rainbow |
| 15 | `rainbow-double` | Two-cycle rainbow across the range |
| 16 | `rainbow-shifted` | Rainbow with hue rotation |
| 17 | `cosmic` | Dark purple/blue to cyan-white |
| 18 | `aurora` | Green, cyan, blue, purple |
| 19 | `coral` | Deep blue, teal, coral-pink |
| 20 | `autumn` | Red, orange, gold |
| 21 | `midnight` | Navy, purple, magenta |
| 22 | `emerald` | Dark green, emerald, gold |
| 23 | `rosegold` | Pink, rose, gold |
| 24 | `electric` | Bright complementary neon colors |
| 25 | `vintage` | Muted pastel/sepia colors |
| 26 | `tropical` | Cyan, magenta, yellow |
| 27 | `galaxy` | Indigo, violet, bright star-like highlights |
| 28 | `lava` | Dark red, orange, yellow-white |
| 29 | `arctic` | Blue, cyan, white |
| 30 | `sakura` | Cherry-blossom pinks |
| 31 | `volcanic` | Black, dark red, orange |
| 32 | `mint` | Green, mint, teal |
| 33 | `sunrise` | Pink, orange, yellow |
| 34 | `steel` | Metallic gray-blue |
| 35 | `prism` | Inverted complementary rainbow |
| 36 | `mystic` | Purple, magenta, pink with sparkle |
| 37 | `amber` | Brown, amber, golden yellow |
| 38 | `noir-ember` | Near-black to ember red and white |
| 39 | `ultraviolet` | Black, violet, electric cyan |
| 40 | `toxic` | Black, neon green, sulfur yellow |
| 41 | `blood-moon` | Black, crimson, magenta |
| 42 | `void-ice` | Black, deep blue, icy white |
| 43 | `acid-rain` | Teal, acid green, mint |
| 44 | `solar-flare` | Black, orange, yellow-white |
| 45 | `deep-sea` | Navy, teal, bioluminescent cyan |
| 46 | `chiaroscuro` | High-contrast grayscale with blue tint |
| 47 | `spectral-night` | Dark, saturated rainbow |

---

## Examples of Palette Formulas

### Classic

```text
R = 0.5t
G = t
B = min(1.5t, 1)
```

### Fire

```text
R = t
G = 0.5t
B = 0
```

### Ocean

```text
R = 0
G = 0.5t
B = t
```

### Monochrome

```text
R = G = B = t
```

### Forest

```text
R = 0.3t
G = 0.8t
B = 0.4t
```

### Rainbow

The RGB channels are generated with phase-shifted cosine waves:

\[
R=0.5+0.5\cos(\theta)
\]

\[
G=0.5+0.5\cos(\theta+2.09)
\]

\[
B=0.5+0.5\cos(\theta+4.18)
\]

### Neon

The palette adds a sinusoidal pulse:

\[
p=\frac{\sin(\pi t)+1}{2}
\]

and mixes that pulse into the RGB channels.

### Cosmic and Galaxy

These use dark bases, staged interpolation, and sinusoidal sparkle modulation to make high-intensity regions look like stars or nebulae.

---

## Palette Processing Pipeline

The palette is generated once per scheme:

```text
1. Iterate i from 0 to 511
2. Calculate t = i / 511
3. Compute RGB color
4. Store RGBA bytes
5. Upload as a 512×1 texture
6. Cache the texture
```

The cache is keyed by:

```text
colorScheme + palette size
```

This avoids regenerating the same palette for every frame.

---

# 4. How the “Surprise Me” Function Works

The user-facing button is wired in:

```text
static/js/ui/discovery-ui.js
```

The main algorithm is in:

```text
static/js/discovery/discovery-algorithm.js
```

The orchestration layer is:

```text
static/js/discovery/discovery-manager.js
```

---

## High-Level Behavior

The function does **not** choose a completely random fractal type.

Instead, it:

1. Keeps the currently selected fractal type.
2. Loads that fractal’s configuration and interesting bounds.
3. Generates 200 random candidate views.
4. Randomizes position, zoom, Julia parameters, and color scheme.
5. Rejects or downgrades candidates that are not interesting.
6. Scores surviving candidates heuristically.
7. Uses the ML model to refine scores when available.
8. Keeps candidates scoring at least `0.4`.
9. Sorts candidates by score.
10. Applies the best candidate.

Therefore, if the current fractal is Mandelbrot, “Surprise Me” finds an interesting Mandelbrot view. It does not necessarily switch to Julia, Lorenz, or another family.

---

## Step 1: The UI Handler Runs

The button listener calls:

```javascript
handleSurpriseMe()
```

Both the normal and fullscreen Surprise Me buttons use the same handler.

The handler:

- Adds a loading class.
- Disables the buttons.
- Reads the current fractal type.
- Requests a discovered configuration.
- Applies the configuration through `onFractalStateChange`.
- Restores the button state in a `finally` block.

The current settings are:

```javascript
{
  candidateCount: 200,
  minScore: 0.4
}
```

---

## Step 2: The Current Fractal Module Is Loaded

`getSurpriseMeFractal()` ensures that the current fractal module is loaded.

The loader first checks its cache. If the fractal is not cached, it dynamically imports the appropriate family module.

For example:

```text
mandelbrot → mandelbrot-family.js
julia → julia-family.js
lorenz-attractor → attractor-family.js
```

The fractal module may define:

```javascript
interestingBounds: {
  offsetX: [...],
  offsetY: [...],
  zoom: [...]
}
```

For example, Mandelbrot defines:

```javascript
interestingBounds: {
  offsetX: [-2.5, 1.5],
  offsetY: [-2, 2],
  zoom: [0.5, 100]
}
```

These bounds prevent the discovery system from searching completely arbitrary coordinates.

---

## Step 3: Candidate Configurations Are Generated

The algorithm calls:

```javascript
generateCandidates(fractalType, options, fractalModule)
```

For each candidate, it generates:

```javascript
{
  fractalType,
  zoom,
  offsetX,
  offsetY,
  iterations,
  colorScheme,
  xScale,
  yScale,
  juliaCX,
  juliaCY
}
```

The default candidate iteration count is:

```text
iterations = 125
```

For each candidate:

### Zoom

\[
\text{zoom}
=
\text{minZoom}
+
r(\text{maxZoom}-\text{minZoom})
\]

where \(r\) is a random number between 0 and 1.

### Offset

The x and y offsets are independently sampled from the fractal’s bounds.

### Color scheme

If color variation is enabled, the algorithm randomly selects from all available schemes:

```javascript
includeColorScheme: true
```

This means Surprise Me can change the color scheme as well as the location.

### Julia parameters

For Julia-type fractals, the algorithm additionally randomizes:

```text
juliaCX
juliaCY
```

using the configured Julia bounds.

---

## Step 4: Candidate Validation

Each candidate is passed to:

```javascript
isValidInterestingView(offset, zoom, fractalType)
```

This callback is supplied by the UI layer and ultimately uses:

```text
static/js/fractals/random-view.js
```

The purpose is to identify views that are not blank, trivial, invalid, or otherwise visually uninteresting.

If a candidate fails validation, the heuristic score is immediately reduced to:

```text
0.1
```

This is only a score penalty; it is not necessarily removed until the final minimum-score filtering stage.

---

## Step 5: Heuristic Scoring

Every candidate starts with:

```text
score = 0.5
```

The heuristic system then applies bonuses and penalties.

### Interesting-view validation

If the view is valid:

```text
score += 0.2
```

If it is invalid:

```text
score = 0.1
```

### Zoom preference

The algorithm prefers moderate zoom values:

```javascript
zoomScore = 1 - Math.abs(Math.log10(zoom + 1) - 1) / 2
score += zoomScore * 0.15
```

This avoids always selecting either the entire fractal overview or extremely deep zooms.

### Distance from origin

Candidates near the origin receive a bonus:

```text
if distanceFromOrigin < 2.5:
    score += 0.1
```

where:

\[
d=\sqrt{x^2+y^2}
\]

### Iteration preference

The preferred iteration count is approximately 125:

```javascript
iterScore = 1 - Math.abs(iterations - 125) / 125
score += iterScore * 0.05
```

The score is finally clamped to the range:

```text
[0, 1]
```

---

## Step 6: ML Feature Extraction

The system converts each candidate into an 11-value feature vector.

The features are:

1. Hashed fractal type
2. Log-scaled zoom
3. Normalized x offset
4. Normalized y offset
5. Normalized iteration count
6. Normalized x scale
7. Normalized y scale
8. Julia real parameter
9. Julia imaginary parameter
10. Distance from origin
11. Zoom category

Conceptually:

```text
[
  typeHash,
  logZoom,
  offsetX,
  offsetY,
  iterations,
  xScale,
  yScale,
  juliaCX,
  juliaCY,
  distanceFromOrigin,
  zoomCategory
]
```

The fractal type is encoded using a simple character-code sum:

```javascript
fractalType.split('').reduce(
  (acc, char) => acc + char.charCodeAt(0),
  0
)
```

This is not a semantic embedding. For example, similarly named fractals do not necessarily receive similar representations.

---

## Step 7: Neural Network Scoring

When enough favorites exist, FractalAI trains a Synaptic.js neural network:

```text
11 inputs
→ 32 hidden neurons
→ 16 hidden neurons
→ 1 output
```

The output is interpreted as a preference score between 0 and 1.

The model is trained with:

- Positive examples: the user’s favorites
- Negative examples: randomly generated configurations
- Positive output: `1.0`
- Negative output: `0.0`
- Learning rate: `0.001`
- Error threshold: `0.005`
- Cross-entropy cost

The model is stored in browser `localStorage` under:

```text
fractalai_ml_model
```

---

## Step 8: Hybrid Scoring

The heuristic and ML scores are combined as follows.

Without an ML model:

```text
finalScore = heuristicScore
```

With an ML model:

```text
finalScore =
    0.3 × heuristicScore
  + 0.7 × mlScore
```

The heuristic is always run first.

If the heuristic score is below `0.3`, the model is skipped:

```javascript
if (heuristic < 0.3) {
  return heuristic;
}
```

This is a performance optimization that prevents poor candidates from consuming ML evaluation time.

---

## Step 9: Filtering and Ranking

After all 200 candidates are scored:

```javascript
filtered = scored
  .filter(candidate => candidate.score >= 0.4)
  .sort((a, b) => b.score - a.score)
  .slice(0, 1)
```

Only the highest-scoring candidate is returned.

If no candidate reaches the minimum score:

```text
null
```

is returned and the UI logs:

```text
No interesting fractals discovered
```

---

## Step 10: Applying the Result

If a candidate is found, it is passed to:

```javascript
onFractalStateChange(discovered)
```

That applies:

- fractal type
- zoom
- x/y position
- iterations
- color scheme
- x/y scale
- Julia parameters, when applicable

The rendering engine then redraws the fractal.

---

# 5. How Favorites Affect Surprise Me

Favorites are stored locally under:

```text
fractalai_favorites
```

A favorite captures values such as:

```javascript
{
  fractalType,
  zoom,
  offsetX,
  offsetY,
  iterations,
  colorScheme,
  xScale,
  yScale,
  juliaCX,
  juliaCY
}
```

The model begins meaningful training only after at least:

```text
5 favorites
```

For each favorite:

```text
positive training example → output 1
```

The system also creates twice as many random negative samples:

```text
negative samples = favorites.length × 2
```

This teaches the model a rough distinction between configurations the user liked and random alternatives.

---

## Model Retraining

The model may retrain when:

1. The favorite count increases by more than 20%.
2. The model is older than seven days.
3. No saved model exists.
4. The model version changes.

Retraining is performed in the background after discovery initialization so that it does not block application startup.

---

# 6. Strengths and Limitations of Surprise Me

## Strengths

- Uses fractal-specific search bounds.
- Supports color scheme variation.
- Supports Julia parameter variation.
- Combines deterministic heuristics with learned preferences.
- Does not require a server or external API.
- Stores favorites and the model locally.
- Evaluates many candidates per button click.

## Limitations

### It does not directly compare rendered screenshots

The algorithm scores numerical configuration features and validation results. It does not appear to run a full image-aesthetic analysis over every candidate.

### The ML model is relatively simple

The fractal type is encoded by a character-code hash rather than a learned categorical embedding. This makes the model lightweight, but limits semantic understanding.

### Randomness is not seeded

Candidates use `Math.random()`. Pressing Surprise Me twice can produce different results even with the same favorites and current fractal.

### It searches the current fractal only

The current `fractalType` is passed into candidate generation. Surprise Me changes the view of the current fractal rather than selecting from all 105 fractal types.

### Color preference is only indirectly learned

The feature extractor destructures `colorScheme` but intentionally ignores it:

```javascript
colorScheme: _colorScheme
```

As a result, the ML feature vector does not directly encode the selected color scheme. Color schemes can still be randomized during candidate generation, but the model cannot strongly learn “the user prefers cosmic colors” from the feature vector alone.

### Negative examples are broadly random

Negative examples use general ranges such as:

```text
zoom: 0.1–100
offsets: -2–2
iterations: 10–399
```

They are not necessarily known-bad visual configurations.

---

# 7. Important Implementation Details

## Dynamic Loading

Fractals are grouped into family chunks for code splitting.

The loader first attempts:

```text
./2d/families/<family>.js
```

and then falls back to:

```text
./2d/<fractalType>.js
```

If loading fails, the loader falls back to Mandelbrot.

---

## Shared Rendering Infrastructure

Most fractals use:

```javascript
createFragmentShader(...)
createStandardDrawCommand(...)
```

from `static/js/fractals/utils.js`.

This provides:

- shared vertex shader
- common coordinate mapping
- common uniforms
- palette texture lookup
- WebGL1/WebGL2 support
- optional uniform buffer support
- shader and palette caching

---

## Performance Techniques

The project uses:

- unrolled first iterations for common escape-time fractals
- squared magnitude comparisons instead of square roots
- smooth escape-time coloring
- persistent full-screen quad buffers
- cached palette textures
- cached shader factories
- WebGL2 immutable texture storage
- optional uniform buffer objects
- dynamic family imports
- adaptive quality
- progressive rendering
- worker-based rendering
- optional WebGPU support

---

# 8. Key Files for Further Investigation

| Purpose | File |
|---|---|
| All registered fractal identifiers | `static/js/fractals/loader.js` |
| Mathematical descriptions | `static/js/fractals/fractal-info.js` |
| Initial positions and Wikipedia links | `static/js/fractals/fractal-config.js` |
| Rendering and colors | `static/js/fractals/utils.js` |
| Global color scheme names | `static/js/core/config.js` |
| Surprise Me UI | `static/js/ui/discovery-ui.js` |
| Discovery orchestration | `static/js/discovery/discovery-manager.js` |
| Candidate generation and scoring | `static/js/discovery/discovery-algorithm.js` |
| ML training | `static/js/discovery/ml-trainer.js` |
| Favorites storage | `static/js/discovery/favorites-manager.js` |
| Mandelbrot shader | `static/js/fractals/2d/mandelbrot.js` |
| Buddhabrot shader | `static/js/fractals/2d/buddhabrot.js` |
| Popcorn shader | `static/js/fractals/2d/popcorn.js` |
| Takagi shader | `static/js/fractals/2d/takagi.js` |

---

# 9. Summary

FractalAI contains **105 registered fractal types** spanning:

- Mandelbrot and Julia escape-time systems
- recursive subdivision fractals
- L-systems
- dragon and space-filling curves
- root-finding basins
- trees and plants
- tilings
- chaotic attractors
- procedural noise
- random growth systems
- geometric constructions
- nowhere-differentiable functions
- orbit-density renderers such as Buddhabrot and Nebulabrot

It includes **48 color schemes**, generated mathematically and uploaded as cached WebGL palette textures.

The **Surprise Me** system:

1. Keeps the current fractal type.
2. Generates 200 random configurations.
3. Samples zoom, position, color, and Julia parameters.
4. Uses fractal-specific bounds.
5. Scores candidates with heuristics.
6. Optionally refines scores using a Synaptic.js neural network trained on favorites.
7. Filters candidates using a minimum score of `0.4`.
8. Applies the highest-scoring result.

The feature is therefore best understood as a **personalized stochastic search over visual parameter space**, not as a universal fractal recommender or image-understanding AI.
