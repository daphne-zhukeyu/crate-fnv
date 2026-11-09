// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Specification of 64-bit FNV-1a (ghost code only; erased from normal builds).
//!
//! The contracts in `lib.rs` are stated with these definitions. Nothing here is trusted:
//! every `spec fn` is a definition and every `proof fn` is checked by Verus.
//!
//! Reference: <http://www.isthe.com/chongo/tech/comp/fnv/index.html> (FNV-1a, 64-bit):
//!
//! ```text
//! hash = offset_basis
//! for each octet_of_data to be hashed
//!     hash = hash xor octet_of_data
//!     hash = hash * FNV_prime          (mod 2^64)
//! return hash
//! ```
#![allow(unused_imports)]

use vstd::prelude::*;

verus! {

/// The 64-bit FNV offset basis, 14695981039346656037.
pub open spec fn fnv_offset_basis() -> u64 {
    0xcbf29ce484222325
}

/// The 64-bit FNV prime, 2^40 + 2^8 + 0xb3 = 1099511628211.
pub open spec fn fnv_prime() -> int {
    0x100000001b3
}

/// One FNV-1a round: xor in the octet, then multiply by the prime modulo 2^64.
/// Stated over mathematical integers, independently of `wrapping_mul`.
pub open spec fn fnv1a_step(hash: u64, octet: u8) -> u64 {
    (((hash ^ (octet as u64)) as int * fnv_prime()) % 0x1_0000_0000_0000_0000int) as u64
}

/// FNV-1a rounds over `data`, starting from the state `hash` (first octet first).
pub open spec fn fnv1a_from(hash: u64, data: Seq<u8>) -> u64
    decreases data.len(),
{
    if data.len() == 0 {
        hash
    } else {
        fnv1a_step(fnv1a_from(hash, data.drop_last()), data.last())
    }
}

/// The 64-bit FNV-1a hash of `data`.
pub open spec fn fnv1a(data: Seq<u8>) -> u64 {
    fnv1a_from(fnv_offset_basis(), data)
}

/// Hashing `a` then `b` is hashing `a + b`: `write` calls compose.
pub proof fn lemma_fnv1a_from_append(hash: u64, a: Seq<u8>, b: Seq<u8>)
    ensures
        fnv1a_from(fnv1a_from(hash, a), b) == fnv1a_from(hash, a + b),
    decreases b.len(),
{
    if b.len() == 0 {
        assert(a + b =~= a);
    } else {
        lemma_fnv1a_from_append(hash, a, b.drop_last());
        assert((a + b).drop_last() =~= a + b.drop_last());
    }
}

/// Spec sanity: the definition reproduces published FNV-1a test vectors (evaluated by
/// `by (compute)`, i.e. Verus's interpreter, not the SMT solver).
proof fn fnv1a_test_vectors() {
    assert(fnv1a(seq![]) == 0xcbf29ce484222325u64) by (compute);
    assert(fnv1a(seq![0x61u8]) == 0xaf63dc4c8601ec8cu64) by (compute); // "a"
    assert(fnv1a(seq![0x00u8]) == 0xaf63bd4c8601b7dfu64) by (compute); // "\0"
    // "foobar"
    assert(fnv1a(seq![0x66u8, 0x6f, 0x6f, 0x62, 0x61, 0x72]) == 0x85944171f73967e8u64) by (compute);
    // "\xff\x00\x00\x01"
    assert(fnv1a(seq![0xffu8, 0x00, 0x00, 0x01]) == 0x6961196491cc682du64) by (compute);
}

} // verus!
