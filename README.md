# SwiftUI Layout Synthesizer (`swiftui-synth`)

[![Build Status](https://github.com/aristide021/swiftui-synth/actions/workflows/rust.yml/badge.svg)](https://github.com/aristide021/swiftui-synth/actions/workflows/rust.yml) [![Release](https://github.com/aristide021/swiftui-synth/actions/workflows/release.yml/badge.svg)](https://github.com/aristide021/swiftui-synth/actions/workflows/release.yml)

`swiftui-synth` is a command-line tool written in Rust that automates the creation of basic SwiftUI layouts for iOS and macOS applications by synthesizing them from high-level textual examples. Provide a description of the desired UI, and the tool generates the corresponding SwiftUI code.

## Features

*   **Synthesizes SwiftUI Layouts:** Generates code for `VStack`, `HStack`, `Text`, `Button`, `Image`, and `Spacer`.
*   **Multiple Input Methods:** Accepts layout descriptions directly via the `--examples` flag or from a file using `--examples-file`.
*   **Flexible Output:** Prints the generated SwiftUI code to standard output or saves it directly to a file using the `--output` flag.
*   **Fast Synthesis:** Quickly translates examples into code (currently uses direct translation based on input structure).
*   **Basic Modifiers:** Automatically adds common modifiers like `.font(.title)` and `.padding()`.
*   **Handles Variations:** Correctly processes examples with optional elements (e.g., omitting a button if its value is an empty string `""`).
*   **Robust Parsing:** Includes error handling for common input format issues.
*   **Extensible:** Built with a modular Rust codebase for future enhancements.
*   **CI/CD Pipeline:** Automated testing, versioning, and release process.

## Installation

### Prerequisites

*   **Rust Toolchain:** Ensure you have Rust and Cargo installed. If not, install them via `rustup`:
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
*   **macOS:** The tool is primarily targeted at macOS developers. Tested on macOS Sonoma/Sequoia.

### Option 1: Install via Cargo

```bash
cargo install swiftui-synth
```

### Option 2: Install via Homebrew

```bash
brew tap aristide021/tap
brew install swiftui-synth
```

### Option 3: Download Pre-built Binary

Download the latest release binary for your platform from the [GitHub Releases page](https://github.com/aristide021/swiftui-synth/releases).

### Option 4: Manual Installation (from Source)

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/aristide021/swiftui-synth.git
    cd swiftui-synth
    ```
2.  **Build the release binary:**
    ```bash
    cargo build --release
    ```
    The optimized binary will be located at `target/release/swiftui-synth`.
3.  **Copy the binary to a location in your PATH:**
    A common location is `/usr/local/bin`.
    ```bash
    cp target/release/swiftui-synth /usr/local/bin/swiftui-synth
    ```
    *(Note: You might need `sudo` depending on permissions. Ensure `/usr/local/bin` is in your shell's `$PATH`.)*
4.  **Verify installation:**
    ```bash
    swiftui-synth --version
    ```

## Usage

### Core Concept

The tool takes a string describing the desired layout. The format consists of dimensions followed by the elements:

*   **VStack Format:** `{(width:W,height:H):{element1:"value1", element2:"value2", ...}}`
    *   Supported elements: `title` (Text), `button` (Button), `Image` (Image).
    *   A `Spacer` is automatically added before the `button` (if present) or at the end if no button exists.
*   **HStack Format:** `{(width:W,height:H):HStack:{"child1","child2","Spacer",...}}`
    *   Children are specified as a comma-separated list of quoted strings.
    *   The literal string `"Spacer"` generates a `Spacer`. Other strings generate `Text` views.

### Command-Line Interface

```
swiftui-synth [OPTIONS]
```

**Options:**

*   `--examples <EXAMPLES>`: Provide the layout description string directly. Use shell quotes to handle spaces and special characters. (Mutually exclusive with `--examples-file`)
*   `--examples-file <FILE>`: Provide the path to a file containing the layout description string. (Mutually exclusive with `--examples`)
*   `--output <FILE>`: Optional. Specify a file path to save the generated SwiftUI code. If omitted, code is printed to standard output.
*   `-h, --help`: Print help information.
*   `-V, --version`: Print version information.

### Examples

#### 1. Basic VStack (Title and Button) via CLI String

```bash
# Note the escaped quotes (\") needed by the shell
swiftui-synth --examples "{(width:390,height:844):{title:\"Hello\",button:\"Click\"}}"
```

**Output:**

```swift
VStack {
    Text("Hello")
        .font(.title)
        .padding()
    Spacer()
    Button("Click") { }
        .padding()
}
.padding()
```

#### 2. VStack (Title Only) via File

*   Create `mytitle.txt`:
    ```
    {(width:320,height:480):{title:"Welcome Screen"}}
    ```
*   Run the command:
    ```bash
    swiftui-synth --examples-file mytitle.txt
    ```

**Output:**

```swift
VStack {
    Text("Welcome Screen")
        .font(.title)
        .padding()
    Spacer()
}
.padding()
```

#### 3. VStack with Empty Button (Button is Omitted)

```bash
swiftui-synth --examples "{(width:390,height:844):{title:\"Info\",button:\"\"}}"
```

**Output:**

```swift
VStack {
    Text("Info")
        .font(.title)
        .padding()
    Spacer()
}
.padding()
```

#### 4. VStack with Image

```bash
swiftui-synth --examples "{(width:390,height:844):{Image:\"logo\"}}"
```

**Output:**

```swift
VStack {
    Image("logo")
    Spacer()
}
.padding()
```

#### 5. VStack with Image, Title, and Button

```bash
swiftui-synth --examples "{(width:390,height:844):{Image:\"profile\", title:\"User Details\", button:\"Edit\"}}"
```

**Output:**

```swift
VStack {
    Image("profile")
    Text("User Details")
        .font(.title)
        .padding()
    Spacer()
    Button("Edit") { }
        .padding()
}
.padding()
```

#### 6. Basic HStack via File and Save to Output File

*   Create `myhstack.txt`:
    ```
    {(width:390,height:100):HStack:{"Label A", "Spacer", "Label B"}}
    ```
*   Run the command:
    ```bash
    swiftui-synth --examples-file myhstack.txt --output MyHStackView.swift
    ```

**Output (Printed to console):**

```
Synthesized SwiftUI layout in X.XXms: // Actual time will vary
HStack {
    Text("Label A")
        .font(.title)
        .padding()
    Spacer()
    Text("Label B")
        .font(.title)
        .padding()
}
.padding()
Saved SwiftUI layout to MyHStackView.swift
```

**Content of `MyHStackView.swift`:**

```swift
HStack {
    Text("Label A")
        .font(.title)
        .padding()
    Spacer()
    Text("Label B")
        .font(.title)
        .padding()
}
.padding()
```

#### 7. Handling Escaped Quotes in Input

```bash
# Input contains escaped quotes: title:"Hello, \"World\"!"
swiftui-synth --examples "{(width:390,height:844):{title:\"Hello, \\\"World\\\"!\"}}"
```

**Output:**

```swift
VStack {
    Text("Hello, \"World\"!") // Note: Quotes inside the string are correctly rendered
        .font(.title)
        .padding()
    Spacer()
}
.padding()
```

## Contributing

Contributions are welcome! Please feel free to open an issue or submit a pull request on GitHub.

*   **Issues:** Report bugs, suggest features, or ask questions [here](https://github.com/yourusername/swiftui-synth/issues). <!-- Replace with your repo URL -->
*   **Pull Requests:** Fork the repository, make your changes, and open a PR [here](https://github.com/yourusername/swiftui-synth/pulls). <!-- Replace with your repo URL -->

Please ensure your code is formatted using `cargo fmt` and that all tests pass (`cargo test`) before submitting a PR.

## License

This project is licensed under the **MIT License**. See the LICENSE file for details (if one exists, otherwise state MIT).
```