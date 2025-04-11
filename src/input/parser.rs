// File: src/input/parser.rs
use crate::ast::Value;

pub fn parse_examples(input: &str) -> Result<Vec<(Value, Value)>, String> {
    let trimmed = input.trim();
    if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
        return Err("Input must be enclosed in curly braces, e.g., {example}".to_string());
    }

    // Get content inside outer braces
    let inner = &trimmed[1..trimmed.len() - 1];
    if inner.is_empty() {
        return Err("Input must contain at least one example".to_string());
    }

    // --- Find the split point between dimensions and elements ---
    let mut depth = 0;
    let mut colon_pos = None;
    let chars: Vec<_> = inner.chars().collect(); // Collect characters for indexed access

    for (i, &ch) in chars.iter().enumerate() { // Iterate through character indices
        match ch {
            '(' => depth += 1,
            ')' => {
                if depth == 0 { // Cannot close parenthesis if not inside one
                     return Err("Mismatched parenthesis in dimensions".to_string());
                }
                depth -= 1;
                if depth == 0 {
                    // Found the closing ')' for dimensions. Now find the ':' after it, skipping whitespace.
                    let mut next_char_idx = i + 1;
                    while next_char_idx < chars.len() && chars[next_char_idx].is_whitespace() {
                        next_char_idx += 1;
                    }
                    // Check if the next non-whitespace char is indeed ':'
                    if next_char_idx < chars.len() && chars[next_char_idx] == ':' {
                        colon_pos = Some(next_char_idx); // Store the index of the colon
                        break; // Found our split point
                    } else {
                        // Found ')' but no ':' following it correctly
                        return Err("Expected ':' after dimensions '(...)', possibly missing or misplaced.".to_string());
                    }
                }
            }
             // Ignore ':' if inside parentheses
            ':' if depth > 0 => {}
            // If we hit a top-level ':' before closing parenthesis, format is wrong
            ':' if depth == 0 => return Err("Found ':' before dimensions '(..)' were closed or defined.".to_string()),
            _ => {} // Other characters
        }
         // Ensure we don't go below depth 0 outside the check for ')'
        if depth < 0 {
             return Err("Mismatched parenthesis in dimensions (extra closing parenthesis?)".to_string());
        }
    }
     // Check if parenthesis were left open
    if depth != 0 {
        return Err("Mismatched parenthesis in dimensions (not closed)".to_string());
    }


    // --- Parse Dimensions ---
    let colon_idx = colon_pos.ok_or("Could not find dimensions-elements separator '):{'")?;
    let dims_str = inner[..colon_idx].trim(); // Text before the colon
    let elements_str = inner[colon_idx + 1..].trim(); // Text after the colon

    if !dims_str.starts_with('(') || !dims_str.ends_with(')') {
         return Err("Dimensions part must be enclosed in parentheses, e.g., (width: W, height: H)".to_string());
    }
