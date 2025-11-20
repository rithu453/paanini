use std::collections::HashMap;

pub struct RunResult {
    pub output: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug)]
enum Value {
    Number(f64),
    Str(String),
    Bool(bool),
    List(Vec<Value>),
    Null,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Str(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", if *b { "सत्य" } else { "असत्य" }),
            Value::List(v) => {
                let s = v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ");
                write!(f, "[{}]", s)
            }
            Value::Null => write!(f, "null"),
        }
    }
}

#[derive(Clone)]
struct FunctionDef {
    params: Vec<String>,
    body: String,
}

#[derive(Clone, Default)]
pub struct Interpreter {
    vars: HashMap<String, Value>,
    functions: HashMap<String, FunctionDef>,
}

impl Interpreter {
    pub fn run(&mut self, src: &str) -> RunResult {
        let mut out = String::new();
        let mut errs = Vec::new();

        let norm = preprocess_indentation(src);
        let lines: Vec<String> = norm.lines().map(|l| l.to_string()).collect();
        let mut i = 0usize;
        while i < lines.len() {
            let line = lines[i].trim();
            if line.is_empty() || line.starts_with("!!") || line.starts_with('#') {
                i += 1;
                continue;
            }

            if line.starts_with("यदि") {
                match self.handle_if_else(&lines, i) {
                    Ok((consumed, block_out, block_errs)) => {
                        out.push_str(&block_out);
                        errs.extend(block_errs);
                        i += consumed;
                        continue;
                    }
                    Err(e) => {
                        errs.push(format!("Line {}: {}", i + 1, e));
                        i += 1;
                        continue;
                    }
                }
            }

            if line.starts_with("यावत्") {
                match self.handle_while(&lines, i) {
                    Ok((consumed, block_out, block_errs)) => {
                        out.push_str(&block_out);
                        errs.extend(block_errs);
                        i += consumed;
                        continue;
                    }
                    Err(e) => {
                        errs.push(format!("Line {}: {}", i + 1, e));
                        i += 1;
                        continue;
                    }
                }
            }

            if line.starts_with("परिभ्रमण") {
                match self.handle_for(&lines, i) {
                    Ok((consumed, block_out, block_errs)) => {
                        out.push_str(&block_out);
                        errs.extend(block_errs);
                        i += consumed;
                        continue;
                    }
                    Err(e) => {
                        errs.push(format!("Line {}: {}", i + 1, e));
                        i += 1;
                        continue;
                    }
                }
            }

            if line.starts_with("कार्य") {
                match self.handle_function_def(&lines, i) {
                    Ok(consumed) => {
                        i += consumed;
                        continue;
                    }
                    Err(e) => {
                        errs.push(format!("Line {}: {}", i + 1, e));
                        i += 1;
                        continue;
                    }
                }
            }

            match self.exec_line(line) {
                Ok(Some(s)) => {
                    out.push_str(&s);
                    if !s.ends_with('\n') {
                        out.push('\n');
                    }
                }
                Ok(None) => {}
                Err(e) => errs.push(format!("Line {}: {}", i + 1, e)),
            }
            i += 1;
        }
        RunResult { output: out, errors: errs }
    }

