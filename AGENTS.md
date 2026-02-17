# Repository Guidelines

## Project Structure & Module Organization
- `conc_cli/` holds the Rust CLI (WASM build output in `conc_cli/pkg/`, CLI sources in `conc_cli/src/`, tests in `conc_cli/tests/`).
- `libconc/` contains the core Rust libraries (`libconc/core/`, `libconc/dev/`, `libconc/expanded/`).
- `conc_server/` and `conc_sh/` are separate Rust crates for server and shell/WASM integrations.
- `docs/` and `docs/conc/` contain specs and architecture notes; `en-gpt/` and `words.txt` carry word lists and dictionary data.

## Build, Test, and Development Commands
- `cargo build` (run inside a crate like `conc_cli/` or `libconc/core/`) builds that crate.
- `cargo run -- <args>` (inside `conc_cli/`) runs the CLI with arguments.
- `cargo test` (inside `conc_cli/` or `libconc/core/`) executes the Rust test harness for that crate.
- `nix develop` (optional) uses the `flake.nix` dev shell if you work with Nix.

## Coding Style & Naming Conventions
- Rust code follows standard Rust formatting; use `cargo fmt` when available to keep changes consistent.
- Keep module names and file paths aligned (for example, `libconc/core/src/lexicon/...`).
- JSONL dictionaries are append-only and should keep their existing key names (for example, `natural`, `conc`, `index`).

## Testing Guidelines
- Rust tests live alongside crates, primarily in `conc_cli/tests/` and `libconc/core/tests/`.
- Name Rust tests descriptively and keep fixtures in `tests/` or `tests/dict/` (for example, `word_map.jsonl`).
- Run `cargo test` in the specific crate you changed.

## Commit & Pull Request Guidelines
- Recent history uses short, prefixed subjects such as `task:`, `feature:`, `fix:`, `maint:`, and `release:`; follow that convention.
- Keep commits focused on one change area (dictionary updates vs. runtime changes).
- In PRs, describe the impacted crate(s) and include any CLI examples used for verification.

## Notes for Contributors
- The repo contains large dictionaries and build artifacts; avoid regenerating data unless you intend to update it.
- Prefer targeted edits in the crate you are changing rather than cross-cutting refactors.
