// File: tests/integration.rs

// --- Imports ---
// These bring the necessary functions from your library crate (swiftui_synth)
// into the scope of this integration test crate.
use swiftui_synth::input::parser::parse_examples;
use swiftui_synth::synthesis::swiftui::synthesize_layout;
use swiftui_synth::output::render::render_swiftui;

// --- Helper Functions ---

// Helper to run the core logic (parse -> synthesize -> render) for a given input string.
// Returns the rendered SwiftUI code or an error string.
fn process_example(input: &str) -> Result<String, String> {
    let examples = parse_examples(input)?; // Propagate parsing errors
    let ir = synthesize_layout(examples)
        .ok_or_else(|| "Failed to synthesize layout".to_string())?; // Handle synthesis failure
    Ok(render_swiftui(&ir)) // Render the IR
}

// Helper to normalize whitespace for consistent string comparisons in tests.
// Removes trailing whitespace from each line.
fn normalize_whitespace(s: &str) -> String {
    s.lines()
        .map(|line| line.trim_end()) // Trim trailing whitespace
        .collect::<Vec<_>>()
        .join("\n") // Re-join lines with a single newline
}

// --- Test Cases ---

#[test]
fn test_end_to_end_full_example() {
    let input = "{(width:390,height:844):{title:\"Hello\",button:\"Click\"}}";
    let result = process_example(input).unwrap();

    // Define the expected output, normalized for consistent comparison
    let expected = normalize_whitespace(
        "VStack {
    Text(\"Hello\")
        .font(.title)
        .padding()
    Spacer()
    Button(\"Click\") { }
        .padding()
}
.padding()"
    );

    // Compare the normalized actual output with the normalized expected output
    assert_eq!(normalize_whitespace(&result), expected);
}

#[test]
fn test_end_to_end_title_only() {
    let input = "{(width:390,height:844):{title:\"Welcome\"}}";
    let result = process_example(input).unwrap();

    let expected = normalize_whitespace(
        "VStack {
    Text(\"Welcome\")
        .font(.title)
        .padding()
    Spacer()
}
.padding()"
    );

    assert_eq!(normalize_whitespace(&result), expected);
}

#[test]
fn test_end_to_end_error_propagation() {
    // Test invalid overall format
    assert!(process_example("not an example").is_err());

    // Test invalid dimensions format (missing height)
    assert!(process_example("{(width:390):{title:\"Hello\"}}").is_err());

    // Test invalid dimension value
    assert!(process_example("{(width:abc,height:844):{title:\"Hello\"}}").is_err());

    // Test unsupported element key
    assert!(process_example("{(width:390,height:844):{TextField:\"placeholder\"}}").is_err());
}

#[test]
fn test_end_to_end_whitespace_handling() {
    let input = "  {  ( width : 390 , height : 844 ) : { title : \"Hello\" , button : \"Click\" }  }  ";
    let result = process_example(input).unwrap(); // Should parse and render successfully

    // Basic checks to ensure it produced valid-looking output
    assert!(result.contains("VStack {"));
    assert!(result.contains("Text(\"Hello\")"));
    assert!(result.contains("Button(\"Click\")"));
    assert!(result.split_whitespace().count() > 10); // Check it's not empty/trivial
}

#[test]
fn test_end_to_end_special_characters() {
    // Test parsing and rendering of escaped quotes within strings
    let input = "{(width:390,height:844):{title:\"Hello, \\\"World\\\"!\"}}";
    let result = process_example(input).unwrap();

    // Ensure the output contains the correctly escaped string for SwiftUI
    assert!(result.contains("Text(\"Hello, \\\"World\\\"!\")"));
}

#[test]
fn test_end_to_end_empty_button() {
    // Test case where button value is an empty string (should omit the button)
    let input = "{(width:390,height:844):{title:\"Hello\",button:\"\"}}";
    let result = process_example(input).unwrap();

    // Verify the Button is not present in the output
    assert!(!result.contains("Button"));
    // Verify the Text element is still present
    assert!(result.contains("Text(\"Hello\")"));
}

#[test]
fn test_end_to_end_hstack() {
    let input = "{(width:390,height:844):HStack:{\"A\",\"B\",\"Spacer\",\"C\"}}";
    let result = process_example(input).unwrap();

    let expected = normalize_whitespace(
        "HStack {
    Text(\"A\")
        .font(.title)
        .padding()
    Text(\"B\")
        .font(.title)
        .padding()
    Spacer()
    Text(\"C\")
        .font(.title)
        .padding()
}
.padding()"
    );

    assert_eq!(normalize_whitespace(&result), expected);
}

#[test]
fn test_end_to_end_hstack_invalid_input() {
    // Test invalid format within HStack children (missing quotes around Spacer)
    let input = "{(width:390,height:844):HStack:{\"A\",\"B\",Spacer,\"C\"}}";
    let result = process_example(input);
    // Expect an error because 'Spacer' without quotes is not a valid string element
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("HStack child value must be quoted")); // Updated to match actual error message
}

#[test]
fn test_end_to_end_image() {
    let input = "{(width:390,height:844):{Image:\"icon\"}}";
    let result = process_example(input).unwrap();

    let expected = normalize_whitespace(
        "VStack {
    Image(\"icon\")
    Spacer()
}
.padding()"
    );

    assert_eq!(normalize_whitespace(&result), expected);
}

#[test]
fn test_end_to_end_image_and_title() {
    let input = "{(width:390,height:844):{Image:\"icon\", title:\"My Title\"}}";
    let result = process_example(input).unwrap();

    let expected = normalize_whitespace(
        "VStack {
    Image(\"icon\")
    Text(\"My Title\")
        .font(.title)
        .padding()
    Spacer()
}
.padding()"
    );

    assert_eq!(normalize_whitespace(&result), expected);
}

#[test]
fn test_end_to_end_image_title_button() {
    let input = "{(width:390,height:844):{Image:\"icon\", title:\"My Title\", button:\"Go\"}}";
    let result = process_example(input).unwrap();

    let expected = normalize_whitespace(
        "VStack {
    Image(\"icon\")
    Text(\"My Title\")
        .font(.title)
        .padding()
    Spacer()
    Button(\"Go\") { }
        .padding()
}
.padding()"
    );

    assert_eq!(normalize_whitespace(&result), expected);
}