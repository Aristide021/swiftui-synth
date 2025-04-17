# Contributing to SwiftUI Layout Synthesizer

Thank you for your interest in contributing to SwiftUI Layout Synthesizer! This document provides guidelines and instructions for contributing to this project.

## Development Workflow

### Setup

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/swiftui-synth.git
   cd swiftui-synth
   ```
3. Add the upstream repository as a remote:
   ```bash
   git remote add upstream https://github.com/aristide021/swiftui-synth.git
   ```
4. Install Rust if you haven't already:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

### Development Cycle

1. Create a new branch for your feature or bugfix:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes and ensure they follow our coding standards

3. Write tests for your changes and ensure all tests pass:
   ```bash
   cargo test
   ```

4. Check code formatting and lint issues:
   ```bash
   cargo fmt --check
   cargo clippy -- -D warnings
   ```

5. Commit your changes using [Conventional Commits](https://www.conventionalcommits.org/) format:
   ```
   feat: add support for TextField
   fix: handle parser edge case with escaped quotes
   docs: update README with new examples
   chore: update dependencies
   ```

6. Push your branch to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

7. Open a pull request against the upstream repository

## CI/CD Pipeline

The project uses GitHub Actions for continuous integration and delivery:

1. **Build and Test**: Runs on every push and pull request to verify that the code builds and all tests pass
2. **Automated Versioning**: Increments version based on commit messages following [Semantic Versioning](https://semver.org/)
3. **Release Process**: Creates GitHub releases and builds binaries for multiple platforms when a new version is tagged

## Code Conventions

- Follow standard Rust coding conventions
- Use meaningful variable and function names
- Write comprehensive documentation comments
- Add unit tests for new functionality
- Ensure error messages are user-friendly and informative

## Adding New SwiftUI Components

When adding support for new SwiftUI components:

1. Update the `IR` enum in `src/ast/ir.rs` with the new component type
2. Add parser support in `src/input/parser.rs` if necessary
3. Implement the synthesis logic in `src/synthesis/swiftui.rs`
4. Add rendering logic in `src/output/render.rs`
5. Write tests for each layer (parsing, synthesis, rendering)
6. Add integration tests for end-to-end functionality
7. Update the README with examples of the new component

## Submitting Issues

When submitting an issue, please include:

- A clear description of the problem
- Steps to reproduce the issue
- Expected vs. actual behavior
- Version of `swiftui-synth` you're using
- Your OS version
- Any relevant logs or error messages 