mod ast;
mod input;
mod synthesis;
mod output;
mod utils;

use clap::Parser;
use std::fs::{self, File};
use std::io::Write;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(name = "swiftui-synth", about = "Synthesizes SwiftUI layouts from examples")]
struct Cli {
    /// Examples in the format {(width:390,height:844):{title:"Hello",button:"Click"}}
    #[arg(long, group = "input")]
    examples: Option<String>,

    /// File containing the examples
    #[arg(long, group = "input")]
    examples_file: Option<String>,

    /// Optional output file to save the synthesized SwiftUI code
    #[arg(long)]
    output: Option<String>,
}

fn main() -> Result<(), String> {
    let args = Cli::parse();

    // Get examples from either the command line or a file
    let examples_str = match (args.examples, args.examples_file) {
        (Some(e), None) => e,
        (None, Some(f)) => fs::read_to_string(&f)
            .map_err(|e| format!("Failed to read examples file '{}': {}", f, e))?,
        _ => return Err("Please provide either --examples or --examples-file".to_string()),
    };

    // Parse examples
    let examples = input::parser::parse_examples(&examples_str)
        .map_err(|e| format!("Failed to parse examples: {}", e))?;

    // Synthesize layout
    let start = Instant::now();
    let ir = synthesis::swiftui::synthesize_layout(examples)
        .ok_or("No matching layout found for the given examples")?;
    let duration = start.elapsed();

    // Render SwiftUI code
    let swiftui_code = output::render::render_swiftui(&ir);

    // Output the result
    println!("Synthesized SwiftUI layout in {:.2?}:\n{}", duration, swiftui_code);

    // Save to file if --output is specified
    if let Some(output_path) = args.output {
        let mut file = File::create(&output_path)
            .map_err(|e| format!("Failed to create output file '{}': {}", output_path, e))?;
        file.write_all(swiftui_code.as_bytes())
            .map_err(|e| format!("Failed to write to output file '{}': {}", output_path, e))?;
        println!("Saved SwiftUI layout to {}", output_path);
    }

    Ok(())
}
