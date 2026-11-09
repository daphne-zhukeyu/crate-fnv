//! Linking lemmas between the Claude spec (`c`) and the gold spec (`h`), one pair per public
//! function. Written by Claude on 2026-11-08 after the gold spec was frozen (human_written_spec.v1.rs).
//!
//! Shape: `claude_implies_human_f` assumes the crate's verified contract for `f` (as stated in
//! `../fnv/lib.rs`, in terms of `c`) and concludes the gold spec's statement; `human_implies_claude_f`
//! is the converse. The state abstraction is the same on both sides (the `u64` hash word, `FnvHasher@`).
#![allow(unused_imports)]
use vstd::prelude::*;
use crate::claude_spec as c;
use crate::human_written_spec as h;

verus! {

/// The two folds are the same function (both recurse on `drop_last`/`last` with the same round).
proof fn lemma_fold_agree(s: u64, bytes: Seq<u8>)
    ensures
        c::fnv1a_from(s, bytes) == h::hash_from(s, bytes),
    decreases bytes.len(),
{
    if bytes.len() > 0 {
        lemma_fold_agree(s, bytes.drop_last());
    }
}

// ---- FnvHasher::default: crate ensures `r@ == fnv_offset_basis()`; gold: `default_result()`

pub proof fn claude_implies_human_default(r: u64)
    requires
        r == c::fnv_offset_basis(),
    ensures
        r == h::default_result(),
{
}

pub proof fn human_implies_claude_default(r: u64)
    requires
        r == h::default_result(),
    ensures
        r == c::fnv_offset_basis(),
{
}

// ---- FnvHasher::with_key(key): crate ensures `r@ == key`; gold: `with_key_result(key)`

pub proof fn claude_implies_human_with_key(key: u64, r: u64)
    requires
        r == key,
    ensures
        r == h::with_key_result(key),
{
}

pub proof fn human_implies_claude_with_key(key: u64, r: u64)
    requires
        r == h::with_key_result(key),
    ensures
        r == key,
{
}

// ---- Hasher::finish: crate ensures `r == self@`; gold: `finish_result(state)`

pub proof fn claude_implies_human_finish(state: u64, r: u64)
    requires
        r == state,
    ensures
        r == h::finish_result(state),
{
}

pub proof fn human_implies_claude_finish(state: u64, r: u64)
    requires
        r == h::finish_result(state),
    ensures
        r == state,
{
}

// ---- Hasher::write(bytes): crate ensures `final(self)@ == fnv1a_from(old(self)@, bytes@)`;
//      gold: `write_next(old, bytes)`

pub proof fn claude_implies_human_write(old_state: u64, bytes: Seq<u8>, new_state: u64)
    requires
        new_state == c::fnv1a_from(old_state, bytes),
    ensures
        new_state == h::write_next(old_state, bytes),
{
    lemma_fold_agree(old_state, bytes);
}

pub proof fn human_implies_claude_write(old_state: u64, bytes: Seq<u8>, new_state: u64)
    requires
        new_state == h::write_next(old_state, bytes),
    ensures
        new_state == c::fnv1a_from(old_state, bytes),
{
    lemma_fold_agree(old_state, bytes);
}

} // verus!
