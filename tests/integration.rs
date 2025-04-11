use swiftui_synth::input::parser::parse_examples;
use swiftui_synth::synthesis::swiftui::synthesize_layout;
use swiftui_synth::output::render::render_swiftui;

fn process_example(input: &str) -> Result<String, String> {
    let examples = parse_examples(input)?;
    let ir = synthesize_layout(examples)
        .ok_or_else(|| "Failed to synthesize layout".to_string())?;
    Ok(render_swiftui(&ir))
}

fn normalize_whitespace(s: &str) -> String {
    s.lines()
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn test_end_to_end_full_example() {
    let input = "{(width:390,height:844):{title:\"Hello\",button:\"Click\"}}";
    let result = process_example(input).unwrap();
    
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
    // Test invalid input format
    assert!(process_example("not an example").is_err());

    // Test missing required field
    assert!(process_example("{(width:390):{title:\"Hello\"}}").is_err());

    // Test invalid number
    assert!(process_example("{(width:abc,height:844):{title:\"Hello\"}}").is_err());

    // Test unsupported element
    assert!(process_example("{(width:390,height:844):{TextField:\"placeholder\"}}").is_err());
}

#[test]
fn test_end_to_end_whitespace_handling() {
    let input = "  {  ( width : 390 , height : 844 ) : { title : \"Hello\" , button : \"Click\" }  }  ";
    let result = process_example(input).unwrap();
    
    // Verify the output is properly formatted regardless of input spacing
    assert!(result.contains("VStack {"));
    assert!(result.contains("Text(\"Hello\")"));
    assert!(result.contains("Button(\"Click\")"));
    assert!(result.split_whitespace().count() > 10); // Should have reasonable amount of formatting
}

#[test]
fn test_end_to_end_special_characters() {
    let input = "{(width:390,height:844):{title:\"Hello, \\\"World\\\"!\"}}";
    let result = process_example(input).unwrap();
    
    // Verify special characters are properly escaped in the output
    assert!(result.contains("Text(\"Hello, \\\"World\\\"!\")"));
}

#[test]
fn test_end_to_end_empty_button() {
    let input = "{(width:390,height:844):{title:\"Hello\",button:\"\"}}";
    let result = process_example(input).unwrap();
    
    // Verify empty button is omitted
    assert!(!result.contains("Button"));
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
    let input = "{(width:390,height:844):HStack:{\"A\",\"B\",Spacer,\"C\"}}";
    let result = process_example(input);
    assert!(result.is_err());
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
