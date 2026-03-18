use owo_colors::OwoColorize;
use std::fmt::Display;

pub fn print_err_header(optional_message: Option<&str>) {
    eprintln!(
        "{} {}",
        " ERROR ".on_red().bold(),
        optional_message.unwrap_or("")
    );
}

pub fn print_err(e: impl Display) {
    print_err_header(None);
    eprintln!("{}", e);
}
