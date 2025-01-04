use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
#[wasm_bindgen]
pub fn run_computation(expression: &str) -> String {
    match evaluate(expression) {
        Ok(result) => String::from(&format!("Result: {}", result)),
        Err(err) => String::from(&format!("Error: {}", err)),
    }
}

fn evaluate(expression: &str) -> Result<f64, String> {
    let tokens = tokenize(expression)?;
    let (result, _) = parse_expression(&tokens, 0)?;
    Ok(result)
}

fn tokenize(expression: &str) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut num_buf = String::new();

    for c in expression.chars() {
        if c.is_whitespace() {
            continue;
        }
        if c.is_digit(10) || c == '.' {
            num_buf.push(c);
        } else if "+-*/()".contains(c) {
            if !num_buf.is_empty() {
                tokens.push(num_buf.clone());
                num_buf.clear();
            }
            tokens.push(c.to_string());
        } else {
            return Err(format!("Unexpected character: {}", c));
        }
    }

    if !num_buf.is_empty() {
        tokens.push(num_buf);
    }
    Ok(tokens)
}

fn parse_expression(tokens: &[String], pos: usize) -> Result<(f64, usize), String> {
    let (mut value, mut pos) = parse_term(tokens, pos)?;

    while pos < tokens.len() {
        let op = &tokens[pos];
        if op != "+" && op != "-" {
            break;
        }
        pos += 1;
        let (next_value, next_pos) = parse_term(tokens, pos)?;
        if op == "+" {
            value += next_value;
        } else {
            value -= next_value;
        }
        pos = next_pos;
    }

    Ok((value, pos))
}

fn parse_term(tokens: &[String], pos: usize) -> Result<(f64, usize), String> {
    let (mut value, mut pos) = parse_factor(tokens, pos)?;

    while pos < tokens.len() {
        let op = &tokens[pos];
        if op != "*" && op != "/" {
            break;
        }
        pos += 1;
        let (next_value, next_pos) = parse_factor(tokens, pos)?;
        if op == "*" {
            value *= next_value;
        } else {
            if next_value == 0.0 {
                return Err("Division by zero".to_string());
            }
            value /= next_value;
        }
        pos = next_pos;
    }

    Ok((value, pos))
}

fn parse_factor(tokens: &[String], mut pos: usize) -> Result<(f64, usize), String> {
    let token = &tokens[pos];
    if token == "(" {
        pos += 1;
        let (value, next_pos) = parse_expression(tokens, pos)?;
        if next_pos >= tokens.len() || tokens[next_pos] != ")" {
            return Err("Mismatched parentheses".to_string());
        }
        pos = next_pos + 1;
        Ok((value, pos))
    } else {
        token
            .parse::<f64>()
            .map(|n| (n, pos + 1))
            .map_err(|_| format!("Invalid number: {}", token))
    }
}
