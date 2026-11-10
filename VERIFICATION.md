# Verifying `fnv` with Verus: results (2026-10-18)

Crate: `fnv` 1.0.7 from crates.io (servo/rust-fnv; 876M downloads). Verus **0.2026.10.04.426d8b0** (stable) on rustc 1.98.1, with vstd `=0.0.0-2026-10-04-0306`. The annotated crate is in [`fnv/`](fnv/), the pristine crates.io sources are in [`upstream/`](upstream/), and the diff between them is [`upstream.diff`](upstream.diff). Claude (Opus 5.5) wrote the specifications and proofs from the FNV reference page and the crate docs; the human spec they were checked against is in [`spec_compare/`](spec_compare/) (see "Spec comparison" at the end).

## Headline

| | |
|---|---|
| Verus result (default features) | **8 verified, 0 errors**, ~0.6 s wall clock |
| no_std (`--no-default-features`) | 8 verified, 0 errors |
| Trusted items (`--no-cheating`) | **0**. No `assume`, `admit`, `external_body`, `assume_specification` or `#[verifier::external]` in crate code |
| Not verified | nothing with executable code. The type aliases `FnvBuildHasher`, `FnvHashMap` and `FnvHashSet` have no code of their own |
| Plain `cargo build`/`cargo test` | pass (upstream unit test with ~200 published vectors, 2 doc tests), also `--no-default-features` |
| Negative controls | 10/10 injected bugs rejected (`negative_controls.sh`) |

## Coverage by public item

| Public item | Status | Contract |
|---|---|---|
| `FnvHasher::default()` | verified | `r@ == fnv_offset_basis()` (0xcbf29ce484222325) |
| `FnvHasher::with_key(key)` | verified | `r@ == key` |
| `Hasher::finish` | verified | `r == self@` |
| `Hasher::write(bytes)` | verified | `final(self)@ == fnv1a_from(old(self)@, bytes@)` |
| `FnvBuildHasher`, `FnvHashMap`, `FnvHashSet` | n/a | type aliases over std. They contain no code |

`FnvHasher@` (a `closed` view) is the 64-bit running hash. The spec in [`fnv/spec.rs`](fnv/spec.rs) consists of:

- `fnv1a_step(h, b) = ((h ^ b) * 0x100000001b3) mod 2^64`, written with mathematical integers, so it is independent of `wrapping_mul`.
- `fnv1a_from(h, data)`, a left fold of `fnv1a_step` over `data` with the first byte first, and `fnv1a(data) = fnv1a_from(offset_basis, data)`.
- `lemma_fnv1a_from_append`, which proves that `write(a); write(b)` gives the same state as `write(a ++ b)`. This is the property that makes it safe for `Hash` impls to split their input across several `write` calls.
- `fnv1a_test_vectors`, which uses `by (compute)` to check that the spec reproduces 5 published FNV-1a vectors (`""`, `"a"`, `"\0"`, `"foobar"`, `"\xff\0\0\x01"`). This checks the spec itself, not only the code against the spec.

## What the specification commits to (intent decisions)

1. **The algorithm is FNV-1a, not FNV-1.** It XORs first and then multiplies. The crate docs only say "Fowler–Noll–Vo", and the code, test vectors and spec all agree on 1a.
2. **`with_key` is a raw state.** `with_key(k)` followed by `write(d)` gives `fnv1a_from(k, d)`. The key is not mixed or hashed. The docs ("starting with a state corresponding to the hash `key`") support this reading.
3. **Only `write` is specified.** `write_u8`, `write_u32`, `write_usize` and the other `Hasher` methods use std's default implementations, which forward `to_ne_bytes()` to `write`. That makes the hash of an integer endianness-dependent. Those defaults are std code and are not verified here.

## Trusted base

None in the crate. The proof relies only on vstd's own (trusted) specifications of `u64::wrapping_mul`, `^`, slice `iter()` and the `for`-loop iterator protocol.

**Limitation:** the contracts are attached to `impl Hasher for FnvHasher`, but vstd gives the `Hasher` trait itself no specification. A caller that holds a concrete `FnvHasher` gets the contracts. Code that is generic over `H: Hasher`, such as `HashMap`, does not. Specifying `Hasher` generically would take a trusted `external_trait_specification`, and nothing in this crate needs one.

