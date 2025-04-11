// File: src/output/render.rs
use crate::ast::IR;

// Helper function to normalize whitespace for consistent string comparisons
// Removes trailing whitespace from each line and ensures single \n line endings.
fn normalize_whitespace_internal(s: &str) -> String {
    s.lines()
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn render_swiftui(ir: &IR) -> String {
    fn render(ir: &IR, indent: usize) -> String {
        let pad = " ".repeat(indent * 4);
        match ir {
            IR::VStack(children) => {
                let mut s = format!("{}VStack {{\n", pad);
                for child in children {
                    // Ensure Spacer and Image are not further indented inside VStack/HStack rendering
                    let child_indent = match child {
                        IR::Spacer | IR::Image(_) => indent + 1, // Keep same level as Text/Button inside Stack
                        _ => indent + 1,
                    };
                     // Add newline before Spacer if it's not the first element
                     if matches!(child, IR::Spacer) && !s.ends_with("{\n") && !s.ends_with("\n\n") {
                        // s.push('\n'); // Avoid double newlines if Spacer follows another element directly
                     }
                    s.push_str(&render(child, child_indent));
                }
                s.push_str(&format!("{}}}\n", pad));
                s.push_str(&format!("{}.padding()", pad)); // Add padding modifier to the Stack
                if indent == 0 { // Add final newline only for the top-level element
                    s.push('\n');
                }
                s
            }
            IR::HStack(children) => {
                let mut s = format!("{}HStack {{\n", pad);
                for child in children {
                     let child_indent = match child {
                        IR::Spacer | IR::Image(_) => indent + 1,
                        _ => indent + 1,
                    };
                     // Add newline before Spacer if needed
                    // if matches!(child, IR::Spacer) && !s.ends_with("{\n") && !s.ends_with("\n\n") {
                       // s.push('\n');
                    // }
                    s.push_str(&render(child, child_indent));
                }
                s.push_str(&format!("{}}}\n", pad));
                s.push_str(&format!("{}.padding()", pad)); // Add padding modifier to the Stack
                if indent == 0 { // Add final newline only for the top-level element
                     s.push('\n');
                }
                s
            }
            IR::Text(text) => format!(
                // Ensure modifiers are indented relative to the Text element
                "{}Text(\"{}\")\n{}    .font(.title)\n{}    .padding()\n",
                pad, text.replace("\"", "\\\""),
                pad, // Indentation for first modifier
                pad  // Indentation for second modifier
            ),
            IR::Button(label) => format!(
                 // Ensure modifiers are indented relative to the Button element
                "{}Button(\"{}\") {{ }}\n{}    .padding()\n",
                pad, label.replace("\"", "\\\""),
                pad // Indentation for modifier
            ),
            IR::Image(name) => format!(
                // Image usually doesn't have padding/font modifiers directly in this simple case
                "{}Image(\"{}\")\n",
                pad, name.replace("\"", "\\\"")
            ),
            IR::Spacer => format!("{}Spacer()\n", pad),
        }
    }
    // Normalize the final output to ensure consistent line endings and no trailing whitespace
    normalize_whitespace_internal(&render(ir, 0))
}

#[cfg(test)]
mod tests {
    use super::*;
    // Use the internal helper for tests too
    use super::normalize_whitespace_internal as normalize_whitespace;


    #[test]
    fn test_render_full_layout() {
        let ir = IR::VStack(vec![
            IR::Text("Hello".to_string()),
            IR::Spacer,
            IR::Button("Click".to_string()),
        ]);

        // Define expected output *without* extra newlines between elements
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

        assert_eq!(render_swiftui(&ir), expected); // render_swiftui now normalizes output
    }

    #[test]
    fn test_render_hstack() {
        let ir = IR::HStack(vec![
            IR::Text("A".to_string()),
            IR::Text("B".to_string()),
            IR::Spacer,
            IR::Text("C".to_string()),
        ]);

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

        assert_eq!(render_swiftui(&ir), expected);
    }

    #[test]
    fn test_render_image() {
        let ir = IR::Image("icon".to_string());
        // The expected output for a standalone Image (not in a stack)
        // should just be the Image line, normalized.
        let expected = normalize_whitespace("Image(\"icon\")");
        assert_eq!(render_swiftui(&ir), expected);
    }

    #[test]
    fn test_render_title_only() {
        let ir = IR::VStack(vec![
            IR::Text("Welcome".to_string()),
            IR::Spacer,
        ]);

        let expected = normalize_whitespace(
            "VStack {
    Text(\"Welcome\")
        .font(.title)
        .padding()
    Spacer()
}
.padding()"
        );

        assert_eq!(render_swiftui(&ir), expected);
    }

    #[test]
    fn test_render_special_characters() {
        let ir = IR::VStack(vec![
            IR::Text("Hello, \"World\"!".to_string()),
            IR::Spacer,
        ]);

        let rendered = render_swiftui(&ir);
        // Check the normalized output
        assert!(rendered.contains("Text(\"Hello, \\\"World\\\"!\")"));
    }

    #[test]
    fn test_render_consistent_indentation() {
        let ir = IR::VStack(vec![
                        IR::Text("Test".to_string()),
                        IR::HStack(vec![
                            IR::Button("Nested".to_string())
                        ])
                    ]);
        let rendered = render_swiftui(&ir);


        for line in rendered.lines() {
            if !line.trim().is_empty() { // Ignore empty lines
                let spaces = line.chars().take_while(|c| *c == ' ').count();
                assert!(spaces % 4 == 0, "Indentation should be a multiple of 4 spaces: '{}'", line);
            }
        }
    }

    #[test]
    fn test_render_empty_vstack() {
        let ir = IR::VStack(vec![]);
        let rendered = render_swiftui(&ir);
        let expected = normalize_whitespace(
            "VStack {
}
.padding()"
        );
        assert_eq!(rendered, expected);
    }

     #[test]
    fn test_render_image_in_vstack() {
        let ir = IR::VStack(vec![IR::Image("icon".to_string()), IR::Spacer]);
         let expected = normalize_whitespace(
            "VStack {
    Image(\"icon\")
    Spacer()
}
.padding()"
        );
        assert_eq!(render_swiftui(&ir), expected);
    }
}