let dims_inner = dims_str.trim_start_matches('(').trim_end_matches(')').trim();
// *** FIX: Check for extra parentheses inside the dimensions block ***
let dims_content = &dims_str[1..dims_str.len()-1];
if dims_content.contains('(') || dims_content.contains(')') {
    return Err("Extra or mismatched parentheses within dimensions block.".to_string());
}
// *** End FIX ***
let mut width = None;
    let mut height = None;

    for part in dims_inner.split(',') {
        let part = part.trim();
        if part.is_empty() { continue; } // Allow trailing comma
        let mut kv = part.splitn(2,':'); // Use splitn to handle potential ':' in values if ever needed
        let key = kv.next().ok_or_else(|| format!("Missing dimension key in part: '{}'", part))?.trim();
        let value = kv.next().ok_or_else(|| format!("Missing dimension value for key '{}'", key))?.trim();

        match key {
            "width" => width = Some(value.parse::<i32>().map_err(|e| format!("Invalid width value '{}': {}", value, e))?),
            "height" => height = Some(value.parse::<i32>().map_err(|e| format!("Invalid height value '{}': {}", value, e))?),
            _ => return Err(format!("Unsupported dimension key: '{}'", key)),
        }
    }

    let width = width.ok_or("Missing width dimension")?;
    let height = height.ok_or("Missing height dimension")?;

    // --- Parse Elements ---
    let elements_str = elements_str.trim();

    // Handle HStack case specifically
    if elements_str.starts_with("HStack:") {
        let hstack_inner = elements_str["HStack:".len()..].trim();
        if !hstack_inner.starts_with('{') || !hstack_inner.ends_with('}') {
            return Err(format!("HStack elements must be enclosed in braces: '{}'", hstack_inner));
        }
        let hstack_children_str = &hstack_inner[1..hstack_inner.len() - 1];
        let mut hstack_children = Vec::new();
        // Simple comma split for HStack children for now
        for elem in hstack_children_str.split(',') {
            let elem = elem.trim();
             if elem.is_empty() { continue; }
             // Ensure HStack children are quoted strings
            if !elem.starts_with('"') || !elem.ends_with('"') {
                 return Err(format!("HStack child value must be quoted: {}", elem));
            }
            let value = elem[1..elem.len()-1].to_string(); // Remove quotes
            hstack_children.push((format!("child{}", hstack_children.len()), Value::String(value)));
        }
        let example = (
            Value::Dict(vec![
                ("width".to_string(), Value::Int(width)),
                ("height".to_string(), Value::Int(height)),
            ]),
            Value::Dict(vec![("HStack".to_string(), Value::Dict(hstack_children))]),
        );
        return Ok(vec![example]);
    }

    // Handle regular {key: "value", ...} case
    if !elements_str.starts_with('{') || !elements_str.ends_with('}') {
        return Err(format!("Elements must be enclosed in braces: '{}'", elements_str));
    }

    let elements_inner = &elements_str[1..elements_str.len() - 1].trim(); // Trim inner whitespace too
    let mut elements = Vec::new();

    // Robust comma splitting respecting quotes
    let mut current = String::new();
    let mut in_quotes = false;
    let mut escaped = false;

    for ch in elements_inner.chars() {
        match ch {
            '\\' if !escaped => escaped = true,
            '"' if !escaped => {
                in_quotes = !in_quotes;
                current.push(ch);
            }
            ',' if !in_quotes => {
                let elem = current.trim();
                if !elem.is_empty() {
                    parse_element(elem, &mut elements)?;
                }
                current.clear(); // Clear after processing
            }
            _ => {
                // Handle escaped backslash or quote properly
                if escaped && (ch == '\\' || ch == '"') {
                    current.push(ch); // Keep the escaped char
                } else if escaped {
                    current.push('\\'); // Keep the backslash if it didn't escape anything special
                    current.push(ch);
                } else {
                    current.push(ch);
                }
                escaped = false;
            }
        }
         // Ensure escaped status is reset if not followed by specific chars
        if escaped && ch != '\\' && ch != '"' {
             escaped = false;
        }
    }

    // Process the last element after the loop
    let elem = current.trim();
    if !elem.is_empty() {
        parse_element(elem, &mut elements)?;
    }

    let example = (
        Value::Dict(vec![
            ("width".to_string(), Value::Int(width)),
            ("height".to_string(), Value::Int(height)),
        ]),
        Value::Dict(elements),
    );

    Ok(vec![example])
}

// Helper to parse a single key:"value" element
fn parse_element(elem: &str, elements: &mut Vec<(String, Value)>) -> Result<(), String> {
    let mut kv = elem.splitn(2, ':');
    let key = kv.next()
        .ok_or_else(|| format!("Invalid element format (missing key?): '{}'", elem))?
        .trim();
    if key != "title" && key != "button" && key != "Image" {
        return Err(format!("Unsupported element key '{}': must be 'title', 'button', or 'Image'", key));
    }
    let value_str = kv.next()
        .ok_or_else(|| format!("Missing value for element key '{}'", key))?
        .trim();

    // Value must be enclosed in double quotes
    if !value_str.starts_with('"') || !value_str.ends_with('"') {
        return Err(format!("Value for key '{}' must be enclosed in double quotes: got '{}'", key, value_str));
    }

    // Remove quotes and handle escaped quotes within the value
    let inner_value = &value_str[1..value_str.len()-1];
    let mut final_value = String::with_capacity(inner_value.len());
    let mut chars = inner_value.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.peek() {
                Some(&'"') => {
                    final_value.push('"');
                    chars.next(); // Consume the quote
                }
                Some(&'\\') => {
                    final_value.push('\\');
                    chars.next(); // Consume the backslash
                }
                _ => final_value.push('\\'), // Keep backslash if it doesn't escape " or \
            }
        } else {
            final_value.push(ch);
        }
    }

    elements.push((key.to_string(), Value::String(final_value)));
    Ok(())
}


