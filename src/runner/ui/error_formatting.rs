use kasl::{Range, error::ErrorRecord, localization::format_error};
use owo_colors::OwoColorize;

pub fn indicate_error(record: &ErrorRecord, file_path: &str, source: &str, preferred_lang: &str) {
    // Show all occurrences of the error in the code
    let mut sorted_ranges: Vec<Range> = record.ranges.iter().cloned().collect();
    sorted_ranges.sort();
    for range in sorted_ranges.iter() {
        println!("* {}:{}", file_path, format_offset(source, range.start));
        indicate_source_loc(source, *range);

        // Print a blank line between ranges
        println!();
    }

    // Show the error message
    let localized_error = format_error(record, preferred_lang);
    println!("{}", localized_error.bold());
}

fn indicate_source_loc(source: &str, range: Range) {
    // Get the start and end line/col positions
    let (start_line, start_col) = offset_to_line_col(source, range.start);
    let (end_line, end_col) = offset_to_line_col(source, range.end);

    // Get the max line number width
    let max_line_width = start_line.to_string().len().max(end_line.to_string().len());

    // Get the lines between the start and end positions
    let lines: Vec<&str> = source.lines().collect();
    for line_number in start_line..=end_line {
        if let Some(line) = lines.get(line_number - 1) {
            if line_number == start_line && line_number == end_line {
                indicate_single_line(max_line_width, line_number, line, start_col, end_col);
            } else if line_number == start_line {
                indicate_single_line(max_line_width, line_number, line, start_col, line.len());
            } else if line_number == end_line {
                indicate_single_line(max_line_width, line_number, line, 0, end_col);
            } else {
                indicate_single_line(max_line_width, line_number, line, 0, line.len());
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
        "{:>width$} | {}\n{} | {}\n",
        line_num.blue().bold(),
        line,
        " ".repeat(line_num_width),
        underline.red(),
        width = line_num_width
    );
}

fn format_offset(source: &str, offset: usize) -> String {
    let line_col = offset_to_line_col(source, offset);
    format!("{}:{}", line_col.0, line_col.1)
}

fn offset_to_line_col(source: &str, offset: usize) -> (usize, usize) {
    let line = source[..offset].chars().filter(|&c| c == '\n').count() + 1;
    let col = source[..offset]
        .rfind('\n')
        .map(|i| offset - i - 1)
        .unwrap_or(offset)
        + 1;
    (line, col)
}
