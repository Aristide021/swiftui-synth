use crate::ast::{IR, Value};

/// Synthesizes a SwiftUI layout from examples.
/// Returns Some(IR) if a matching layout is found, or None otherwise.
pub fn synthesize_layout(examples: Vec<(Value, Value)>) -> Option<IR> {
    let (_dims, elements) = examples.get(0)?;

    // HStack support: look for a Dict with a "HStack" key
    if let Value::Dict(ref elems) = elements {
        if let Some((_, Value::Dict(children))) = elems.iter().find(|(k, _)| k == "HStack") {
            let mut ir_children = Vec::new();
            for (_k, v) in children {
                match v {
                    Value::String(s) => {
                        // Remove surrounding quotes if present
                        let s = s.trim_matches('"');
                        if s == "Spacer" {
                            ir_children.push(IR::Spacer);
                        } else {
                            ir_children.push(IR::Text(s.to_string()));
                        }
                    }
                    _ => {
                        eprintln!("Unsupported HStack child type: {:?}", _k);
                    }
                }
            }
            return Some(IR::HStack(ir_children));
        }
    }

    // Default: VStack logic
    let mut title = None;
    let mut button = None;
    let mut image = None; // Added Image support

    if let Value::Dict(ref elems) = elements {
        for (k, v) in elems {
            match (k.as_str(), v) {
                ("title", Value::String(s)) => title = Some(s.clone()),
                ("button", Value::String(s)) => button = Some(s.clone()),
                ("Image", Value::String(s)) => image = Some(s.clone()), // Added Image key
                _ => {}
            }
        }
    }

    let mut children = Vec::new();
    if let Some(img) = image {
        children.push(IR::Image(img));
    }
    if let Some(t) = title {
        children.push(IR::Text(t));
    }
    children.push(IR::Spacer);
    if let Some(b) = button {
        if !b.is_empty() {
            children.push(IR::Button(b));
        }
    }

    Some(IR::VStack(children))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_example(title: Option<&str>, button: Option<&str>, image: Option<&str>, hstack_children: Option<Vec<&str>>) -> Vec<(Value, Value)> {
        let mut elements = Vec::new();
        if let Some(t) = title {
            elements.push(("title".to_string(), Value::String(t.to_string())));
        }
        if let Some(b) = button {
            elements.push(("button".to_string(), Value::String(b.to_string())));
        }
        if let Some(img) = image {
            elements.push(("Image".to_string(), Value::String(img.to_string())));
        }
        if let Some(h) = hstack_children {
            let mut hstack_elements = Vec::new();
            for (i, child) in h.iter().enumerate() {
                hstack_elements.push((format!("child{}", i), Value::String(child.to_string())));
            }
            elements.push(("HStack".to_string(), Value::Dict(hstack_elements)));
        }

        vec![(
            Value::Dict(vec![
                ("width".to_string(), Value::Int(390)),
                ("height".to_string(), Value::Int(844)),
            ]),
            Value::Dict(elements),
        )]
    }

    #[test]
    fn test_synthesize_full_layout() {
        let examples = create_example(Some("Hello"), Some("Click"), None, None);
        let ir = synthesize_layout(examples).unwrap();
        
        match ir {
            IR::VStack(children) => {
                assert_eq!(children.len(), 3);
                assert!(matches!(&children[0], IR::Text(t) if t == "Hello"));
                assert!(matches!(&children[1], IR::Spacer));
                assert!(matches!(&children[2], IR::Button(b) if b == "Click"));
            }
            _ => panic!("Expected VStack"),
        }
    }

    #[test]
    fn test_synthesize_title_only() {
        let examples = create_example(Some("Welcome"), None, None, None);
        let ir = synthesize_layout(examples).unwrap();
        
        match ir {
            IR::VStack(children) => {
                assert_eq!(children.len(), 2);
                assert!(matches!(&children[0], IR::Text(t) if t == "Welcome"));
                assert!(matches!(&children[1], IR::Spacer));
            }
            _ => panic!("Expected VStack"),
        }
    }

    #[test]
    fn test_synthesize_empty_button() {
        let examples = create_example(Some("Title"), Some(""), None, None);
        let ir = synthesize_layout(examples).unwrap();
        
        match ir {
            IR::VStack(children) => {
                assert_eq!(children.len(), 2);
                assert!(matches!(&children[0], IR::Text(t) if t == "Title"));
                assert!(matches!(&children[1], IR::Spacer));
            }
            _ => panic!("Expected VStack"),
        }
    }

    #[test]
    fn test_synthesize_no_elements() {
        let examples = create_example(None, None, None, None);
        let ir = synthesize_layout(examples).unwrap();
        
        match ir {
            IR::VStack(children) => {
                assert_eq!(children.len(), 1);
                assert!(matches!(&children[0], IR::Spacer));
            }
            _ => panic!("Expected VStack"),
        }
    }

    #[test]
    fn test_synthesize_empty_examples() {
        let examples = Vec::new();
        assert!(synthesize_layout(examples).is_none());
    }

    #[test]
    fn test_synthesize_hstack() {
        let hstack_children = vec!["A", "B", "Spacer", "C"];
        let examples = create_example(None, None, None, Some(hstack_children));
        let ir = synthesize_layout(examples).unwrap();

        match ir {
            IR::HStack(children) => {
                assert_eq!(children.len(), 4);
                assert!(matches!(&children[0], IR::Text(t) if t == "A"));
                assert!(matches!(&children[1], IR::Text(t) if t == "B"));
                assert!(matches!(&children[2], IR::Spacer));
                assert!(matches!(&children[3], IR::Text(t) if t == "C"));
            }
            _ => panic!("Expected HStack"),
        }
    }

    #[test]
    fn test_synthesize_image() {
        let examples = create_example(None, None, Some("icon"), None);
        let ir = synthesize_layout(examples).unwrap();

        match ir {
            IR::VStack(children) => {
                assert_eq!(children.len(), 2);
                assert!(matches!(&children[0], IR::Image(name) if name == "icon"));
                assert!(matches!(&children[1], IR::Spacer));
            }
            _ => panic!("Expected VStack"),
        }
    }
}