    fn exec_line(&mut self, line: &str) -> Result<Option<String>, String> {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("!!") || trimmed.starts_with('#') {
            return Ok(None);
        }

        // Assignment: name = expr (but not ==, >=, <=)
        if let Some(eq) = find_top_level_char(trimmed, '=') {
            let left_is_cmp = eq > 0 && trimmed.as_bytes().get(eq - 1) == Some(&b'=');
            let right_is_cmp = trimmed.as_bytes().get(eq + 1) == Some(&b'=');
            let ge = eq > 0 && trimmed.as_bytes().get(eq - 1) == Some(&b'>');
            let le = eq > 0 && trimmed.as_bytes().get(eq - 1) == Some(&b'<');
            if !(left_is_cmp || right_is_cmp || ge || le) {
                let left = trimmed[..eq].trim();
                let right = trimmed[eq + 1..].trim();
                if !is_valid_identifier(left) {
                    return Err("त्रुटिः: असाइनस्य नाम अवैधम्".into());
                }
                let val = self
                    .eval_expr(right)
                    .ok_or_else(|| format!("त्रुटिः: अभिव्यक्ति न संगृहीता -> {}", right))?;
                self.vars.insert(left.to_string(), val);
                return Ok(None);
            }
        }

        // Print: दर्श(expr)
        if trimmed.starts_with("दर्श") {
            let rest = trimmed.strip_prefix("दर्श").unwrap().trim_start();
            if !rest.starts_with('(') || !trimmed.ends_with(')') {
                return Err("त्रुटिः: दर्श प्रयोगः केवलं दर्श(expr) स्वरूपेण भवेत्".into());
            }
            let lp = trimmed.find('(').unwrap();
            let rp = trimmed.rfind(')').unwrap();
            let inner = &trimmed[lp + 1..rp];
            let val = self.eval_expr(inner).unwrap_or(Value::Null);
            return Ok(Some(format!("{}", val)));
        }

        // Function call as a statement: name(...)
        if let Some(lp) = trimmed.find('(') {
            if trimmed.ends_with(')') {
                let name = trimmed[..lp].trim();
                if is_valid_identifier(name) {
                    let args_str = &trimmed[lp + 1..trimmed.len() - 1];
                    let args = split_args(args_str)?;
                    let arg_vals: Vec<Value> = args
                        .into_iter()
                        .map(|a| self.eval_expr(a))
                        .collect::<Option<Vec<_>>>()
                        .ok_or_else(|| "त्रुटिः: तर्काः न संगृहीताः".to_string())?;
                    let _ = self.call_function(name, arg_vals)?; // ignore return
                    return Ok(None);
                }
            }
        }

        if trimmed == "help" {
            return Ok(Some(
                "Paanini आज्ञाः (Python-रूपेण):\n  x = 5\n  नाम = \"नमस्ते\"\n  दर्श(expr)\n  यदि x == 5:\n    दर्श(\"सत्यं\")\n  अन्यथा:\n    दर्श(\"असत्यं\")\n  यावत् x < 5:\n    दर्श(x)\n    x = x + 1\n  परिभ्रमण i in परिधि(5):\n    दर्श(i)\n  कार्य greet(नाम):\n    दर्श(\"नमस्ते \" + नाम)\n  greet(\"विश्व\")\n  !! टिप्पण्यः\n"
                    .to_string(),
            ));
        }

        Err(format!("अज्ञाता आज्ञा: {}", trimmed))
    }

