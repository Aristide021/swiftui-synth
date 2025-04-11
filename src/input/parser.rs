use crate::ast::Value;

/// Parses examples from a string into a vector of (input, output) Value pairs.
/// Returns an error string if parsing fails.
pub fn parse_examples(input: &str) -> Result<Vec<(Value, Value)>, String> {
    let trimmed = input.trim();
    if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
        return Err("Input must be enclosed in curly braces, e.g., {example}".to_string());
    }

    // Remove outer braces
    let inner = &trimmed[1..trimmed.len() - 1];
    if inner.is_empty() {
        return Err("Input must contain at least one example".to_string());
    }

    // Find the dimensions-elements separator
    let mut depth = 0;
    let mut split_pos = None;
    let chars: Vec<_> = inner.chars().collect();
    
    for (i, &ch) in chars.iter().enumerate() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 && i + 1 < chars.len() && chars[i + 1] == ':' {
                    split_pos = Some(i);
                    break;
                }
            }
            _ => {}
        }
    }

    let split_pos = split_pos.ok_or("Could not find dimensions-elements separator")?;
    let dims_str = inner[..split_pos+1].trim(); // Include the closing parenthesis
    let elements_str = inner[split_pos + 2..].trim(); // Skip the colon

    // Parse dimensions (remove outer parentheses)
    let dims_inner = dims_str.trim_start_matches('(').trim_end_matches(')').trim();
    let mut width = None;
    let mut height = None;

    for part in dims_inner.split(',') {
        let part = part.trim();
        let mut kv = part.split(':');
        let key = kv.next().ok_or("Missing dimension key")?.trim();
        let value = kv.next().ok_or("Missing dimension value")?.trim();

        match key {
            "width" => width = Some(value.parse::<i32>().map_err(|_| "Invalid width value")?),
            "height" => height = Some(value.parse::<i32>().map_err(|_| "Invalid height value")?),
            _ => return Err(format!("Unsupported dimension key: {}", key)),
        }
    }

    let width = width.ok_or("Missing width")?;
    let height = height.ok_or("Missing height")?;

    // Parse elements
    let elements_str = elements_str.trim();
    if elements_str.starts_with("HStack:") {
        // Parse HStack as a special value
        let hstack_inner = elements_str["HStack:".len()..].trim();
        if !hstack_inner.starts_with('{') || !hstack_inner.ends_with('}') {
            return Err(format!("HStack elements must be enclosed in braces: '{}'", hstack_inner));
        }
        let hstack_children_str = &hstack_inner[1..hstack_inner.len() - 1];
        let mut hstack_children = Vec::new();
        for elem in hstack_children_str.split(',') {
            let elem = elem.trim();
            if !elem.is_empty() {
                hstack_children.push((format!("child{}", hstack_children.len()), Value::String(elem.to_string())));
            }
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

    if !elements_str.starts_with('{') || !elements_str.ends_with('}') {
        return Err(format!("Elements must be enclosed in braces: '{}'", elements_str));
    }

    let elements_inner = &elements_str[1..elements_str.len() - 1];
    let mut elements = Vec::new();

    // Split by commas, but respect quotes and ignore whitespace around commas
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
                    current.clear();
                }
            }
            _ => {
                if escaped && ch != '"' {
                    current.push('\\');
                }
                current.push(ch);
                escaped = false;
            }
        }
    }

    // Handle last element
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

fn parse_element(elem: &str, elements: &mut Vec<(String, Value)>) -> Result<(), String> {
    let mut kv = elem.splitn(2, ':');
    let key = kv.next()
        .ok_or_else(|| format!("Invalid element format: {}", elem))?
        .trim();
    if key != "title" && key != "button" && key != "Image" { // Added Image key
        return Err(format!("Unsupported element key '{}': must be 'title', 'button', or 'Image'", key));
    }
    let value = kv.next()
        .ok_or_else(|| format!("Missing value in element: {}", elem))?
        .trim();
    
    // Value must be quoted
    if !value.starts_with('"') || !value.ends_with('"') {
        return Err(format!("Value must be quoted: {}", elem));
    }
    
    let value = value[1..value.len()-1].to_string();
    elements.push((key.to_string(), Value::String(value)));
    Ok(())
}

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
    fn test_missing_braces() {
        let input = "(width:390,height:844):{title:\"Hello\"}";
        assert!(parse_examples(input).is_err());
    }

    #[test]
    fn test_invalid_dimension() {
        let input = "{(width:abc,height:844):{title:\"Hello\"}}";
        assert!(parse_examples(input).is_err());
    }

    #[test]
    fn test_unsupported_key() {
        let input = "{(width:390,height:844):{TextField:\"placeholder\"}}"; // Use TextField for unsupported key test
        assert!(parse_examples(input).is_err());
    }

    #[test]
    fn test_malformed_elements() {
        let input = "{(width:390,height:844):{title}}";
        assert!(parse_examples(input).is_err());
    }

    #[test]
    fn test_missing_quotes() {
        let input = "{(width:390,height:844):{title:Hello}}";
        assert!(parse_examples(input).is_err());
    }

    #[test]
    fn test_extra_whitespace() {
        let input = "  {  ( width : 390 , height : 844 ) : { title : \"Hello\" , button : \"Click\" }  }  ";
        let result = parse_examples(input);
        assert!(result.is_ok());
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
                assert!(e.iter().any(|(k, v)| k == "HStack" && matches!(v, Value::Dict(_))));
            }
            _ => panic!("Expected Dict for elements"),
        }
    }

    #[test]
    fn test_parse_invalid_hstack_missing_braces() {
        let input = "{(width:390,height:844):HStack:\"A\",\"B\",\"Spacer\",\"C\"}";
        let result = parse_examples(input);
        assert!(result.is_err());
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
}
