# Math Core Documentation

## Scope
This document describes the computation flow implemented in `src/bindings.rs`.
It is a behavior-preserving Rust port of the original Fortran routines.

Public entry point:
- `norfil2(...) -> Results`

Internal core routines:
- `norecfil(...)`: top-level dispatcher
- `norfil(...)`: non-recursive filter branch
- `recfil(...)`: recursive filter branch
- `n1afds(...)`: random band-limited signal generator (operation mode helper)
- `n1yfbe(...)`: Bessel approximation helper
- `n1arnd(...)`: linear congruential RNG helper

Readability helpers (same behavior, clearer intent):
- `nonrecursive_band_params(...)`
- `window_shape_constants(...)`
- `compute_relative_errors(...)`
- `compute_shifted_relative_errors(...)`
- `fill_operation_signals(...)`
- `setup_design_mode_ranges(...)`
- `setup_operation_mode_lengths(...)`
- `mirror_nonrecursive_kernel(...)`

## Parameter Modes
These integer flags are used exactly as in the old code:

- `iw` (mode):
  - `0`: design mode (frequency and design characteristics)
  - `1`: operation mode (signal processing and error metrics)
- `il` (filter implementation):
  - `1`: non-recursive
  - `2`: recursive
- `in_` (filter class):
  - `1`: low-pass
  - `2`: band-pass
  - `3`: band-stop
  - `4`: high-pass
  - `5`: differentiator

## Data Layout Contract
- Arrays are allocated with fixed size `CAP = 5001`.
- The computational code uses 1-based indexing by design.
- Index `0` is reserved sentinel space and is not part of mathematical output.

This is required for exact parity with Fortran indexing semantics.

## Readable Flow in `norfil2`
`norfil2` has three explicit steps:

1. Build `CoreState` from UI/model inputs.
2. Allocate `CoreBuffers` scratch arrays.
3. Call `run_core(state, buffers)` and map to `Results`.

### `CoreState`
Stores all scalar inputs and scalar outputs used by the core:
- Inputs: `iw`, `il`, `in_`, `dt`, `df`, `os`, `of_`, `dm`, `dl`, `v1`, `ox`, `on`, `fk`, etc.
- Outputs: `r`, `l`, `l1`, `nx`, `ne`, `nf`, `nt`, `ey`, `ez`, `tp`, `tf`, `n`, etc.

### `CoreBuffers`
Stores all large working arrays:
- signal arrays: `x`, `v`, `y`, `z`
- frequency arrays: `a`, `f`
- coefficient/scratch arrays: `w`, `a1`, `a2`, `ak`, `c1`, `c2`, `c3`, `u`

## Branch Logic
Inside `norecfil`:

1. Prepare ranges (`ks`, `kf`) for design mode or signal lengths for operation mode.
2. Generate input/reference signals for operation mode.
3. Dispatch:
   - `il != 2` -> `norfil` (non-recursive)
   - `il == 2` -> `recfil` (recursive)
4. Finalize mode-specific outputs (`kn`, `tp`, `tf`, mirrored coefficients for non-recursive branch).

## Error Metrics
When operation mode runs:
- `ey = sqrt(sum((v - y)^2) / sum(v^2))`
- `ez = sqrt(sum((v - z)^2) / sum(v^2))`

These formulas are preserved from legacy logic.

## Maintenance Rules
To avoid regressions:
- Keep 1-based indexing contract unchanged.
- Keep `CAP` unchanged unless all loops and tests are updated.
- Preserve call order and branch order in `norecfil`.
- Validate against existing parity tests in `src/bindings.rs`.