    fn eval_expr(&self, expr: &str) -> Option<Value> {
        let s = expr.trim();
        if s.is_empty() {
            return Some(Value::Null);
        }
        // Parentheses unwrap
        if s.starts_with('(') && s.ends_with(')') {
            if let Some((start, end)) = outer_paren_bounds(s) {
                if start == 0 && end == s.len() - 1 {
                    return self.eval_expr(&s[1..s.len() - 1]);
                }
            }
        }
        // String literal
        if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
            return Some(Value::Str(s[1..s.len() - 1].to_string()));
        }
        // Boolean
        if s == "सत्य" {
            return Some(Value::Bool(true));
        }
        if s == "असत्य" {
            return Some(Value::Bool(false));
        }
        // Function call within expression
        if let Some(lp) = s.find('(') {
            if s.ends_with(')') {
                let name = s[..lp].trim();
                if is_valid_identifier(name) {
                    let args_str = &s[lp + 1..s.len() - 1];
                    let args = match split_args(args_str) { Ok(v) => v, Err(_) => return None };
                    let arg_vals: Vec<Value> = match args
                        .into_iter()
                        .map(|a| self.eval_expr(a))
                        .collect::<Option<Vec<_>>>() {
                        Some(v) => v,
                        None => return None,
                    };
                    return self.call_function(name, arg_vals).ok();
                }
            }
        }
        // Number
        if let Ok(n) = s.parse::<f64>() {
            return Some(Value::Number(n));
        }
        // Addition/concatenation at top level
        if let Some(idx) = find_top_level_plus(s) {
            let lv = self.eval_expr(&s[..idx])?;
            let rv = self.eval_expr(&s[idx + 1..])?;
            return match (lv, rv) {
                (Value::Number(a), Value::Number(b)) => Some(Value::Number(a + b)),
                (Value::Str(a), Value::Str(b)) => Some(Value::Str(format!("{}{}", a, b))),
                (Value::Str(a), v) => Some(Value::Str(format!("{}{}", a, v))),
                (v, Value::Str(b)) => Some(Value::Str(format!("{}{}", v, b))),
                _ => None,
            };
        }
        // Variable lookup
        if is_valid_identifier(s) {
            if let Some(v) = self.vars.get(s) {
                return Some(v.clone());
            }
        }
        None
    }

    fn eval_condition(&self, cond: &str) -> Result<bool, String> {
        let ops = ["==", "!=", ">=", "<=", ">", "<"];
        for op in ops.iter() {
            if let Some(p) = find_top_level_op(cond, op) {
                let left = cond[..p].trim();
                let right = cond[p + op.len()..].trim();
                let lv = self
                    .eval_expr(left)
                    .ok_or_else(|| "त्रुटिः: यदि शर्ता अपठिता".to_string())?;
                let rv = self
                    .eval_expr(right)
                    .ok_or_else(|| "त्रुटिः: यदि शर्ता अपठिता".to_string())?;
                return match (lv, rv, *op) {
                    (Value::Number(a), Value::Number(b), "==") => Ok(a == b),
                    (Value::Number(a), Value::Number(b), "!=") => Ok(a != b),
                    (Value::Number(a), Value::Number(b), ">") => Ok(a > b),
                    (Value::Number(a), Value::Number(b), "<") => Ok(a < b),
                    (Value::Number(a), Value::Number(b), ">=") => Ok(a >= b),
                    (Value::Number(a), Value::Number(b), "<=") => Ok(a <= b),
                    _ => Err("त्रुटिः: यदि शर्ते संख्यायाः तुलनाः एव समर्थिताः".into()),
                };
            }
        }
        Err("त्रुटिः: यदि शर्ता अवैध".into())
    }

    fn handle_if_else(
        &mut self,
        lines: &Vec<String>,
        start: usize,
    ) -> Result<(usize, String, Vec<String>), String> {
        let mut output = String::new();
        let mut errors = Vec::new();
        let line = lines[start].trim();
        let lp = line
            .find('(')
            .ok_or_else(|| "त्रुटिः: यदि शर्ता ( ) मध्ये भवेत्".to_string())?;
        let rp = line
            .rfind(')')
            .ok_or_else(|| "त्रुटिः: यदि शर्ता ( ) मध्ये भवेत्".to_string())?;
        let cond_str = &line[lp + 1..rp];
        let cond = self.eval_condition(cond_str)?;
        let (then_block, consumed_then) = collect_block(lines, start)?;
        let mut total = consumed_then;

        // search for else after then block
        let mut idx = start + consumed_then;
        while idx < lines.len() {
            let l = lines[idx].trim();
            if l.is_empty() || l.starts_with("!!") || l.starts_with('#') {
                idx += 1;
                continue;
            }
            if l.starts_with("अन्यथा") {
                let (else_block, consumed_else) = collect_block(lines, idx)?;
                total = (idx + consumed_else) - start;
                if cond {
                    let res = self.run(&then_block);
                    output.push_str(&res.output);
                    errors.extend(res.errors);
                } else {
                    let res = self.run(&else_block);
                    output.push_str(&res.output);
                    errors.extend(res.errors);
                }
                return Ok((total, output, errors));
            }
            break;
        }
        if cond {
            let res = self.run(&then_block);
            output.push_str(&res.output);
            errors.extend(res.errors);
        }
        Ok((total, output, errors))
    }

    fn handle_while(
        &mut self,
        lines: &Vec<String>,
        start: usize,
    ) -> Result<(usize, String, Vec<String>), String> {
        let mut output = String::new();
        let mut errors = Vec::new();
        let line = lines[start].trim();
        let lp = line
            .find('(')
            .ok_or_else(|| "त्रुटिः: यावत् शर्ता ( ) मध्ये भवेत्".to_string())?;
        let rp = line
            .rfind(')')
            .ok_or_else(|| "त्रुटिः: यावत् शर्ता ( ) मध्ये भवेत्".to_string())?;
        let cond_str = &line[lp + 1..rp];
        let (body, consumed) = collect_block(lines, start)?;
        let mut guard = 0usize;
        while guard < 10000 {
            guard += 1;
            if self.eval_condition(cond_str).unwrap_or(false) {
                let res = self.run(&body);
                output.push_str(&res.output);
                errors.extend(res.errors);
            } else {
                break;
            }
        }
        Ok((consumed, output, errors))
    }

    fn handle_for(
        &mut self,
        lines: &Vec<String>,
        start: usize,
    ) -> Result<(usize, String, Vec<String>), String> {
        let mut output = String::new();
        let mut errors = Vec::new();
        let line = lines[start].trim();
        // परिभ्रमण x in परिधि(n)
        let after_kw = line
            .strip_prefix("परिभ्रमण")
            .ok_or_else(|| "त्रुटिः: परिभ्रमण वाक्य अवैधम्".to_string())?
            .trim_start();
        let in_pos = after_kw
            .find(" in ")
            .ok_or_else(|| "त्रुटिः: परिभ्रमण स्वरूपः: परिभ्रमण x in परिधि(n)".to_string())?;
        let var = after_kw[..in_pos].trim();
        if !is_valid_identifier(var) {
            return Err("त्रुटिः: परिभ्रमण चरः अवैधः".into());
        }
        let iter_part = after_kw[in_pos + 4..].trim();
        let lp = iter_part
            .find('(')
            .ok_or_else(|| "त्रुटिः: परिभ्रमण परिधि( ) अपेक्षितम्".to_string())?;
        let rp = iter_part
            .rfind(')')
            .ok_or_else(|| "त्रुटिः: परिभ्रमण परिधि( ) अपेक्षितम्".to_string())?;
        let name = iter_part[..lp].trim();
        let arg = iter_part[lp + 1..rp].trim();
        if name != "परिधि" {
            return Err("त्रुटिः: परिभ्रमण केवलं परिधि(n) सह समर्थितम्".into());
        }
        let n = match self.eval_expr(arg) {
            Some(Value::Number(x)) => x as i64,
            _ => return Err("त्रुटिः: परिधि(n) मध्ये n संख्या भवेत्".into()),
        };
        let (body, consumed) = collect_block(lines, start)?;
        for i in 0..n {
            self.vars
                .insert(var.to_string(), Value::Number(i as f64));
            let res = self.run(&body);
            output.push_str(&res.output);
            errors.extend(res.errors);
        }
        Ok((consumed, output, errors))
    }

    fn handle_function_def(
        &mut self,
        lines: &Vec<String>,
        start: usize,
    ) -> Result<usize, String> {
        let line = lines[start].trim();
        // कार्य name(params)
        let rest = line
            .strip_prefix("कार्य")
            .ok_or_else(|| "त्रुटिः: कार्य स्वरूप अवैधः".to_string())?
            .trim_start();
        let lp = rest
            .find('(')
            .ok_or_else(|| "त्रुटिः: कार्य नामस्य अनन्तरं ( अपेक्षितम्".to_string())?;
        let rp = rest
            .rfind(')')
            .ok_or_else(|| "त्रुटिः: कार्य तर्काणां ')' न लब्धम्".to_string())?;
        let name = rest[..lp].trim();
        if !is_valid_identifier(name) {
            return Err("त्रुटिः: कार्य नाम अवैधम्".into());
        }
        let params_str = &rest[lp + 1..rp];
        let params = if params_str.trim().is_empty() {
            Vec::new()
        } else {
            params_str
                .split(',')
                .map(|s| s.trim().to_string())
                .collect()
        };
        for p in &params {
            if !is_valid_identifier(p) {
                return Err("त्रुटिः: कार्य तर्कस्य नाम अवैधम्".into());
            }
        }
        let (body, consumed) = collect_block(lines, start)?;
        self.functions
            .insert(name.to_string(), FunctionDef { params, body });
        Ok(consumed)
    }

    fn call_function(&self, name: &str, args: Vec<Value>) -> Result<Value, String> {
        // Builtins
        if name == "परिधि" {
            if args.len() != 1 {
                return Err("त्रुटिः: परिधि(n) एकः एव तर्कः".into());
            }
            let n = match args[0] {
                Value::Number(x) => x as i64,
                _ => return Err("त्रुटिः: परिधि(n) मध्ये n संख्या भवेत्".into()),
            };
            let list = (0..n).map(|i| Value::Number(i as f64)).collect::<Vec<_>>();
            return Ok(Value::List(list));
        }
        if name == "दर्श" {
            return Err("त्रुटिः: दर्श प्रयोगः केवलं दर्श(expr) स्वरूपेण भवेत्".into());
        }

        if let Some(def) = self.functions.get(name) {
            if def.params.len() != args.len() {
                return Err("त्रुटिः: कार्य तर्कसंख्या न समा".into());
            }
            let mut child = self.clone();
            for (p, v) in def.params.iter().zip(args.into_iter()) {
                child.vars.insert(p.clone(), v);
            }
            let res = child.run(&def.body);
            // No return yet
            let _ = res; // silence unused var in case
            return Ok(Value::Null);
        }
        Err(format!("त्रुटिः: अज्ञातः कार्यः: {}", name))
    }
}

fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    s.chars()
        .all(|c| c.is_alphanumeric() || c == '_' || (c as u32) > 127)
}

fn find_top_level_plus(s: &str) -> Option<usize> {
    find_top_level_char(s, '+')
}

fn find_top_level_char(s: &str, target: char) -> Option<usize> {
    let mut in_str = false;
    let mut depth = 0usize;
    for (i, c) in s.char_indices() {
        if c == '"' {
            in_str = !in_str;
            continue;
        }
        if in_str {
            continue;
        }
        if c == '(' {
            depth += 1;
        }
        if c == ')' && depth > 0 {
            depth -= 1;
        }
        if depth == 0 && c == target {
            return Some(i);
        }
    }
    None
}

fn find_top_level_op(s: &str, op: &str) -> Option<usize> {
    let mut in_str = false;
    let mut depth = 0usize;
    let bytes = s.as_bytes();
    let mut i = 0usize;
    while i < s.len() {
        let c = s[i..].chars().next().unwrap();
        let clen = c.len_utf8();
        if c == '"' {
            in_str = !in_str;
            i += clen;
            continue;
        }
        if in_str {
            i += clen;
            continue;
        }
        if c == '(' {
            depth += 1;
            i += clen;
            continue;
        }
        if c == ')' {
            if depth > 0 {
                depth -= 1;
            }
            i += clen;
            continue;
        }
        if depth == 0 {
            if i + op.len() <= bytes.len() && &s[i..i + op.len()] == op {
                return Some(i);
            }
        }
        i += clen;
    }
    None
}

