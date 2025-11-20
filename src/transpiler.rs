use anyhow::{Result, anyhow};

/// Transpile Paanini Sanskrit code to Rust code
pub fn transpile_to_rust(paanini_code: &str) -> Result<String> {
    let mut rust_code = String::new();
    
    // Add Rust boilerplate
    rust_code.push_str("fn main() {\n");
    
    let lines: Vec<&str> = paanini_code.lines().collect();
    let indent_level = 1;
    
    for line in lines {
        let trimmed = line.trim();
        
        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with("!!") {
            continue;
        }
        
        // Handle indentation
        let _current_indent = line.len() - line.trim_start().len();
        let rust_indent = "    ".repeat(indent_level);
        
        // Transpile line based on Sanskrit keywords
        let rust_line = transpile_line(trimmed)?;
        
        if !rust_line.is_empty() {
            rust_code.push_str(&rust_indent);
            rust_code.push_str(&rust_line);
            rust_code.push('\n');
        }
    }
    
    rust_code.push_str("}\n");
    
    Ok(rust_code)
}

fn transpile_line(line: &str) -> Result<String> {
    // दर्श() -> println!()
    if line.starts_with("दर्श(") || line.starts_with("darsh(") {
        let args = extract_function_args(line)?;
        return Ok(format!("println!({});", args));
    }
    
    // यदि -> if
    if line.starts_with("यदि ") || line.starts_with("yadi ") {
        let condition = line.split_whitespace().skip(1).collect::<Vec<_>>().join(" ");
        let condition = condition.trim_end_matches(':');
        return Ok(format!("if {} {{", condition));
    }
    
    // अन्यथा -> else
    if line == "अन्यथा:" || line == "anyatha:" {
        return Ok("} else {".to_string());
    }
    
    // यावत् -> while
    if line.starts_with("यावत् ") || line.starts_with("yavat ") {
        let condition = line.split_whitespace().skip(1).collect::<Vec<_>>().join(" ");
        let condition = condition.trim_end_matches(':');
        return Ok(format!("while {} {{", condition));
    }
    
    // कार्य -> fn (function definition)
    if line.starts_with("कार्य ") || line.starts_with("karya ") {
        let func_def = line.split_whitespace().skip(1).collect::<Vec<_>>().join(" ");
        let func_def = func_def.trim_end_matches(':');
        return Ok(format!("fn {} {{", func_def));
    }
    
    // Handle block endings (dedentation)
    if line.ends_with(":") {
        return Ok("".to_string()); // Already handled above
    }
    
    // Variable assignments and expressions
    if line.contains("=") && !line.contains("==") {
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() == 2 {
            let var_name = parts[0].trim();
            let value = parts[1].trim();
            return Ok(format!("let {} = {};", var_name, transpile_expression(value)?));
        }
    }
    
    // Function calls
    if line.contains("(") && line.contains(")") {
        return Ok(format!("{};", transpile_expression(line)?));
    }
    
    // Simple expressions
    Ok(format!("{};", transpile_expression(line)?))
}

fn extract_function_args(line: &str) -> Result<String> {
    if let Some(start) = line.find('(') {
        if let Some(end) = line.rfind(')') {
            let args = &line[start + 1..end];
            return Ok(args.to_string());
        }
    }
    Err(anyhow!("Invalid function call: {}", line))
}

fn transpile_expression(expr: &str) -> Result<String> {
    let mut result = expr.to_string();
    
    // Replace Sanskrit operators and keywords with Rust equivalents
    result = result.replace("दर्श(", "println!(");
    result = result.replace("darsh(", "println!(");
    
    // Replace Sanskrit variable names with transliterated versions
    result = result.replace("योग", "yog");
    result = result.replace("नाम", "naam");
    
    // Handle string literals in Sanskrit
    if result.contains("\"") {
        // Keep string literals as-is since Rust supports UTF-8
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_transpilation() {
        let paanini_code = r#"
!! Simple hello world
दर्श("नमस्ते विश्व")
        "#;
        
        let result = transpile_to_rust(paanini_code).unwrap();
        assert!(result.contains("println!(\"नमस्ते विश्व\");"));
    }
    
    #[test]
    fn test_variable_assignment() {
        let paanini_code = r#"
x = 5
दर्श(x)
        "#;
        
        let result = transpile_to_rust(paanini_code).unwrap();
        assert!(result.contains("let x = 5;"));
        assert!(result.contains("println!(x);"));
    }
}