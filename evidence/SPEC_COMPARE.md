# Spec comparison (two-way refinement)

Log: `spec_compare.log` (exit 0). Each cell is a Verus proof obligation in `spec_compare/links.rs`.

Verus: 14 verified, 0 errors over the whole comparison crate (both specs, their vectors, and the links).

| function | Claude ⇒ human | human ⇒ Claude |
|---|---|---|
| `default` | verified | verified |
| `finish` | verified | verified |
| `with_key` | verified | verified |
| `write` | verified | verified |

Claude ⇒ human: **4 of 4** verified (0 failed, 0 TODO)

human ⇒ Claude: **4 of 4** verified (0 failed, 0 TODO)
