mod cli;
mod highlighter;
mod print_err;
mod runner;
mod std_installer;
mod subcommands;

use crate::{print_err::print_err, runner::run_target, std_installer::install_std};
use clap::Parser;
use cli::Cli;
use owo_colors::OwoColorize;
use rust_embed::RustEmbed;
use std::{env, path::Path};
use subcommands::Subcommands;

#[derive(RustEmbed)]
#[folder = "kasl_std/std"]
struct StdLib;

fn main() {
    let cli = Cli::parse();

    let preferred_lang = cli.lang.unwrap_or_else(|| {
        std::env::var("LANG")
            .unwrap_or_default()
            .split('_')
            .next()
            .unwrap_or("en")
            .to_string()
    });

    // Get the KASL_STD_PATH environment variable, or set a default path
    let default_std_path = match env::var("KASL_STD_PATH") {
        Ok(value) => value,
        Err(_) => {
            if let Some(home_path) = dirs::home_dir() {
                let default_path = home_path.join(".kasl").join("std");
                unsafe {
                    env::set_var("KASL_STD_PATH", &default_path);
                }
                default_path.to_string_lossy().into_owned()
            } else {
                print_err("Could not determine home directory for default KASL_STD_PATH");
                return;
            }
        }
    };

    match &cli.command {
        Subcommands::Install { std_path } => {
            let copy_path = Path::new(std_path.as_ref().unwrap_or(&default_std_path));
            install_std(copy_path).unwrap();
        }
        Subcommands::Run {
            target_path,
            iterations,
            input,
            no_spread,
        } => {
            if *iterations < 1 {
                print_err("Iterations must be greater than 0");
                return;
            }

            let target_path = Path::new(target_path);
            let std_path = Path::new(&default_std_path);
            run_target(
                target_path,
                std_path.to_path_buf(),
                *iterations,
                !*no_spread,
                input.as_ref(),
                preferred_lang,
            );
        }
        Subcommands::StdPath => {
            println!(
                "Path to {}: {}",
                "std".bright_green().bold(),
                default_std_path
            );
        }
    }
}
