# Evidence index for fnv (2026-11-04T22:48:45Z)

| log | command | exit | key line |
|---|---|---|---|
| `verus_std.log` | `/home/zhukeyu/verus-0.2026.10.04/verus-x86-linux/verus --crate-type=lib --edition 2015 --cfg feature="std" --time lib.rs` | 0 | verification results:: 8 verified, 0 errors  |
| `verus_nostd.log` | `/home/zhukeyu/verus-0.2026.10.04/verus-x86-linux/verus --crate-type=lib --edition 2015 --time lib.rs` | 0 | verification results:: 8 verified, 0 errors  |
| `verus_nocheating.log` | `/home/zhukeyu/verus-0.2026.10.04/verus-x86-linux/verus --crate-type=lib --edition 2015 --no-cheating --cfg feature="std" lib.rs` | 0 | verification results:: 8 verified, 0 errors  |
| `verus_seed1.log` | `/home/zhukeyu/verus-0.2026.10.04/verus-x86-linux/verus --crate-type=lib --edition 2015 --cfg feature="std" --smt-option smt.random_seed=1 lib.rs` | 0 | verification results:: 8 verified, 0 errors  |
| `verus_seed2.log` | `/home/zhukeyu/verus-0.2026.10.04/verus-x86-linux/verus --crate-type=lib --edition 2015 --cfg feature="std" --smt-option smt.random_seed=2 lib.rs` | 0 | verification results:: 8 verified, 0 errors  |
| `verus_seed3.log` | `/home/zhukeyu/verus-0.2026.10.04/verus-x86-linux/verus --crate-type=lib --edition 2015 --cfg feature="std" --smt-option smt.random_seed=3 lib.rs` | 0 | verification results:: 8 verified, 0 errors  |
| `verus_seed4.log` | `/home/zhukeyu/verus-0.2026.10.04/verus-x86-linux/verus --crate-type=lib --edition 2015 --cfg feature="std" --smt-option smt.random_seed=4 lib.rs` | 0 | verification results:: 8 verified, 0 errors  |
| `verus_seed5.log` | `/home/zhukeyu/verus-0.2026.10.04/verus-x86-linux/verus --crate-type=lib --edition 2015 --cfg feature="std" --smt-option smt.random_seed=5 lib.rs` | 0 | verification results:: 8 verified, 0 errors  |
| `verus_seed6.log` | `/home/zhukeyu/verus-0.2026.10.04/verus-x86-linux/verus --crate-type=lib --edition 2015 --cfg feature="std" --smt-option smt.random_seed=6 lib.rs` | 0 | verification results:: 8 verified, 0 errors  |
| `cargo_build.log` | `cargo build` | 0 | warning: Cargo.toml: `package.edition` is unspecified, defaulting to `2015` while the latest is `2024` warning: `fnv` (manifest) generated 1 warning     Finishe |
| `cargo_test.log` | `cargo test` | 0 | test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filt |
| `cargo_test_nostd.log` | `cargo test --no-default-features` | 0 | test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filt |
| `cargo_test_release.log` | `cargo test --release` | 0 | test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filt |
| `negative_controls.log` | `env VERUS=/home/zhukeyu/verus-0.2026.10.04/verus-x86-linux/verus ./negative_controls.sh` | 0 | rejected 10, problems 0  |
| `cargo_verus_nostd.log` | `env PATH=/home/zhukeyu/verus-0.2026.10.04/verus-x86-linux:/home/zhukeyu/.local/bin:/home/zhukeyu/.vscode-server/cli/servers/Stable-6d62b1d5dace93a33b3a6d3eca1317bd5e15bcb1/server/bin/remote-cli:/home/zhukeyu/.local/bin:/home/zhukeyu/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/snap/bin cargo verus verify --no-default-features` | 0 | verification results:: 1876 verified, 0 errors verification results:: 8 verified, 0 errors  |
| `cargo_verus_std.log` | `env PATH=/home/zhukeyu/verus-0.2026.10.04/verus-x86-linux:/home/zhukeyu/.local/bin:/home/zhukeyu/.vscode-server/cli/servers/Stable-6d62b1d5dace93a33b3a6d3eca1317bd5e15bcb1/server/bin/remote-cli:/home/zhukeyu/.local/bin:/home/zhukeyu/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/snap/bin cargo verus verify` | 0 | verification results:: 8 verified, 0 errors  |

Other evidence files in this directory: FN_INVENTORY.md, TIMELINE.md, code_changes/ (CODE_CHANGES.md, SIGNOFF.md), verus_std.json, DOC_REVIEW.md, SPEC_COMPARE.md (when the gold spec exists).
