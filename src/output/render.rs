use crate::ast::IR;

/// Renders the IR as SwiftUI code.
pub fn render_swiftui(ir: &IR) -> String {
    fn render(ir: &IR, indent: usize) -> String {
        let pad = " ".repeat(indent * 4);
        match ir {
            IR::VStack(children) => {
                let mut s = format!("{}VStack {{\n", pad);
                for child in children {
                    s.push_str(&render(child, indent + 1));
                }
                s.push_str(&format!("{}}}\n", pad));
                s.push_str(&format!("{}.padding()", pad));
                if indent == 0 {
                    s.push('\n');
                }
                s
            }
            IR::HStack(children) => {
                let mut s = format!("{}HStack {{\n", pad);
                for child in children {
                    s.push_str(&render(child, indent + 1));
                }
                s.push_str(&format!("{}}}\n", pad));
                s.push_str(&format!("{}.padding()", pad));
                if indent == 0 {
                    s.push('\n');
                }
                s
            }
            IR::Text(text) => format!(
                "{}Text(\"{}\"){}    .font(.title){}    .padding()\n",
                pad, text.replace("\"", "\\\""), "\n".to_string() + &pad, "\n".to_string() + &pad
            ),
            IR::Button(label) => format!(
                "{}Button(\"{}\") {{ }}{}    .padding()\n",
                pad, label.replace("\"", "\\\""), "\n".to_string() + &pad
            ),
            IR::Image(name) => format!(
                "{}Image(\"{}\")\n",
                pad, name.replace("\"", "\\\"")
            ),
            IR::Spacer => format!("{}Spacer()\n", pad),
        }
    }
    render(ir, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normalize_whitespace(s: &str) -> String {
        s.lines()
            .map(|line| line.trim_end())
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn test_render_full_layout() {
        let ir = IR::VStack(vec![
            IR::Text("Hello".to_string()),
            IR::Spacer,
            IR::Button("Click".to_string()),
        ]);

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

        assert_eq!(normalize_whitespace(&render_swiftui(&ir)), expected);
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

        assert_eq!(normalize_whitespace(&render_swiftui(&ir)), expected);
    }

    #[test]
    fn test_render_image() {
        let ir = IR::Image("icon".to_string());
        let expected = "Image(\"icon\")\n";
        assert_eq!(normalize_whitespace(&render_swiftui(&ir)), expected);
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

        assert_eq!(normalize_whitespace(&render_swiftui(&ir)), expected);
    }

    #[test]
    fn test_render_special_characters() {
        let ir = IR::VStack(vec![
            IR::Text("Hello, \"World\"!".to_string()),
            IR::Spacer,
        ]);

        let rendered = render_swiftui(&ir);
        assert!(rendered.contains("Text(\"Hello, \\\"World\\\"!\")"));
    }

    #[test]
    fn test_render_consistent_indentation() {
        let ir = IR::VStack(vec![IR::Text("Test".to_string())]);
        let rendered = render_swiftui(&ir);
        
        // Check that every line starts with 0 or 4 spaces (except empty lines)
        for line in rendered.lines() {
            if !line.is_empty() {
                let spaces = line.chars().take_while(|c| *c == ' ').count();
                assert!(spaces % 4 == 0, "Indentation should be a multiple of 4 spaces");
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
        assert_eq!(normalize_whitespace(&rendered), expected);
    }
}