fn split_args(s: &str) -> Result<Vec<&str>, String> {
    let mut res = Vec::new();
    let mut in_str = false;
    let mut depth = 0usize;
    let mut start = 0usize;
    for (i, c) in s.char_indices() {
        if c == '"' {
            in_str = !in_str;
            continue;
        }
        if in_str {
            continue;
        }
        if c == '(' {
            depth += 1;
        }
        if c == ')' {
            if depth > 0 {
                depth -= 1;
            }
        }
        if c == ',' && depth == 0 {
            res.push(s[start..i].trim());
            start = i + 1;
        }
    }
    if start <= s.len() {
        res.push(s[start..].trim());
    }
    Ok(res.into_iter().filter(|p| !p.is_empty()).collect())
}

fn outer_paren_bounds(s: &str) -> Option<(usize, usize)> {
    if !s.starts_with('(') || !s.ends_with(')') {
        return None;
    }
    let mut depth = 0usize;
    for (i, c) in s.char_indices() {
        if c == '(' {
            depth += 1;
        }
        if c == ')' {
            depth -= 1;
            if depth == 0 {
                return Some((0, i));
            }
        }
    }
    None
}

// Convert indentation-based blocks to synthetic braces lines so block extraction works
fn preprocess_indentation(src: &str) -> String {
    let mut out = String::new();
    let mut stack: Vec<usize> = vec![0];
    let mut prev_ended_colon = false;
    for orig in src.lines() {
        let raw = orig.replace('\t', "  ");
        let trimmed = raw.trim();
        if trimmed.is_empty() || trimmed.starts_with("!!") || trimmed.starts_with('#') {
            out.push_str(orig);
            out.push('\n');
            continue;
        }
        let indent = raw.chars().take_while(|c| *c == ' ').count();
        let curr = *stack.last().unwrap();
        if indent > curr {
            if prev_ended_colon {
                out.push_str("{\n");
                stack.push(indent);
            }
        } else if indent < curr {
            while indent < *stack.last().unwrap() {
                stack.pop();
                out.push_str("}\n");
            }
        }
        let mut line = trimmed.to_string();
        if line.ends_with(':') {
            line.pop();
            if line.starts_with("यदि") && !line.contains('(') {
                line = format!("यदि ({})", line.trim_start_matches("यदि").trim());
            }
            if line.starts_with("यावत्") && !line.contains('(') {
                line = format!("यावत् ({})", line.trim_start_matches("यावत्").trim());
            }
            if line.starts_with("अन्यथा") {
                line = "अन्यथा".to_string();
            }
            // परिभ्रमण / कार्य left as-is
            prev_ended_colon = true;
        } else {
            prev_ended_colon = false;
        }
        out.push_str(&line);
        out.push('\n');
    }
    while stack.len() > 1 {
        stack.pop();
        out.push_str("}\n");
    }
    out
}

