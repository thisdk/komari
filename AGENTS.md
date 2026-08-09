# Repository Guidelines

## Project Structure & Module Organization

Komari is a Rust Cargo workspace with three members:

- `backend/` — core automation logic: `player/` state machines, `services/` runtime mediators, `solvers/` mini-game solvers, `models/` persisted data, `tracker/` object tracking, and `grpc/` for the input bridge.
- `platforms/` — Windows capture and input abstractions.
- `ui/` — the Dioxus desktop app; `assets/` and `public/` hold Tailwind and JS resources.

Detection templates and ONNX models live in `backend/resources/`. Examples, docs, and utility scripts live in `examples/python/`, `docs/`, and `scripts/`.

## Build, Test, and Development Commands

The supported environment is Windows (`x86_64-pc-windows-msvc`) with the pinned nightly toolchain.

- `cargo fmt --check` — verify formatting.
- `cargo clippy -- -D warnings` — lint; workspace lints deny selected clippy rules.
- `cargo test -- --no-capture` — run all unit tests, as CI does.
- `cargo test -p backend` — run backend tests only.
- `cd ui && npm install` — install Tailwind dependencies.
- `dx build --package ui` — debug build; `dx build --release --package ui` — release build. Outputs go to `target/dx/ui/<profile>/windows/app/`.
- `ui_debug.bat` / `ui_release.bat` — launch existing builds with admin elevation.

Full prerequisites (LLVM, vcpkg/OpenCV, protoc, Dioxus CLI) are in [docs/building.md](docs/building.md).

## Coding Style & Naming Conventions

Format with rustfmt using `edition 2024` and `group_imports = "StdExternalCrate"` from `rustfmt.toml`. Nightly features are expected; declare them in `#![feature(...)]` at the crate root. Use `snake_case` for functions and variables, `CamelCase` for types, and `SCREAMING_SNAKE_CASE` for constants. Keep functions small and behavior-named.

## Testing Guidelines

Unit tests are colocated in `#[cfg(test)] mod tests` at the bottom of backend source modules. Name tests as `behavior_under_condition` (e.g., `find_points_with_no_path`). Add tests when changing state machines, pathing, detection thresholds, or configuration parsing. Run `cargo test`; CI runs all tests with `--no-capture`. No separate coverage gate is configured.

## Commit & Pull Request Guidelines

Git history uses Conventional Commits: `feat:`, `fix:`, `refactor:`, `perf:`, `chore:`, `test:`, `build:`, `ci:`, and `docs:`. Keep subjects short and lowercase; use the body to explain behavior and rationale. PRs target `master`, must pass fmt, clippy, and tests, and should link related issues; include screenshots for UI changes. Tagged `v*.*.*` releases build release and debug artifacts.

## Security & Configuration Tips

Never commit credentials, `.env` content, or local proxy settings. `.gitignore` excludes `/target` and `dataset/`; keep generated datasets out of the repo. Local environment configuration (`OPENCV_*`, `LIBCLANG_PATH`, proxy) belongs in your user-level `.cargo/config.toml`, not in repository files.
