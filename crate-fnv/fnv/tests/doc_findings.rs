//! Executable checks of claims in the upstream documentation (see ../evidence/DOC_REVIEW.md).
extern crate fnv; // edition 2015
use fnv::FnvHasher;
use std::hash::Hasher;

/// Doc (lib.rs:100-101): `with_key` creates "an FNV hasher starting with a state corresponding
/// to the hash `key`". Checked: the key is the raw state (not mixed), and the default state is
/// the FNV-1a 64-bit offset basis, which the docs never name.
#[test]
fn d1_with_key_is_raw_state_and_default_is_offset_basis() {
    assert_eq!(FnvHasher::with_key(0x1234_5678_9abc_def0).finish(), 0x1234_5678_9abc_def0);
    assert_eq!(FnvHasher::default().finish(), 0xcbf29ce484222325);
    let mut a = FnvHasher::with_key(0xcbf29ce484222325);
    let mut b = FnvHasher::default();
    a.write(b"foobar");
    b.write(b"foobar");
    assert_eq!(a.finish(), b.finish());
}

/// Doc (lib.rs:1): "An implementation of the Fowler–Noll–Vo hash function". The docs do not say
/// which variant. Checked against the published FNV-1a vectors (xor, then multiply); FNV-1
/// (multiply, then xor) gives a different value for "a".
#[test]
fn d2_variant_is_fnv1a_not_fnv1() {
    let mut h = FnvHasher::default();
    h.write(b"a");
    assert_eq!(h.finish(), 0xaf63dc4c8601ec8c); // FNV-1a 64 of "a"
    let fnv1_a = (0xcbf29ce484222325u64).wrapping_mul(0x100000001b3) ^ b'a' as u64;
    assert_ne!(h.finish(), fnv1_a);
}

/// Doc (lib.rs:13): "performs badly on larger inputs" and "no protection against collision
/// attacks" are performance/security statements, not functional claims; not tested.
/// `write_u8`, `write_u32`, ... are std defaults over `to_ne_bytes`; checked that `write_u32`
/// equals writing the native-endian bytes, so the hash of integers is endianness-dependent.
#[test]
fn d3_integer_writes_use_native_endian_bytes() {
    let mut a = FnvHasher::default();
    let mut b = FnvHasher::default();
    a.write_u32(0x0102_0304);
    b.write(&0x0102_0304u32.to_ne_bytes());
    assert_eq!(a.finish(), b.finish());
}
