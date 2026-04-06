//
//  Copyright 2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

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
