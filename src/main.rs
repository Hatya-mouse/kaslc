mod cli;
mod std_installer;
mod subcommands;

use clap::Parser;
use cli::Cli;
use rust_embed::RustEmbed;
use std::{env, path::Path};
use subcommands::Subcommands;

use crate::std_installer::install_std;

#[derive(RustEmbed)]
#[folder = "kasl_std/std"]
struct StdLib;

fn main() {
    let cli = Cli::parse();

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
                panic!("Could not determine home directory for default KASL_STD_PATH");
            }
        }
    };

    match &cli.command {
        Subcommands::Install { std_path } => {
            let copy_path = Path::new(std_path.as_ref().unwrap_or(&default_std_path));
            install_std(copy_path).unwrap();
        }
    }
}
