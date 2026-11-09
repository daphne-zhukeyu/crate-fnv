// Gold spec for fnv 1.0.7.
// Sources: the public API, upstream test vectors, and the FNV-1a reference.

use vstd::prelude::*;

verus! {

/// The current 64-bit hash word.
pub type State = u64;

/// One FNV-1a round: XOR the zero-extended byte, then multiply modulo 2^64.
pub open spec fn step(state: State, byte: u8) -> State {
    (((state ^ (byte as u64)) as int * 0x100000001b3int)
        % 0x1_0000_0000_0000_0000int) as u64
}

/// Fold in input order. Empty input preserves the supplied state.
pub open spec fn hash_from(state: State, bytes: Seq<u8>) -> State
    decreases bytes.len(),
{
    if bytes.len() == 0 {
        state
    } else {
        step(hash_from(state, bytes.drop_last()), bytes.last())
    }
}

/// `FnvHasher::default` starts at the standard FNV-1a 64-bit offset basis.
pub open spec fn default_result() -> State {
    0xcbf29ce484222325u64
}

/// `FnvHasher::with_key` treats the key as the raw starting hash word.
/// This interprets the documentation's state corresponding to the key
/// as preserving that word exactly.
pub open spec fn with_key_result(key: u64) -> State {
    key
}

/// `Hasher::finish` reports the current word.
pub open spec fn finish_result(state: State) -> u64 {
    state
}

/// `Hasher::write` continues the fold from the current word.
pub open spec fn write_next(state: State, bytes: Seq<u8>) -> State {
    hash_from(state, bytes)
}

proof fn vectors() {
    // Harness note (Claude, 2026-11-08, definitions untouched; blind version in
    // human_written_spec.v1.rs): `by (compute)` treats a `let`-bound variable as symbolic and
    // hands the residue to Z3, which times out on the 64-bit multiply. Calls are inlined instead.
    assert(finish_result(write_next(default_result(), Seq::<u8>::empty()))
        == 0xcbf29ce484222325u64) by (compute);
    assert(finish_result(write_next(default_result(), seq![0x61u8]))
        == 0xaf63dc4c8601ec8cu64) by (compute);
    assert(finish_result(write_next(default_result(), seq![0u8]))
        == 0xaf63bd4c8601b7dfu64) by (compute);
    assert(finish_result(write_next(default_result(), seq![0x66u8, 0x6f, 0x6f, 0x62, 0x61, 0x72]))
        == 0x85944171f73967e8u64) by (compute);
    assert(finish_result(write_next(default_result(), seq![0xffu8, 0x00, 0x00, 0x01]))
        == 0x6961196491cc682du64) by (compute);
}

} // verus!
