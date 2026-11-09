# Code changes after macro expansion

upstream: `/home/zhukeyu/crate-fnv/upstream`  
annotated: `/home/zhukeyu/crate-fnv/fnv`  
features: `default`

Full expansions: `expanded_upstream.rs`, `expanded_annotated.rs`; diff: `expanded.diff`.

**3 diff hunk(s)** in executable code, **0 trusted/unsafe marker line(s)**. Each is a finding in `SIGNOFF.md`.

## C1

```diff
@@ -63,4 +63,30 @@
 use std::default::Default;
 use std::hash::{BuildHasherDefault, Hasher};
+// Edition 2015: the Verus annotations (and the code `verus!` expands to) need these in scope.
+extern crate core;
+extern crate vstd;
+use vstd::prelude::*;
+#[doc = " Specification of FNV-1a (ghost; erased from normal builds)."]
+pub mod spec {
+    // verus!
+    //! Specification of 64-bit FNV-1a (ghost code only; erased from normal builds).
+    //!
+    //! The contracts in `lib.rs` are stated with these definitions. Nothing here is trusted:
+    //! every `spec fn` is a definition and every `proof fn` is checked by Verus.
+    //!
+    //! Reference: <http://www.isthe.com/chongo/tech/comp/fnv/index.html> (FNV-1a, 64-bit):
+    //!
+    //! ```text
+    //! hash = offset_basis
+    //! for each octet_of_data to be hashed
+    //!     hash = hash xor octet_of_data
+    //!     hash = hash * FNV_prime          (mod 2^64)
+    //! return hash
+    //! ```
+    #![allow(unused_imports)]
+    use vstd::prelude::*;
+}
+#[allow(unused_imports)]
+use crate::spec::*;
 #[doc = " An implementation of the Fowler–Noll–Vo hash function."]
 #[doc = ""]
```

## C2

```diff
@@ -68,4 +94,8 @@
 #[allow(missing_copy_implementations)]
 pub struct FnvHasher(u64);
+#[doc = " The abstract state of a hasher is its current 64-bit hash value."]
+impl View for FnvHasher {
+    type V = u64;
+}
 impl Default for FnvHasher {
     #[inline]
```

## C3

```diff
@@ -91,7 +121,9 @@
         let FnvHasher(mut hash) = *self;
         for byte in bytes.iter() {
+            {}
             hash = hash ^ (*byte as u64);
             hash = hash.wrapping_mul(0x100000001b3);
         }
+        {}
         *self = FnvHasher(hash);
     }
```

## Trusted / unsafe markers (annotated sources)


## Release assembly comparison (per function; symbol hashes and local labels normalised)

| fn | result |
|---|---|
| `write` | symbol not found (inlined away?) |
