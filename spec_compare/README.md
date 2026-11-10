# Spec comparison for `fnv`

Files:

| file | what |
|---|---|
| `human_written_spec.rs` | the human spec of the public API, written from the FNV reference page and the crate (workflow step 1) |
| `human_written_spec.v1.rs` | the frozen version of that file; the working copy differs only in the `vectors()` harness |
| `compare.rs` | the comparison crate: includes `../fnv/spec.rs` (Claude's spec), the human spec and `links.rs` |
| `links.rs` | one pair of proof fns per public function, `claude_implies_human_<f>` and `human_implies_claude_<f>`, written by Claude (workflow step 3); no `assume`/`admit` |
| `GOLD_SPEC_fnv.md` | the sheet used to write the human spec: files and line ranges, functions, open decisions, vectors |

Procedure (`/home/zhukeyu/verus-tools/WORKFLOW.md`):

1. You write `human_written_spec.rs` from the crate source, its docs and the FNV reference,
   make sure it compiles, and freeze it as `.v1.rs`.
2. Claude writes its spec and proof in `../fnv/`.
3. Given the human spec, Claude amends its spec and proof and fills `links.rs`.
4. You run `bash /home/zhukeyu/verus-tools/spec_compare_run.sh /home/zhukeyu/crate-fnv 2021`.
   It writes `../evidence/SPEC_COMPARE.md` (per-function table, counts per direction) and
   `../evidence/spec_compare.log` (raw Verus output; a FAILED cell's error is the discrepancy).
5. You analyse every gap and decide whether Claude's amended spec is correct.
6. If it is not, Claude regenerates the proof for the correct spec.

Result for fnv: `../evidence/SPEC_COMPARE.md`. Claude's spec implies the human spec for 4 of 4
functions and the human spec implies Claude's for 4 of 4; the two specs are equivalent on the
whole API.
