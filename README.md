# Holographic Infinity Phoenix (HIP)

HIP is an experimental Rust workspace for exploring bounded modular
arithmetic, weighted-graph operators, and asynchronous message passing. The
included code is a prototype, not a validated implementation of the
HAN/MERA/ERG claims in the source notes.

## Workspace

- `hip-algebra`: quaternion-style modular arithmetic over the configured
  residue modulus. This is **not** a quaternion division algebra: nonzero
  zero-norm elements exist, and `inverse()` returns an error for them.
- `hip-topology`: a weighted undirected graph, its symmetric normalized
  Laplacian, and connected-component counting.
- `hip-mera`: checked conjugation and modular averaging operators.
- `hip-runtime`: deterministic, bounded Tokio actors and a repeatable packet
  loss simulation. Logical transitions do not depend on wall-clock sleeps.
- `hip-cli`: a small simulation harness.

The algebra, MERA, and runtime crates expose execution traits with functional
public mock backends. Enable the workspace `enterprise-core` feature to expose
the adapter hooks under each crate's `enterprise` module. This feature adds no
private dependencies or proprietary implementation; downstream private crates
can implement the public traits independently.

There is no safe-prime rotation, cryptographic mechanism, persistent homology,
Ricci flow, Wetterich solver, or proof of fault tolerance in this prototype.
`tests/ricci_betti_homology.rs` checks connected components under a graph
change; its filename is historical and does not imply a homology guarantee.
See [the Via Negativa audit](docs/via_negativa_audit.md) for the mathematical
limitations and the checked zero-norm counterexample.

## Build and test

With a stable Rust toolchain installed:

```text
cargo test --workspace
cargo run -p hip-cli -- 100
```

The CLI argument is the number of logical simulation steps (default: 100).
For a reproducible 90% drop pattern, call `run_simulation` with `Some(10)`.

## Licensing

HIP is available under the GNU Affero General Public License, version 3
(AGPL-3.0-only); see [LICENSE](LICENSE). Network use of a modified version is
subject to the AGPL's requirements. A separate commercial license is required
for proprietary integration without AGPL copyleft obligations; see
[COMMERCIAL_LICENSING.md](COMMERCIAL_LICENSING.md). No commercial license is
granted by this repository's notice alone.
