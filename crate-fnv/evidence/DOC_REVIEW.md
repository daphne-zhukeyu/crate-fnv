# Documentation review: fnv 1.0.7

Every doc-comment claim in `upstream/lib.rs` that says something about behaviour, with the check
that backs the verdict. Line numbers refer to `upstream/lib.rs`. Runtime checks are in
`fnv/tests/doc_findings.rs` (run: `cargo test --test doc_findings`, log: `cargo_test.log`).

| # | lines | doc claim | verdict | evidence |
|---|---|---|---|---|
| 1 | 1 | "An implementation of the Fowler–Noll–Vo hash function" | **D1 (doc gap):** the variant is not named anywhere in the docs. The code is 64-bit FNV-1a (xor, then multiply); FNV-1 gives different values. The crate's own test is named `fnv1a` | `doc_findings.rs::d2_variant_is_fnv1a_not_fnv1`; spec `fnv1a_step` + `fnv1a_test_vectors` (`by (compute)`) in `fnv/spec.rs` |
| 2 | 5-6, 13 | "more efficient for smaller hash keys", "performs badly on larger inputs" | performance statements; not checked |  |
| 3 | 14-18 | "no protection against collision attacks" | security statement; not checked |  |
| 4 | 26-62 | `FnvHashMap` / `FnvHashSet` usage; `HashMap::new` is only for `RandomState` | holds (doc tests) | `cargo_test.log` (2 doc tests) |
| 5 | 85-87 | `FnvHasher`: "An implementation of the Fowler–Noll–Vo hash function" | as 1 |  |
| 6 | 100-101 | `with_key`: "starting with a state corresponding to the hash `key`" | holds; the key is used as the raw state, unmixed. The default state (offset basis `0xcbf29ce484222325`) is not named in the docs | `doc_findings.rs::d1_with_key_is_raw_state_and_default_is_offset_basis`; Verus: `with_key` ensures `r@ == key`, `default` ensures `r@ == fnv_offset_basis()` |
| 7 | 108-125 | `Hasher` impl (no doc comment) | `write` verified against `fnv1a_from`; `write_u8`, `write_u32`, ... are std defaults over `to_ne_bytes`, so integer hashes are endianness-dependent (undocumented, same as adler2) | `doc_findings.rs::d3_integer_writes_use_native_endian_bytes`; `FN_INVENTORY.md` |

## Upstream bugs found

None.
