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

use crate::highlighter::highlight;
use kasl::core::{ast::Range, error::ErrorRecord, localization::format_error};
use owo_colors::OwoColorize;

pub fn indicate_error(record: &ErrorRecord, file_path: &str, source: &str, preferred_lang: &str) {
    // Show the error message
    let localized_error = format_error(record, preferred_lang);
    println!(
        "* {} [{}]: {}",
        "Error".red().bold(),
        record.key.kind,
        localized_error.bold()
    );

    // Show all occurrences of the error in the code
    let mut sorted_ranges: Vec<Range> = record.ranges.iter().cloned().collect();
    sorted_ranges.sort();
    for range in sorted_ranges {
        indicate_source_loc(source, range, file_path);
    }
}

fn indicate_source_loc(source: &str, range: Range, file_path: &str) {
    // Get the start and end line/col positions
    let (start_line, start_col) = offset_to_line_col(source, range.start);
    let (end_line, end_col) = offset_to_line_col(source, range.end);

    // Get the max line number width
    let max_line_width = start_line.to_string().len().max(end_line.to_string().len());

    // Print the file path and start line/col
    println!(
        "{}{} {}:{}:{}",
        " ".repeat(max_line_width),
        "-->".bright_blue().bold(),
        file_path,
        start_line,
        start_col
    );

    // Get the lines between the start and end positions
    let lines: Vec<&str> = source.lines().collect();
    for line_number in start_line..=end_line {
        if let Some(line) = lines.get(line_number - 1) {
            if line_number == start_line && line_number == end_line {
                indicate_single_line(max_line_width, line_number, line, start_col, end_col);
            } else if line_number == start_line {
                indicate_single_line(max_line_width, line_number, line, start_col, line.len() + 1);
            } else if line_number == end_line {
                indicate_single_line(max_line_width, line_number, line, 1, end_col);
            } else {
                indicate_single_line(max_line_width, line_number, line, 1, line.len() + 1);
            }
        }
    }
}

fn indicate_single_line(
    line_num_width: usize,
    line_num: usize,
    line: &str,
    start_col: usize,
    end_col: usize,
) {
    let underline_len = (end_col - start_col).max(1);
    let underline = " ".repeat(start_col - 1) + &"^".repeat(underline_len);
    println!(
        "{:>width$} | {}\n{} | {}",
        line_num.blue().bold(),
        highlight(line),
        " ".repeat(line_num_width),
        underline.red(),
        width = line_num_width
    );
}

fn offset_to_line_col(source: &str, offset: usize) -> (usize, usize) {
    let clamped_offset = offset.min(source.len());
    let line = source[..clamped_offset]
        .chars()
        .filter(|&c| c == '\n')
        .count()
        + 1;
    let col = source[..clamped_offset]
        .rfind('\n')
        .map(|i| clamped_offset - i - 1)
        .unwrap_or(clamped_offset)
        + 1;
    (line, col)
}
