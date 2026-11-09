# Proposed classification of each finding (Claude's proposal; the reviewer decides in SIGNOFF.md)

Findings are from `CODE_CHANGES.md` (expansion diff of a normal build, where `verus!` erases ghost
code).

| id | what the hunk is | proposed | why |
|---|---|---|---|
| C1 | `extern crate core; extern crate vstd; use vstd::prelude::*;`; new empty `pub mod spec` (doc comment + unused import); `impl View for FnvHasher { type V = u64; }`; doc comments on `FnvHasher` re-emitted in `#[doc]` form | ghost-only | edition-2015 imports; an empty module; a trait impl with only an associated type (the `view` fn is erased). No executable code |
| C2 | doc comments on `with_key` in `#[doc]` form | ghost-only | `verus!` re-emits `///` as `#[doc = …]`; identical to rustc |
| C3 | two `{}` no-op blocks in `write` (erased `proof { }` blocks) | ghost-only | an empty block statement compiles to nothing. Supporting: the only MIR difference in `write` is the display name of an inlined scope (`core::ub_checks` vs `ub_checks`, from the added `extern crate core`), `mir_diff_write.diff` |
| (T) | none | | the text scan found no trusted/external/unsafe marker in the annotated sources; `--no-cheating` passes with 8 verified (`../verus_nocheating.log`) |

Assembly comparison: `write` is `#[inline]` with no caller inside the crate, so neither build
emits a symbol for it (`symbol not found` in the table). The MIR comparison above stands in.

Nothing here is `behaviour-change` or `behaviour-preserving`: every executable statement of
upstream `lib.rs` is unchanged.
