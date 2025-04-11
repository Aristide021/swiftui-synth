# SwiftUI Layout Synthesizer (`swiftui-synth`)

A production-ready CLI tool that synthesizes SwiftUI layouts for iOS/macOS apps from high-level textual examples.

## Features

- Synthesizes SwiftUI layouts from examples in milliseconds.
- Supports `VStack`, `Text`, `Button`, and `Spacer` with basic modifiers.
- Handles optional elements and outputs to stdout or a file.
- Modular, extensible Rust codebase.

## Project Structure

- `src/ast/` — Data structures for parsed input and IR.
- `src/input/` — Input parsing and validation.
- `src/synthesis/` — Synthesis engine and future expansion.
- `src/output/` — SwiftUI code rendering.
- `src/utils/` — Utilities and profiling (future).
- `tests/` — Integration tests (future).

## Getting Started

1. Install Rust: https://rustup.rs
2. Build: `cargo build --release`
3. Run: `./target/release/swiftui-synth --help`

## License

MIT License
