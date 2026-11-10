#!/usr/bin/env bash
# Injects one bug at a time into a scratch copy of fnv/ and checks that Verus rejects it.
# Usage: VERUS=/path/to/verus ./negative_controls.sh
set -u
VERUS=${VERUS:-verus}
HERE=$(cd "$(dirname "$0")" && pwd)
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT
pass=0; fail=0
run() { # id, file, sed-expression, description
  rm -rf "$TMP/c"; cp -r "$HERE/fnv" "$TMP/c"
  sed -i "$3" "$TMP/c/$2"
  if cmp -s "$HERE/fnv/$2" "$TMP/c/$2"; then echo "$1 SED DID NOT APPLY ($4)"; fail=$((fail+1)); return; fi
  if "$VERUS" --crate-type=lib --cfg 'feature="std"' "$TMP/c/lib.rs" >/dev/null 2>&1; then
    echo "$1 ACCEPTED (bad): $4"; fail=$((fail+1))
  else
    why=$("$VERUS" --crate-type=lib --cfg 'feature="std"' "$TMP/c/lib.rs" 2>&1 | grep -m1 "^error" | cut -c1-60)
    echo "$1 rejected: $4  [$why]"; pass=$((pass+1))
  fi
}
run N1 lib.rs 's/hash.wrapping_mul(0x100000001b3)/hash.wrapping_mul(0x100000001b2)/' "wrong prime"
run N2 lib.rs 's/FnvHasher(0xcbf29ce484222325)/FnvHasher(0xcbf29ce484222324)/' "wrong offset basis in default()"
run N3 lib.rs 's/hash = hash ^ (\*byte as u64);/hash = hash | (*byte as u64);/' "or instead of xor"
run N4 lib.rs 's/hash = hash ^ (\*byte as u64);/hash = hash.wrapping_mul(0x100000001b3) ^ (*byte as u64); hash = hash;/; s/hash = hash.wrapping_mul(0x100000001b3);$/hash = hash;/' "FNV-1 order (multiply, then xor)"
run N5 lib.rs 's/hash = hash.wrapping_mul(0x100000001b3);/hash = hash.wrapping_add(0x100000001b3);/' "add instead of multiply"
run N6 lib.rs 's/\*self = FnvHasher(hash);//' "state not written back"
run N7 lib.rs 's/        FnvHasher(key)/        FnvHasher(0)/' "with_key ignores key"
run N8 lib.rs 's/^        self.0$/        self.0 ^ 1/' "finish perturbs the state"
run N9 lib.rs 's/(\*byte as u64)/((*byte as u64) << 1)/' "byte shifted before xor"
run N10 spec.rs 's/^    0x100000001b3$/    0x100000001b5/' "spec typo: wrong prime (caught by test vectors)"
echo "rejected $pass, problems $fail"
