use owo_colors::OwoColorize;
use std::fmt::Display;

pub fn print_err_header() {
    eprintln!("{}", " ERROR ".on_red().bold());
}

pub fn print_err(e: impl Display) {
    print_err_header();
    eprintln!("{}", e);
}
