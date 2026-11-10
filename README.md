# fnv 1.0.7 verified with Verus

[fnv](https://crates.io/crates/fnv) is the Fowler–Noll–Vo hasher (64-bit FNV-1a) used throughout
the Rust ecosystem. This repository holds the crate annotated with Verus specifications and
proofs, the evidence behind every reported number, and the hand-written human spec the
Claude-written spec was checked against.

Workflow (details in [verus-tools/WORKFLOW.md](../verus-tools/WORKFLOW.md)): the human writes a
spec from the crate source, docs and the FNV reference and checks that it compiles; Claude writes
its spec and proof; given the human spec, Claude amends them; the human runs the equivalence
script in both directions and analyses every gap; if Claude's amended spec is wrong, Claude
regenerates the proof for the correct spec. Spec and proof generation and amendment use Opus 5.5
at thinking level max.

**Results:** 4 of 4 functions verified against the FNV-1a definition; empty trusted base
(`--no-cheating` flags nothing); 0 `assume`; every executable statement of upstream unchanged;
the spec is equivalent to the human spec on the whole API (4 of 4 in both directions).

| read this | for |
|---|---|
| [SUMMARY.md](SUMMARY.md) | the report: results, a claim-to-evidence table, caveats |
| [VERIFICATION.md](VERIFICATION.md) | spec rationale, intent decisions, Verus toolchain findings |

## Layout

```
upstream/        the crates.io tarball, unpacked, never edited
fnv/             the annotated crate: lib.rs (contracts and proofs in place), spec.rs
                 (specification, with published FNV-1a vectors checked by `by (compute)`),
                 tests/doc_findings.rs
upstream.diff    annotated crate vs upstream
spec_compare/    human spec (human_written_spec.v1.rs is the frozen version), linking
                 proofs (links.rs), and the sheet used to write the human spec
evidence/        every log, inventory, diff and comparison the reports cite (INDEX.md lists them)
negative_controls.sh   10 injected bugs, each rejected by Verus
```

## Reproduce

Toolchain: Verus `0.2026.10.04.426d8b0` (stable release, bare binary), rustc 1.98.1 via rustup
(pinned in `fnv/rust-toolchain.toml`), vstd `=0.0.0-2026-10-04-0306` from crates.io. The crate is
edition 2015, so pass `--edition 2015` to the bare binary.

```sh
cd fnv
verus --crate-type=lib --edition 2015 --cfg 'feature="std"' lib.rs                 # 8 verified, 0 errors
verus --crate-type=lib --edition 2015 lib.rs                                       # no_std: 8 verified
verus --crate-type=lib --edition 2015 --no-cheating --cfg 'feature="std"' lib.rs   # 8 verified, nothing flagged
cargo verus verify && cargo verus verify --no-default-features                     # also works
cargo build && cargo test && cargo test --no-default-features && cargo test --release
cd .. && VERUS=$(which verus) ./negative_controls.sh                               # 10/10 rejected
```

Spec equivalence check (human spec vs Claude spec, both directions, `--no-cheating`):

```sh
cd spec_compare && verus --crate-type=lib --edition 2021 --no-cheating compare.rs  # 14 verified, 0 errors
```

## Scope

The contracts are on `impl Hasher for FnvHasher`. vstd gives the `Hasher` trait itself no
specification, so callers generic over `H: Hasher` (for example `HashMap`) see none of them.
`write_u8`, `write_u32` and the other integer methods are std's defaults over `to_ne_bytes` and
sit outside the verified surface.

## Status

Complete. Documentation note for upstream: the crate never names its variant (it is 64-bit
FNV-1a), and the hash of integers written through `write_u32` etc. is endianness-dependent.

## License

The crate is `Apache-2.0 / MIT` (see `upstream/`). The annotations, specifications, proofs and
tooling in this repository are offered under the same terms.