// --- Unit Tests --- (Keep existing tests, they should now pass with the fixed parser logic)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_full_example() {
        let input = "{(width:390,height:844):{title:\"Hello\",button:\"Click\"}}";
        let result = parse_examples(input).unwrap();
        assert_eq!(result.len(), 1);

        let (dims, elements) = &result[0];
        match dims {
            Value::Dict(d) => {
                assert_eq!(d.len(), 2);
                assert!(d.iter().any(|(k, v)| k == "width" && matches!(v, Value::Int(390))));
                assert!(d.iter().any(|(k, v)| k == "height" && matches!(v, Value::Int(844))));
            }
            _ => panic!("Expected Dict for dimensions"),
        }

        match elements {
            Value::Dict(e) => {
                assert_eq!(e.len(), 2);
                assert!(e.iter().any(|(k, v)| k == "title" && matches!(v, Value::String(s) if s == "Hello")));
                assert!(e.iter().any(|(k, v)| k == "button" && matches!(v, Value::String(s) if s == "Click")));
            }
            _ => panic!("Expected Dict for elements"),
        }
    }

    #[test]
    fn test_parse_valid_title_only() {
        let input = "{(width:390,height:844):{title:\"Welcome\"}}";
        let result = parse_examples(input).unwrap();
        assert_eq!(result.len(), 1);

        match &result[0].1 {
            Value::Dict(e) => {
                assert_eq!(e.len(), 1);
                assert!(e.iter().any(|(k, v)| k == "title" && matches!(v, Value::String(s) if s == "Welcome")));
            }
            _ => panic!("Expected Dict for elements"),
        }
    }

     #[test]
    fn test_parse_escaped_quotes_in_value() {
        let input = r#"{(width:390,height:844):{title:"Hello, \"World\"!", button:"\"OK\""}}"#;
        let result = parse_examples(input).unwrap();
        match &result[0].1 {
            Value::Dict(e) => {
                let title = e.iter().find(|(k,_)| k=="title").unwrap().1.clone();
                let button = e.iter().find(|(k,_)| k=="button").unwrap().1.clone();
                assert_eq!(title, Value::String("Hello, \"World\"!".to_string()));
                assert_eq!(button, Value::String("\"OK\"".to_string()));
            }
            _ => panic!("Expected Dict for elements"),
        }
    }

    #[test]
    fn test_missing_braces() {
        let input = "(width:390,height:844):{title:\"Hello\"}";
        assert!(parse_examples(input).is_err());
    }

    #[test]
    fn test_invalid_dimension_value() {
        let input = "{(width:abc,height:844):{title:\"Hello\"}}";
        let err = parse_examples(input).expect_err("Should fail");
        assert!(err.contains("Invalid width value"));
    }

     #[test]
    fn test_missing_dimension_key() {
        let input = "{(390,height:844):{title:\"Hello\"}}";
        assert!(parse_examples(input).is_err());
    }

    #[test]
    fn test_unsupported_key() {
        let input = "{(width:390,height:844):{TextField:\"placeholder\"}}";
        let err = parse_examples(input).expect_err("Should fail");
        assert!(err.contains("Unsupported element key 'TextField'"));
    }

    #[test]
    fn test_malformed_elements_missing_colon() {
        let input = "{(width:390,height:844):{title}}";
        let err = parse_examples(input).expect_err("Should fail");
        assert!(err.contains("Missing value for element key 'title'"));
    }

    #[test]
    fn test_missing_quotes_in_value() {
        let input = "{(width:390,height:844):{title:Hello}}";
         let err = parse_examples(input).expect_err("Should fail");
        assert!(err.contains("Value for key 'title' must be enclosed in double quotes"));
    }

    #[test]
    fn test_extra_whitespace() {
        // This test should now pass with the updated parser logic
        let input = "  {  ( width : 390 , height : 844 ) : { title : \"Hello\" , button : \"Click\" }  }  ";
        let result = parse_examples(input);
        assert!(result.is_ok(), "Parser failed with extra whitespace: {:?}", result.err());
        // Optionally, check the parsed values too
         let (dims, elements) = &result.unwrap()[0];
         match dims {
            Value::Dict(d) => {
                assert!(d.iter().any(|(k, v)| k == "width" && matches!(v, Value::Int(390))));
                assert!(d.iter().any(|(k, v)| k == "height" && matches!(v, Value::Int(844))));
            }
            _ => panic!("Expected Dict for dimensions"),
        }
         match elements {
            Value::Dict(e) => {
                assert!(e.iter().any(|(k, v)| k == "title" && matches!(v, Value::String(s) if s == "Hello")));
                assert!(e.iter().any(|(k, v)| k == "button" && matches!(v, Value::String(s) if s == "Click")));
            }
            _ => panic!("Expected Dict for elements"),
        }
    }

    #[test]
    fn test_parse_valid_hstack() {
        let input = "{(width:390,height:844):HStack:{\"A\",\"B\",\"Spacer\",\"C\"}}";
        let result = parse_examples(input).unwrap();
        assert_eq!(result.len(), 1);

        let (dims, elements) = &result[0];
        match dims {
            Value::Dict(d) => {
                assert_eq!(d.len(), 2);
                assert!(d.iter().any(|(k, v)| k == "width" && matches!(v, Value::Int(390))));
                assert!(d.iter().any(|(k, v)| k == "height" && matches!(v, Value::Int(844))));
            }
            _ => panic!("Expected Dict for dimensions"),
        }

        match elements {
            Value::Dict(e) => {
                assert_eq!(e.len(), 1);
                match e.iter().find(|(k,_)| k == "HStack") {
                    Some((_, Value::Dict(children))) => {
                         assert_eq!(children.len(), 4);
                         assert_eq!(children[0].1, Value::String("A".to_string()));
                         assert_eq!(children[1].1, Value::String("B".to_string()));
                         assert_eq!(children[2].1, Value::String("Spacer".to_string()));
                         assert_eq!(children[3].1, Value::String("C".to_string()));
                    }
                    _ => panic!("Expected HStack dict")
                }
            }
            _ => panic!("Expected Dict for elements"),
        }
    }

    #[test]
    fn test_parse_invalid_hstack_missing_braces() {
        let input = "{(width:390,height:844):HStack:\"A\",\"B\",\"Spacer\",\"C\"}";
        let result = parse_examples(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("HStack elements must be enclosed in braces"));
    }

     #[test]
    fn test_parse_invalid_hstack_missing_quotes() {
        let input = "{(width:390,height:844):HStack:{\"A\",B,\"Spacer\",\"C\"}}";
        let result = parse_examples(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("HStack child value must be quoted"));
    }


    #[test]
    fn test_parse_valid_image() {
        let input = "{(width:390,height:844):{Image:\"icon\"}}";
        let result = parse_examples(input).unwrap();
        assert_eq!(result.len(), 1);

        match &result[0].1 {
            Value::Dict(e) => {
                assert_eq!(e.len(), 1);
                assert!(e.iter().any(|(k, v)| k == "Image" && matches!(v, Value::String(s) if s == "icon")));
            }
            _ => panic!("Expected Dict for elements"),
        }
    }

    #[test]
    fn test_mismatched_parentheses() {
        let input1 = "{(width:390,height:844:{title:\"Hello\"}}"; // Missing closing )
        assert!(parse_examples(input1).is_err());

        let input2 = "{width:390,height:844):{title:\"Hello\"}}"; // Missing opening (
        assert!(parse_examples(input2).is_err());

        let input3 = "{((width:390,height:844)):{title:\"Hello\"}}"; // Extra opening (
        assert!(parse_examples(input3).is_err());

        let input4 = "{(width:390,height:844))):{title:\"Hello\"}}"; // Extra closing )
         assert!(parse_examples(input4).is_err());
    }

    #[test]
    fn test_malformed_separator() {
         let input1 = "{(width:390,height:844){title:\"Hello\"}}"; // Missing : separator
         assert!(parse_examples(input1).is_err());

         let input2 = "{(width:390,height:844) {title:\"Hello\"}}"; // Missing : separator (with space)
         assert!(parse_examples(input2).is_err());

        let input3 = "{(width:390,height:844);{title:\"Hello\"}}"; // Wrong separator ;
         assert!(parse_examples(input3).is_err());
    }

    #[test]
    fn test_empty_input_string() {
        assert!(parse_examples("").is_err());
        assert!(parse_examples("   ").is_err());
    }

     #[test]
    fn test_empty_braces() {
        assert!(parse_examples("{}").is_err());
    }

     #[test]
    fn test_empty_dimensions() {
        let input = "{():{title:\"Hello\"}}";
        assert!(parse_examples(input).is_err());
    }

     #[test]
    fn test_empty_elements() {
        let input = "{(width:100, height:100):{}}";
        let result = parse_examples(input).unwrap();
         match &result[0].1 {
            Value::Dict(e) => { assert!(e.is_empty()); }
            _ => panic!("Expected empty Dict for elements"),
         }
    }
}