fn collect_block(lines: &Vec<String>, start: usize) -> Result<(String, usize), String> {
    // Find a '{' at or after start
    let mut i = start;
    let mut found_open: Option<usize> = None;
    let mut first_after_open = String::new();
    while i < lines.len() {
        let l = lines[i].trim();
        if let Some(pos) = l.find('{') {
            found_open = Some(i);
            if let Some(close_pos) = l[pos + 1..].find('}') {
                let inner = l[pos + 1..pos + 1 + close_pos].trim();
                return Ok((inner.to_string(), (i + 1) - start));
            }
            first_after_open = l[pos + 1..].to_string();
            break;
        }
        i += 1;
    }
    let open_idx = found_open.ok_or_else(|| "त्रुटिः: अपेक्षितम् '{'".to_string())?;
    let mut block_lines: Vec<String> = Vec::new();
    if !first_after_open.trim().is_empty() {
        block_lines.push(first_after_open);
    }
    i = open_idx + 1;
    while i < lines.len() {
        let l = lines[i].trim();
        if l.contains('}') {
            let before = l.split('}').next().unwrap_or("").trim();
            if !before.is_empty() {
                block_lines.push(before.to_string());
            }
            return Ok((block_lines.join("\n"), (i + 1) - start));
        } else {
            block_lines.push(l.to_string());
        }
        i += 1;
    }
    Err("त्रुटिः: '}' न लब्धम्".into())
}
