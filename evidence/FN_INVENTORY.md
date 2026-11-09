# Function inventory

Upstream source: `/home/zhukeyu/crate-fnv/upstream`  
Annotated crate: `/home/zhukeyu/crate-fnv/fnv`  
Verus JSON: `/home/zhukeyu/crate-fnv/evidence/verus_std.json` (version 0.2026.10.04.426d8b0; results: 8 verified, 0 errors, entire crate: True)

**4 of 4 upstream functions verified** (non-test, non-bench `fn` items).

| # | upstream location | function | Verus name | status |
|---|---|---|---|---|
| 1 | `lib.rs:94` | `FnvHasher::default  (impl Default)` | `lib::FnvHasher::default` | verified |
| 2 | `lib.rs:103` | `FnvHasher::with_key` | `lib::FnvHasher::with_key` | verified |
| 3 | `lib.rs:110` | `FnvHasher::finish  (impl Hasher)` | `lib::FnvHasher::finish` | verified |
| 4 | `lib.rs:115` | `FnvHasher::write  (impl Hasher)` | `lib::FnvHasher::write` | verified |

## Trusted / external markers in the annotated crate (text scan)

none
