use owo_colors::OwoColorize;

pub fn highlight(line: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        if c == '/' && chars.get(i + 1) == Some(&'/') {
            let rest: String = chars[i..].iter().collect();
            result.push_str(&rest.bright_black().to_string());
            break;
        }

        if c.is_alphanumeric() || c == '_' {
            let j = chars[i..]
                .iter()
                .take_while(|&&c| c.is_alphanumeric() || c == '_')
                .count();
            let token: String = chars[i..i + j].iter().collect();

            let colored = match token.as_str() {
                // Keywords
                "func" | "let" | "var" | "if" | "else" | "return" | "import" | "struct"
                | "loop" | "input" | "output" | "state" | "static" | "infix" | "prefix"
                | "postfix" => token.magenta().to_string(),
                // Primtive type names
                "Float" | "Int" | "Bool" | "Void" => token.green().to_string(),
                // Boolean literals
                "true" | "false" => token.bright_yellow().to_string(),
                // Number literals
                _ if token.chars().next().is_some_and(|c| c.is_ascii_digit()) => {
                    token.bright_yellow().to_string()
                }
                // Identifier
                _ => token.bright_blue().to_string(),
            };

            result.push_str(&colored);
            i += j;
            continue;
        }

        result.push(c);
        i += 1;
    }

    result
}