## Changes to the crate (audit of `upstream.diff`)

The upstream build files list `lib.rs` at the crate root (no `src/`), and that layout is kept. Every executable statement is unchanged.

| Change | Effect on compiled code |
|---|---|
| Wrapped `FnvHasher` and its three impls in `verus! { }` | none (the macro erases ghost code) |
| Added `requires`/`ensures` to the 4 functions and an `impl View for FnvHasher` (closed spec fn) | none (ghost) |
| `for byte in bytes.iter()` → `for byte in it: bytes.iter()` plus a loop invariant and two `proof { }` asserts | none (the `it:` name is ghost) |
| `extern crate vstd;`, plus `extern crate core;` under `feature = "std"` | none. The crate is edition 2015, so `vstd` and the `::core` paths that `verus!` expands to need explicit `extern crate` |
| New `spec.rs` (ghost only) | none |
| Cargo.toml: `vstd` dependency, `[package.metadata.verus]`, `rust-toolchain.toml` | build-time only |

## Negative controls (all rejected)

`VERUS=/path/to/verus ./negative_controls.sh` injects each bug into a scratch copy and checks that verification fails.

| id | injected bug | how Verus rejects it |
|---|---|---|
| N1 | prime `0x100000001b3` → `…1b2` | loop invariant (resource limit) |
| N2 | wrong offset basis in `default()` | postcondition |
| N3 | `\|` instead of `^` | loop invariant |
| N4 | FNV-1 order (multiply, then xor) | loop invariant |
| N5 | `wrapping_add` instead of `wrapping_mul` | loop invariant (resource limit) |
| N6 | state not written back to `*self` | postcondition |
| N7 | `with_key` ignores `key` | postcondition |
| N8 | `finish` returns `self.0 ^ 1` | postcondition |
| N9 | byte shifted left before xor | loop invariant |
| N10 | spec typo: wrong prime in `spec.rs` | `by (compute)` test vector fails |

N1 and N5 hit the SMT resource limit instead of producing a clean counterexample. Verus could not prove those versions, so they count as rejected, but the error message is less informative.

## Verus findings from this pass (toolchain notes)

- **`cargo verus` works on 0.2026.10.04.** The panic seen with 0.2026.05.03 is gone, and `cargo verus verify` (with and without `--no-default-features`) verifies the crate against vstd from crates.io.
- **`cargo verus verify -- --no-cheating` is unusable.** The flag is also passed to the vstd build, which fails with 34 errors. The trusted-base check therefore uses the bare binary (see Reproduce).
- **Editions differ between drivers.** The bare `verus` binary and `cargo verus` default to different editions. For a 2015-edition crate, the bare binary accepted code that `cargo verus` rejected (`cannot find core in the crate root`, from `verus!`'s `for`-loop expansion). The commands below pass `--edition 2015` explicitly.
- **Ensures on an external trait's impl are accepted.** Verus allowed `ensures` on the methods of `impl Hasher for FnvHasher` without any trait specification. They are enforced on the impl body and visible to callers that use the concrete type.

## Reproduce

```sh
cd fnv
cargo verus verify                                   # 8 verified, 0 errors
cargo verus verify --no-default-features             # no_std
verus --crate-type=lib --edition 2015 --no-cheating --cfg 'feature="std"' lib.rs   # trusted-base check
cargo build && cargo test && cargo test --no-default-features
cd .. && VERUS=$(which verus) ./negative_controls.sh # 10/10 rejected
```

## Spec comparison

The human spec of the public API is `spec_compare/human_written_spec.rs` (frozen as `.v1.rs`), written from the FNV reference page and the crate docs. Given it, Claude's spec needed no amendment; Claude wrote the linking proofs in `spec_compare/links.rs`, and the equivalence script gives 4 of 4 functions in each direction (`evidence/SPEC_COMPARE.md`). Both specs make the same choices on the FNV-1a order, the prime and basis constants, the mod 2^64 multiply, the first-byte-first fold order, and the reading of `with_key`.

### Amendments

None to Claude's spec or proof. The only edit to the working copy of the human spec is inside its `vectors()` harness (`let initial = …` inlined so `by (compute)` can evaluate); `diff spec_compare/human_written_spec.v1.rs spec_compare/human_written_spec.rs` shows it.
