# Human spec sheet: fnv 1.0.7

General guide: `/home/zhukeyu/verus-tools/GOLD_SPEC_GUIDE.md` (workflow step 1). Output file:
`human_written_spec.rs` in this directory.

## Read

| file | why | what to take from it |
|---|---|---|
| `../upstream/lib.rs` lines 1–67 | crate docs | what the crate promises; note that the variant (FNV-1 vs FNV-1a) is never named |
| `../upstream/lib.rs` lines 85–125 | the public API: `FnvHasher`, `Default`, `with_key`, `Hasher::finish`, `Hasher::write` | the signatures and the doc comment on `with_key` ("a state corresponding to the hash `key`") |
| `../upstream/lib.rs` lines 139–365 | the crate's own unit test | ~200 published FNV vectors (`fnv1a(b"a") == 0xaf63dc4c8601ec8c`, …). Pick 5–10 short ones for `by (compute)`. The test's name tells you the variant |
| `../upstream/README.md` | usage | nothing normative beyond the docs |
| FNV reference: http://www.isthe.com/chongo/tech/comp/fnv/index.html, sections "FNV-1a hash" and "FNV-1a parameters" (64-bit: offset basis `14695981039346656037`, prime `1099511628211`) | the algorithm | the per-octet rule and the order of xor and multiply |

The body of `write` (lines 115–124, 7 lines) *is* the reference algorithm. Read it, but write
the spec from the reference page's wording, not from the loop.

## Claude's files

The human spec is written before Claude's spec and proof exist (workflow steps 1 and 2). Where
Claude's files are already present (`../fnv/lib.rs` with its contracts, `../fnv/spec.rs`,
`../VERIFICATION.md`, `../SUMMARY.md`, `../evidence/`, `links.rs`), leave them until the
comparison has run, so the human spec stays an independent reference.

## Functions to specify

| API | shape of the spec you need |
|---|---|
| `FnvHasher::default()` | the initial state: a `u64` constant |
| `FnvHasher::with_key(key: u64)` | the state after construction, as a function of `key` |
| `Hasher::finish(&self) -> u64` | the returned value as a function of the state |
| `Hasher::write(&mut self, bytes: &[u8])` | the new state as a function of the old state and `bytes` (as `Seq<u8>`) |

Suggested abstract state: `u64` (the hasher is one 64-bit word). If you pick something else,
say what it means.

Out of scope (type aliases, no code): `FnvBuildHasher`, `FnvHashMap`, `FnvHashSet`.
Out of scope (std defaults, not the crate's code): `write_u8`, `write_u32`, `write_usize`, ….

## Decisions the docs leave to you (write each down as a comment)

1. **Variant.** FNV-1 is multiply-then-xor; FNV-1a is xor-then-multiply. Decide from the
   reference and the vectors, and state which one you specify. (Discussed in chat on
   2026-10-19; write your own decision down anyway.)
2. **`with_key`.** Is `key` the raw starting state, or is it mixed/hashed in some way? The doc
   says "a state corresponding to the hash `key`".
3. **Order of bytes.** First byte first. State it; a reversed fold has the same type.
4. **Empty `write`.** What is the state after `write(&[])`? It follows from the definition, but
   check that your `by (compute)` vector for `b""` gives the offset basis.
5. **Multiply.** The multiplication is modulo 2^64. Decide how to write it (`as u64` truncation,
   `% 0x1_0000_0000_0000_0000int`, or `wrapping_mul`); all three are equivalent.

## Vectors to include (from the crate's own test; all 64-bit FNV-1a)

```
b""        0xcbf29ce484222325
b"a"       0xaf63dc4c8601ec8c
b"\0"      0xaf63bd4c8601b7df
b"foobar"  0x85944171f73967e8
b"\xff\x00\x00\x01"  0x6961196491cc682d
```

## fnv-specific places to get stuck

- Edition: this file compiles alone with the command in the guide; no `extern crate` needed.
- The prime `0x100000001b3` and the basis `0xcbf29ce484222325` are `u64` literals; in `int`
  context write them without suffix, in `u64` context add `u64` if Verus asks.
- `h ^ (b as u64)` is fine in spec code; `h ^ b` with `b: u8` is a type error.
- If a `by (compute)` vector fails, the message prints the number your spec computed. Compare
  it with the FNV-1 value of the same input before concluding the constant is wrong: a
  swapped xor/multiply order is the usual cause.
