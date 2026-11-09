# fnv 1.0.7 with Verus: summary (Updated: 2026-11-08)

fnv is the Fowler–Noll–Vo hasher (876M downloads), a single `write` loop; it served as the
toolchain check for Verus 0.2026.10.04 before adler2. The rules are those used for hex: no
`assume`, the crate passes `--no-cheating`, and anything Verus lacks a model for goes into a
declared trusted base. Two additions this time: every number in [REPORT.md](REPORT.md) points to
the file that produced it, and the spec was checked against a gold spec written blind.

## Headline

4 of 4 functions verified (`default`, `with_key`, `finish`, `write`); the remaining public items
are type aliases with no code. The trusted base is empty: `--no-cheating` passes with no flagged
item ([verus_nocheating.log](evidence/verus_nocheating.log)). No upstream bug was found. One
documentation gap: the crate never names its variant; the code is 64-bit FNV-1a.

Source of the count: [FN_INVENTORY.md](evidence/FN_INVENTORY.md), generated from Verus's
`--output-json` over every `fn` in the upstream source. Verus result: 8 verified, 0 errors
([verus_std.log](evidence/verus_std.log)).

## What "verified" is measured against

Claude's spec is [fnv/spec.rs](fnv/spec.rs): `fnv1a_step` as xor then multiply modulo 2^64,
written over mathematical integers, and `fnv1a_from` as the fold over the bytes. The gold spec,
[human_written_spec.v1.rs](spec_compare/human_written_spec.v1.rs), was written from the FNV
reference page and the crate docs before the Claude spec was opened, and has the same shape.
Both reproduce five published FNV-1a vectors inside Verus (`by (compute)`).

## Spec comparison

Each cell in [SPEC_COMPARE.md](evidence/SPEC_COMPARE.md) is a Verus proof obligation from
[links.rs](spec_compare/links.rs), run with `--no-cheating`. Claude's spec implies the gold spec
for 4 of 4 functions, and the gold spec implies Claude's for 4 of 4
([spec_compare.log](evidence/spec_compare.log): 14 verified, 0 errors). The two specs are
equivalent on the whole API; `write` needs one induction showing the two folds are the same
function. Both specs made the same choices on the open points: FNV-1a order, `with_key` as the
raw starting state, first byte first, multiply modulo 2^64.

Documentation claims and their checks are in [DOC_REVIEW.md](evidence/DOC_REVIEW.md); the
executable checks are in [doc_findings.rs](fnv/tests/doc_findings.rs), including the FNV-1 versus
FNV-1a distinction and the native-endian behaviour of the integer `write_*` methods.

## Effect of annotation on the compiled code

A script expands both the upstream and the annotated crate with rustc, which erases ghost code,
and diffs the result: [CODE_CHANGES.md](evidence/code_changes/CODE_CHANGES.md). There are 3
hunks, all ghost residue: imports, an empty `spec` module, an empty `View` impl, and `{}` blocks
where proof blocks were erased. Every executable statement of upstream `lib.rs` is unchanged.
The only MIR difference in `write` is a scope display name
([mir_diff_write.diff](evidence/code_changes/mir_diff_write.diff)). Each hunk has a line in
[SIGNOFF.md](evidence/code_changes/SIGNOFF.md), with proposed classifications in
[REVIEW_NOTES.md](evidence/code_changes/REVIEW_NOTES.md); the sign-off is still open.

## Scope limits

The contracts are attached to `impl Hasher for FnvHasher`; vstd gives the `Hasher` trait itself
no specification, so code generic over `H: Hasher`, such as `HashMap`, sees none of them.
`write_u8`, `write_u32` and the other integer methods are std defaults over `to_ne_bytes` and sit
outside the verified surface.

## Robustness checks

Ten injected bugs were all rejected by Verus
([negative_controls.log](evidence/negative_controls.log)). Results are identical under six SMT
random seeds; verification takes 0.4 s wall clock. The upstream tests pass in std, no_std and
release builds, and `cargo verus verify` works for both feature sets. Every command behind these
statements, with its exit code and output, is listed in [INDEX.md](evidence/INDEX.md).

## Cost

From the transcript timestamps in [TIMELINE.md](evidence/TIMELINE.md): about 25 seconds from
writing the spec to the first full pass, about 2 minutes of spec-and-proof work including the
test vectors, and 6 minutes in total with negative controls, builds and the write-up. The rest of
the pipeline is scripted: [WORKFLOW.md](../verus-tools/WORKFLOW.md) is the eight-step procedure,
and [GOLD_SPEC_GUIDE.md](../verus-tools/GOLD_SPEC_GUIDE.md) covers writing the gold spec.

## Toolchain findings

Edition-2015 crates need `extern crate vstd;` and `extern crate core;` for the annotations, and
the bare `verus` binary and `cargo verus` default to different editions; details in
[VERIFICATION.md](VERIFICATION.md). `--no-cheating` passed through cargo breaks the vstd build,
so that check uses the bare binary.


## Note on the gold spec file

Claude's only edit to the working gold spec is inside the `vectors()` proof function: the
`let`-bound initial state was inlined into the asserts, because `by (compute)` treats such a
variable as symbolic and the residue sent the SMT solver over its resource limit. Definitions are
untouched; `diff spec_compare/human_written_spec.v1.rs spec_compare/human_written_spec.rs` shows
the change.
